use webview_rust_sys::{SizeHint, Webview};

fn main() {
    let mut data = Webview::create(true, None);
    data.set_title("TEST");
    data.set_size(1024, 768, SizeHint::NONE);
    data.init("window.x = 42");
    data.dispatch(|w| {
        w.set_size(800, 600, SizeHint::MIN);
        println!("Hello World");
    });
    let mut w = data.clone();
    data.bind("xxx", move |seq, _req| {
        w.eval("console.log('The anwser is ' + window.x);");
        w.r#return(seq, 0, "{ result: 'We always knew it!' }");
    });
    data.navigate("https://google.com");
    data.run();
}
