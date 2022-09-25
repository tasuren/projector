//! projector by tasuren

use eframe::{
    CreationContext, App, Frame, Storage, egui::{
        Context, Window, CentralPanel, Layout, Button,
        FontDefinitions, FontData, FontFamily, Id,
        Pos2, Color32, Stroke, Align, TopBottomPanel,
        Rect, Style, ScrollArea, global_dark_light_mode_switch
    }, get_value, set_value
};

use serde::{ Deserialize, Serialize };
use serde_json::{ from_str, to_string };

#[cfg(not(target_arch="wasm32"))]
use {
    std::{ io::Write, fs::{ File, read_to_string } },
    rfd::FileDialog
};

#[cfg(target_arch="wasm32")]
use {
    rfd::AsyncFileDialog, async_std::task::block_on,
    gloo_utils::document, gloo_file::{ Blob, ObjectUrl }, eframe::{
        wasm_bindgen::JsCast, web_sys::{ Element, HtmlElement }
    }
};

use super::APPLICATION_NAME;


// デフォルトのRectを作ります。値が意味をなしていないRectが返されます。
fn make_default_rect() -> Rect {
    Rect::from_min_max(Pos2::default(), Pos2::new(200., 130.))
}


/// カード情報を格納するための構造体です。
#[derive(Clone, Deserialize, Serialize)]
struct Card {
    title: String,
    description: String,
    parent: isize,
    rect: Rect,
    id: isize,
    collapsed: bool,
    #[serde(skip)]
    editing: bool
}

impl Card {
    /// これのインスタンスを新しく作ります。
    fn new(id: isize) -> Self {
        Self {
            title: String::new(), description: String::new(), parent: id,
            rect: make_default_rect(), id: id, collapsed: false, editing: false
        }
    }
}


/// GUIアプリの構造体です。
#[derive(Default, Deserialize, Serialize)]
pub struct Application {
    #[cfg(not(target_arch="wasm32"))]
    changed_window_size: bool,
    #[serde(skip)]
    maybe_card_title: String,
    #[serde(skip)]
    loaded_just_now: bool,
    #[serde(skip)]
    #[cfg(target_arch="wasm32")]
    selecting_file: bool,
    data: Vec<Card>
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
        setup(&cc.egui_ctx);
        cc.egui_ctx.set_pixels_per_point(1.5);

        if let Some(storage) = cc.storage {
            get_value(storage, APPLICATION_NAME).unwrap_or_default()
        } else { Default::default() }
    }

    /// 設定を読み込む。
    fn load(&mut self, ctx: &Context, raw: String) {
        ctx.memory().reset_areas();
        self.data = from_str(&raw).expect("ファイルが壊れていて開けませんでした。");
        self.loaded_just_now = true;
    }
}

/// projector用のファイルダイアログのインスタンスを作ります。
#[cfg(target_arch="wasm32")]
fn make_file_dialog() -> AsyncFileDialog {
    AsyncFileDialog::new().add_filter("プロジェクト", &["ptd"])
}
#[cfg(not(target_arch="wasm32"))]
fn make_file_dialog() -> FileDialog {
    FileDialog::new().add_filter("プロジェクト", &["ptd"])
}

/// ファイル読み込み時のデータを格納するためのHTMLタグのElementを取得します。
#[cfg(target_arch="wasm32")]
fn get_file_stack_element() -> Element {
    document().get_element_by_id("stack").unwrap()
}

/// ファイルダイアログを表示して、ファイルが選択されたら、HTMLにデータを格納します。
/// ウェブ版でのみ使われます。
#[cfg(target_arch="wasm32")]
async fn create_file_dialog() -> Option<()> {
    if let Some(file_handle) = make_file_dialog().pick_file().await {
        get_file_stack_element().set_attribute("data-stack", &unsafe {
            String::from_utf8_unchecked(file_handle.read().await)
        }).unwrap();
        Some(())
    } else { None }
}

