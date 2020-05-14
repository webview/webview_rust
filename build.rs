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

        let webview2_path = if target.contains("x86_64") {
            "webview-official/script/Microsoft.Web.WebView2.0.8.355/build/native/x64"
        } else {
            "webview-official/script/Microsoft.Web.WebView2.0.8.355/build/native/x86"
        };

        for &lib in &["WebView2Loader.dll", "WebView2Loader.dll.lib"] {
            let lib_path = format!("{}/{}", webview2_path, lib);
            println!("cargo:rerun-if-changed={}", lib_path);
        }

        println!("cargo:rustc-link-search={}", webview2_path);

        println!("cargo:rustc-link-lib={}", "WebView2Loader.dll");
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=WebKit");
    }

    println!("cargo:rerun-if-changed=webview-official/webview.h");
    println!("cargo:rerun-if-changed=webview-official/webview.cc");

    build.file("webview-official/webview.cc").flag("/std:c++17");
    build.compile("webview");
}
