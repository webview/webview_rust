use webview_rust_sys::{
    webview_create, webview_navigate, webview_run, webview_set_size, webview_set_title, SizeHint,
};

fn main() {
    unsafe {
        let data = webview_create(true, None);
        webview_set_title(data, "TEST");
        webview_set_size(data, 800, 600, SizeHint::NONE);
        webview_navigate(data, "https://google.com");
        webview_run(data);
    }
}
