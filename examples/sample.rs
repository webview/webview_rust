use webview_rust_sys::{
    Webview, SizeHint,
};

fn main() {
    let mut data = Webview::create(true, None);
    data.set_title("TEST");
    data.set_size(800, 600, SizeHint::NONE);
    data.navigate("https://google.com");
    data.run();
}
