use cc::Build;
use std::env;

fn main() {
    let mut build = Build::new();

    let target = env::var("TARGET").unwrap();

    build
        .include("webview-offical/webview.h")
        .flag_if_supported("-std=c11")
        .flag_if_supported("-w");

    if target.contains("windows") {
        build.include("webview-offical/script/WebView2.h");
        build.include("webview.cc").flag_if_supported("/std:c++17");
    }

    println!(
        "cargo:rustc-link-lib={}",
        "webview-offical/dll/x64/webview.dll"
    );
    println!(
        "cargo:rustc-link-lib={}",
        "webview-offical/dll/x64/WebView2Loader.dll"
    );
}
