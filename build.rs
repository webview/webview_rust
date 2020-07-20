use cc::Build;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut build = Build::new();

    let target = env::var("TARGET").unwrap();

    build
        .cpp(true)
        .include("webview-official/webview.h")
        .flag_if_supported("/std:c++11")
        .flag_if_supported("-w");

    // if env::var("DEBUG").is_err() {
    //     build.define("NDEBUG", None);
    // } else {
    //     build.define("DEBUG", None);
    // }

    if target.contains("windows") {
        // build.define("UNICODE", None); // doesn't work atm.
        build
            .file("webview-official/webview.cc")
            .flag_if_supported("/std:c++17");
        build.include("webview-official/script");

        for &lib in &["windowsapp", "user32", "oleaut32", "ole32"] {
            println!("cargo:rustc-link-lib={}", lib);
        }

        let webview2_arch = if target.contains("x86_64") {
            "x64"
        } else {
            "x86"
        };

        // calculate full path to WebView2Loader.dll
        let mut webview2_path_buf = PathBuf::from(env::current_dir().unwrap().to_str().unwrap());
        webview2_path_buf
            .push("webview-official/script/Microsoft.Web.WebView2.0.9.488/build/native");
        webview2_path_buf.push(webview2_arch);
        let webview2_dir = webview2_path_buf.as_path().to_str().unwrap();

        let loader_asm_name = "WebView2Loader.dll";

        println!("cargo:rustc-link-search={}", webview2_dir);
        println!("cargo:rustc-link-lib={}", loader_asm_name);

        // copy WebView2Loader.dll to `target/debug`
        let mut src_asm_buf = PathBuf::from(webview2_dir);
        src_asm_buf.push(loader_asm_name);

        // we want to be able to calculate C:\crate\root\target\debug\
        //           while we can get this ^^^^^^^^^^^^^   and  ^^^^^ from env::current_dir() and %PROFILE% respectively
        // there's no way to get this (reliably)         ^^^^^^
        // we can, however, use %OUT_DIR% (C:\crate\root\target\debug\build\webview_rust-xxxx\out\)
        // and navigate backwards to here  ^^^^^^^^^^^^^^^^^^^^^^^^^^
        let mut target_asm_buf = PathBuf::from(env::var("OUT_DIR").unwrap());
        target_asm_buf.pop();
        target_asm_buf.pop();
        target_asm_buf.pop();
        target_asm_buf.push(loader_asm_name);

        fs::copy(src_asm_buf.as_path(), target_asm_buf.as_path()).unwrap();
    } else if target.contains("apple") {
        build.file("webview-official/webview.cc").flag("-std=c++11");

        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=WebKit");
    } else if target.contains("linux") || target.contains("bsd") {
        let lib = pkg_config::Config::new()
            .atleast_version("2.8")
            .probe("webkit2gtk-4.0")
            .unwrap();

        for path in lib.include_paths {
            build.include(path);
        }
        // pkg_config::Config::new()
        //     .atleast_version("3.0")
        //     .probe("gtk+-3.0")
        //     .unwrap();

        build.file("webview-official/webview.cc");
    } else {
        panic!("Unsupported platform");
    }

    println!("cargo:rerun-if-changed=webview-official/webview.h");
    println!("cargo:rerun-if-changed=webview-official/webview.cc");

    build.compile("webview");
}
