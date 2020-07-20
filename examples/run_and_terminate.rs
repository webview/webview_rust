use std::{thread, time};
use webview_official::{SizeHint, WebviewBuilder};

fn main() {
    let mut webview = WebviewBuilder::new()
        .debug(true)
        .title("TEST")
        .width(800)
        .height(600)
        .resize(SizeHint::NONE)
        .url("https://google.com")
        .build();

    let mut webview_ref = webview.as_mut();

    thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(5));
        webview_ref.terminate().unwrap();
    });

    webview.run();
}
