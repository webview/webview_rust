use webview_rust_sys::{
    webview_create, webview_navigate, webview_run, webview_set_size, webview_set_title, SizeHint,
};

fn main() {
    let mut data = webview_create(true, None);
    webview_set_title(&mut data, "TEST");
    webview_set_size(&mut data, 800, 600, SizeHint::NONE);
    webview_navigate(&mut data, "https://google.com");
    webview_run(&mut data);
}
