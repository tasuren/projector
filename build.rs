//! projector - Build

#[cfg(target_os="windows")]
use winres::WindowsResource;


#[cfg(target_os="windows")]
fn main() {
    let mut res = WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.compile().unwrap();
}

#[cfg(target_os="macos")]
fn main() {}