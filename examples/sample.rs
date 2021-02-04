use webview_official::{SizeHint, WebviewBuilder};

fn main() {
    let mut webview = WebviewBuilder::new()
        .debug(true)
        .title("TEST")
        .width(1024)
        .height(768)
        .resize(SizeHint::NONE)
        .init("window.x = 42")
        .dispatch(|mut w| {
            w.set_size(800, 600, SizeHint::MIN);
            println!("Hello World");
        })
        .url("https://google.com")
        .build();

    webview.eval("console.log('The anwser is ' + window.x);");
    let w = webview.clone();
    webview.bind("xxx", move |seq, _req| {
        w.r#return(seq, 0, "{ result: 'We always knew it!' }");
    });
    webview.run();
}
