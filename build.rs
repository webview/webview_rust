use cc::Build;
use std::env;

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

        let webview2_path = if target.contains("x86_64") {
            "webview-official/script/Microsoft.Web.WebView2.0.8.355/build/native/x64/WebView2Loader.dll"
        } else {
            "webview-official/script/Microsoft.Web.WebView2.0.8.355/build/native/x86/WebView2Loader.dll"
        };

        println!("cargo:rustc-link-lib={}", webview2_path);

        println!("cargo:rustc-link-search={}", webview2_path);
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
