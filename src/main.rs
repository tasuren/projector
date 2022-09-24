//! projector - Program for Native

// Windowsでの本番時は、ターミナルを表示しないようにする。
#![cfg_attr(not(debug_assertions), windows_subsystem="windows")]

mod app;
pub use app::Application;


pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(APPLICATION_NAME, native_options, Box::new(
        |cc| Box::new(Application::new(cc))
    ));
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(eframe_template::Application::new(cc))),
    ).expect("failed to start eframe");
}