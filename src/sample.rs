use std::ptr::null_mut;
use webview_rust_sys::{webview_navigate, webview_create, webview_set_title, webview_set_size, webview_run, SizeHint};

fn main() {
    unsafe {
        let data = webview_create(true, null_mut());
        webview_set_title(data, "TEST");
        webview_set_size(data, 800, 600, SizeHint::NONE);
        webview_navigate(data, "https://google.com");
        webview_run(data);
    }
}