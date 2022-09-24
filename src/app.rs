//! projector by tasuren

use eframe::{
    CreationContext, App, Frame, Storage, egui::{
        Context, Window, CentralPanel, Layout,
        FontDefinitions, FontData, FontFamily, Id
    }, get_value, set_value
};

use egui::{Pos2, Color32, Stroke, Align, TopBottomPanel, Visuals, Rect, Style,  ScrollArea};
use serde::{ Deserialize, Serialize };

use super::{ APPLICATION_NAME, VERSION };


// デフォルトのRectを作ります。値が意味をなしていないRectが返されます。
fn make_default_rect() -> Rect {
    Rect::from_min_max(Pos2::default(), Pos2::default())
}


/// アイテム情報を格納するための構造体です。
#[derive(Clone, Deserialize, Serialize)]
struct Item {
    title: String,
    description: String,
    parent: usize,
    rect: Rect,
    #[serde(skip)]
    editing: bool
}

impl Item {
    /// これのインスタンスを新しく作ります。
    fn new(parent: usize) -> Self {
        Self {
            title: String::new(), description: String::new(),
            parent: parent, rect: make_default_rect(), editing: false
        }
    }
}


/// GUIアプリを表す構造体です。
#[derive(Default, Deserialize, Serialize)]
pub struct Application {
    #[serde(skip)]
    changed_window_size: bool,
    #[serde(skip)]
    maybe_item_title: String,
    is_light_mode: bool,
    data: Vec<Item>
}

/// フォントを設定します。
fn setup(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("ZenMaruGothic".to_owned(), FontData::from_static(
        include_bytes!("../assets/ZenMaruGothic-Regular.ttf")
    ));
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "ZenMaruGothic".to_owned());
    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .push("ZenMaruGothic".to_owned());
    ctx.set_fonts(fonts);
    let mut style = Style::default();
    for (_, font) in style.text_styles.iter_mut() {
        font.size = 20.;
    }
    ctx.set_style(style);
}

impl Application {
    /// 新しくアプリのインスタンスを作ります。
    pub fn new(cc: &CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        setup(&cc.egui_ctx);

        if let Some(storage) = cc.storage {
            return get_value(storage, APPLICATION_NAME).unwrap_or_default();
        };

        Default::default()
    }
}

impl App for Application {
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APPLICATION_NAME, self);
    }

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // 初期状態のウィンドウのサイズを設定する。
        if !self.changed_window_size {
            frame.set_window_size((1500., 900.).into());
            self.changed_window_size = true;
        };

        // もしまだ何もアイテムがないのなら、新しく作る。
        if self.data.is_empty() {
            self.data.push(Item::new(0));
            self.data[0].title = "Origin".to_string();
        };

        // メニューを作る。
        TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 新しくアイテムを追加するためのフォームを作る。
                ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                    // ダークモードとライトモードの切り替えボタンを付ける。
                    if ui.button(if self.is_light_mode { "☀️" } else { "🌙" }).clicked() {
                        ctx.set_visuals(if self.is_light_mode {
                            Visuals::dark() } else { Visuals::light()
                        });
                        self.is_light_mode = !self.is_light_mode;
                    };
                    ui.separator();

                    // タイトル入力欄と、新しいアイテム追加ボタンを付ける。
                    ui.text_edit_singleline(&mut self.maybe_item_title);
                    if ui.button("アイテムを追加する").clicked() {
                        let mut item = Item::new(self.data.len());
                        item.title = self.maybe_item_title.clone();
                        self.maybe_item_title = String::new();
                        self.data.push(item);
                    };
                    ui.separator();
                });

                // 著作権表記を作る。
                ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                    ui.hyperlink_to("(c) 2022 tasuren", "https://tasuren.xyz");
                });
            });
        });

        // メモのアイテムを追加する。
        let mut window_statuses = vec![true;self.data.len()];
        let mut lines = Vec::new();
        let mut window;
        for (i, window_status) in window_statuses.iter_mut().enumerate() {
            window = Window::new(&self.data[i].title).id(Id::new(format!("{}", i)))
                .open(window_status).min_width(150.).min_height(75.)
                .default_width(150.).default_height(75.)
                .default_rect(self.data[i].rect).vscroll(true);
            self.data[i].rect = window.show(ctx, |ui| {
                TopBottomPanel::bottom(format!("control_{}", i)).show_inside(ui, |ui| {
                    ui.columns(2, |columns| {
                        // 他のアイテムに繋げるメニューボタンを付ける。
                        columns[0].menu_button("他に繋ぐ", |ui| {
                            for (maybe_parent_index, item) in self.data.iter().enumerate() {
                                if ui.button(&item.title).clicked() {
                                    self.data[i].parent = maybe_parent_index;
                                    break;
                                };
                            };
                        });
                        // 編集ボタンを付ける。
                        if columns[1].button(if self.data[i].editing { "完了" } else { "編集" })
                            .clicked() { self.data[i].editing = !self.data[i].editing; };
                    });
                });

                // 内容を編集するテキストボックスを付ける。
                CentralPanel::default().show_inside(ui, |ui|
                    ScrollArea::vertical().show(ui, |ui| if self.data[i].editing {
                        ui.text_edit_singleline(&mut self.data[i].title);
                        ui.text_edit_multiline(&mut self.data[i].description);
                    } else { ui.label(&self.data[i].description); })
                );
            }).unwrap().response.rect;

            // 後で線で繋ぐのに使うためのウィンドウの座標をキャッシュしておく。
            lines.push((self.data[i].rect.center(), self.data[i].parent));
        };

        // 消されたアイテムをデータを消す。
        for (i, window_status) in window_statuses.iter().enumerate() {
            if !window_status {
                self.data.remove(i);
                let length = self.data.len();
                for tentative_index in (0..length).map(|i| length - 1 - i) {
                    if self.data[tentative_index].parent == i {
                        self.data.remove(tentative_index);
                    };
                };
                break;
            };
        };

        // 中央のパネルを作り、ウィンドウとウィンドウの間に線を引く。
        CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            for line in lines {
                if let Some(other) = self.data.get(line.1) {
                    painter.line_segment(
                        [line.0, other.rect.center()],
                        Stroke::new(5., if self.is_light_mode {
                            Color32::GRAY } else { Color32::DARK_GRAY
                        })
                    );
                } else { break; };
            };
        });
    }
}