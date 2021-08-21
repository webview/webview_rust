use cc::Build;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut build = Build::new();
    let target = env::var("TARGET").unwrap();

    if target.contains("windows") {
        build
            .file("webview-official/webview.cc")
            .flag_if_supported("/std:c++17");
        build.include("webview-official/script");

        for &lib in &["windowsapp", "user32", "oleaut32", "ole32", "version", "shell32"] {
            println!("cargo:rustc-link-lib={}", lib);
        }

        let webview2_arch = if target.contains("x86_64") {
            "x64"
        } else {
            "x86"
        };

        // calculate full path to WebView2LoaderStatic.lib
        let mut webview2_path_buf = PathBuf::from(env::current_dir().unwrap().to_str().unwrap());
        webview2_path_buf
            .push("webview-official/script/microsoft.web.webview2.1.0.664.37/build/native");
        webview2_path_buf.push(webview2_arch);
        let webview2_dir = webview2_path_buf.as_path().to_str().unwrap();

        let loader_asm_name = "WebView2LoaderStatic";

        println!("cargo:rustc-link-search={}", webview2_dir);
        println!("cargo:rustc-link-lib={}", loader_asm_name);
    } else if target.contains("apple") {
        build.file("webview-official/webview.cc").flag("-std=c++11");

        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=WebKit");
    } else if target.contains("linux") || target.contains("bsd") {
        build.file("webview-official/webview.cc");
        let lib = pkg_config::Config::new()
            .atleast_version("2.8")
            .probe("webkit2gtk-4.0")
            .unwrap();

        for path in lib.include_paths {
            build.include(path);
        }
    } else {
        panic!("Unsupported platform");
    }

    println!("cargo:rerun-if-changed=webview-official/webview.h");
    println!("cargo:rerun-if-changed=webview-official/webview.cc");

    build.compile("webview");
}