impl App for Application {
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APPLICATION_NAME, self);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // 初期状態のウィンドウのサイズを設定する。
        #[cfg(not(target_arch="wasm32"))]
        if !self.changed_window_size {
            _frame.set_window_size((1500., 900.).into());
            self.changed_window_size = true;
        };

        // メニューを作る。
        TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // カードの管理を行うためのメニューの中身を作る。
                ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
                    // ダークモードとライトモードの切り替えボタンを付ける。
                    global_dark_light_mode_switch(ui);
                    ui.separator();

                    // 読み込みを付ける。
                    let mut button = Button::new("📂").fill(Color32::TRANSPARENT);
                    if ui.add(button).clicked() {
                        #[cfg(target_arch="wasm32")]
                        {
                            block_on(create_file_dialog());
                            self.selecting_file = true;
                        }
                        #[cfg(not(target_arch="wasm32"))]
                        {
                            let tentative = make_file_dialog().pick_file();

                            if let Some(file_handle_or_path) = tentative {
                                let raw = read_to_string(file_handle_or_path)
                                    .expect("ファイルを開けませんでした。");
                                self.load(ctx, raw)
                            };
                        }
                    };
                    #[cfg(target_arch="wasm32")]
                    if self.selecting_file {
                        // ウェブ版では、rdfのファイルダイアログから、データを受け取ることが困難となっていた。
                        // そのため、気分は悪いが、やむを得ずHTMLを介してデータをやり取りすることにした。
                        // 詳細は`/index.html`の下の方に記載した。
                        let element = get_file_stack_element();
                        if let Some(raw) = element.get_attribute("data-stack") {
                            self.load(ctx, raw);
                            self.selecting_file = false;
                            element.remove_attribute("data-stack").unwrap();
                        };
                    };

                    // 保存ボタンを付ける。
                    button = Button::new("💾").fill(Color32::TRANSPARENT);
                    if ui.add(button).clicked() {
                        let data = to_string(&self.data).unwrap();
                        #[cfg(target_arch="wasm32")]
                        {
                            // ウェブ版では、ダウンロードリンクを作って、ブラウザにダウンロードを行わせる。
                            let element = document().get_element_by_id("download").unwrap();
                            let url = ObjectUrl::from(
                                Blob::new_with_options(
                                    data.as_bytes(), Some("octet/stream")
                                )
                            );
                            element.set_attribute("href", &url).unwrap();
                            element.dyn_ref::<HtmlElement>().unwrap().click();
                        }
                        #[cfg(not(target_arch="wasm32"))]
                        if let Some(path) = make_file_dialog().save_file() {
                            if let Ok(mut f) = File::create(path
                            ) {
                                f.write_all(data.as_bytes())
                                    .expect("ファイルの読み込みに失敗しました。");
                            };
                        };
                    };
                    ui.separator();

                    // タイトル入力欄と、新しいカード追加ボタンを付ける。
                    ui.text_edit_singleline(&mut self.maybe_card_title);
                    if ui.button("追加").clicked() {
                        // まだ使われていないIDを検索する。
                        let mut id = 0;
                        let mut changed = false;
                        self.data.sort_by_key(|c| c.id);
                        for card in self.data.iter() {
                            if card.id - id  > 1 { id += 1; changed = true; break; };
                            id = card.id;
                        };
                        if !changed { id += 1; }
                        // 新しくカードを作る。
                        let mut card = Card::new(id);
                        card.title = self.maybe_card_title.clone();
                        self.maybe_card_title = String::new();
                        self.data.push(card);
                    };
                });

                // 著作権表記を作る。
                ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                    ui.hyperlink_to("© 2022 tasuren", "https://tasuren.xyz");
                    ui.separator();
                    ui.hyperlink_to(" ❔ このツールについて", "https://projector.tasuren.xyz/information.html");
                    /*
                    #[cfg(target_arch="wasm32")]
                    ui.hyperlink_to("📥 オフライン版", "https://github.com/tasuren/projector/releases");
                    */
                });
            });
        });

        let length = self.data.len();

        // カードのウィンドウを全て追加する。
        let mut window_statuses = vec![true;length];
        let mut lines = Vec::new();

        for (i, window_status) in window_statuses.iter_mut().enumerate() {
            // ウィンドウを作る。
            let window = Window::new(&self.data[i].title).vscroll(true)
                .id(Id::new(format!("{}", self.data[i].id)))
                .open(window_status).min_width(155.).min_height(110.);
            self.data[i].rect = if self.loaded_just_now {
                window.fixed_rect(self.data[i].rect)
            } else {
                window.default_rect(self.data[i].rect)
            } .show(ctx, |ui| {
                // ウィンドウの中身を組み立てる。
                TopBottomPanel::bottom(format!("control_{}", i))
                    .show_inside(ui, |ui| {
                        ui.columns(2, |columns| {
                            // 他のカードに繋げるメニューボタンを付ける。
                            columns[0].menu_button("他に繋ぐ", |ui| {
                                for (maybe_parent_index, card) in self.data
                                        .iter().enumerate() {
                                    if ui.button(
                                        if maybe_parent_index == i { "繋がない" }
                                        else { &card.title }
                                    ).clicked() {
                                        self.data[i].parent = card.id;
                                        break;
                                    };
                                };
                            });
                            // 編集ボタンを付ける。
                            if columns[1].button(
                                if self.data[i].editing { "完了" } else { "編集" }
                            ).clicked() {
                                self.data[i].editing = !self.data[i].editing;
                            };
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
        if self.loaded_just_now { self.loaded_just_now = false; };

        // 消されたカードをデータを消す。
        for (i, window_status) in window_statuses.iter().enumerate() {
            if !window_status { self.data.remove(i); };
        };

        // 中央のパネルを作り、ウィンドウとウィンドウの間に線を引く。
        CentralPanel::default().show(ctx, |ui| {
            if !self.data.is_empty() {
                let painter = ui.painter();
                for line in lines {
                    for card in self.data.iter() {
                        if card.id == line.1 {
                            painter.line_segment(
                                [line.0, card.rect.center()],
                                Stroke::new(5., Color32::DARK_GRAY)
                            );
                            break;
                        };
                    };
                };
            };
        });
    }
}