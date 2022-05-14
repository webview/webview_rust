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

  let mut webview_ = webview.clone();

  thread::spawn(move || {
    thread::sleep(time::Duration::from_secs(5));
    webview_.terminate();
  });

  webview.run();
}
