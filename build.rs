use cc::Build;
use std::env;

fn main() {
    let mut build = Build::new();

    let target = env::var("TARGET").unwrap();

    build.include("webview-offical");

    if target.contains("windows") {
        build.include("webview-official/script");

        for &lib in &["windowsapp", "user32", "oleaut32", "ole32"] {
            println!("cargo:rustc-link-lib={}", lib);
        }

        if target.contains("x86_64") {
            println!(
                "cargo:rustc-link-search={}",
                "webview-official/dll/x64"
            );
        } else {
            println!(
                "cargo:rustc-link-search={}",
                "webview-official/dll/x86"
            );
        }

        println!(
            "cargo:rustc-link-lib={}",
            "WebView2Loader.dll"
        );
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=WebKit");
    }

    build.file("webview-official/webview.cc").flag_if_supported("/std:c++17");
    build.compile("webview");

}
