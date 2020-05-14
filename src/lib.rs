use std::ffi::CString;
use std::os::raw::*;
use std::ptr::null_mut;

type DispatchFn = extern "C" fn(webview: *mut Webview, arg: *mut c_void);
type BindFn = extern "C" fn(seq: *const c_char, req: *const c_char, arg: *mut c_void);

pub enum Webview {}
pub enum Window {}

#[repr(i32)]
pub enum SizeHint {
    NONE = 0,
    MIN = 1,
    MAX = 2,
    FIXED = 3,
}

mod ffi {
    use crate::{BindFn, DispatchFn, SizeHint, Webview, Window};
    use std::os::raw::*;

    extern "C" {
        pub fn webview_create(debug: c_int, window: *mut Window) -> *mut Webview;
        pub fn webview_destroy(w: *mut Webview);
        pub fn webview_run(w: *mut Webview);
        pub fn webview_terminate(w: *mut Webview);
        pub fn webview_dispatch(w: *mut Webview, f: Option<DispatchFn>, arg: *mut c_void);
        pub fn webview_get_window(w: *mut Webview) -> *mut Window;
        pub fn webview_set_title(w: *mut Webview, title: *const c_char);
        pub fn webview_set_size(w: *mut Webview, width: c_int, height: c_int, hints: SizeHint);
        pub fn webview_navigate(w: *mut Webview, url: *const c_char);
        pub fn webview_init(w: *mut Webview, js: *const c_char);
        pub fn webview_eval(w: *mut Webview, js: *const c_char);
        pub fn webview_bind(
            w: *mut Webview,
            name: *const c_char,
            f: Option<BindFn>,
            arg: *mut c_void,
        );
        pub fn webview_return(
            w: *mut Webview,
            seq: *const c_char,
            status: c_int,
            result: *const c_char,
        );
    }
}

pub use ffi::{
    webview_destroy, webview_get_window, webview_run, webview_set_size, webview_terminate,
};

pub unsafe fn webview_create(debug: bool, window: Option<*mut Window>) -> *mut Webview {
    if let Some(w) = window {
        ffi::webview_create(debug as c_int, w)
    } else {
        ffi::webview_create(debug as c_int, null_mut())
    }
}

pub unsafe fn webview_dispatch(w: *mut Webview, f: DispatchFn, arg: *mut c_void) {
    ffi::webview_dispatch(w, f.into(), arg)
}

pub unsafe fn webview_set_title(w: *mut Webview, title: &str) {
    let c_title = CString::new(title).expect("No nul bytes in parameter title");
    ffi::webview_set_title(w, c_title.as_ptr())
}

pub unsafe fn webview_navigate(w: *mut Webview, url: &str) {
    let c_url = CString::new(url).expect("No nul bytes in parameter url");
    ffi::webview_navigate(w, c_url.as_ptr())
}

pub unsafe fn webview_init(w: *mut Webview, js: &str) {
    let c_js = CString::new(js).expect("No nul bytes in parameter js");
    ffi::webview_init(w, c_js.as_ptr())
}

pub unsafe fn webview_eval(w: *mut Webview, js: &str) {
    let c_js = CString::new(js).expect("No nul bytes in parameter js");
    ffi::webview_eval(w, c_js.as_ptr())
}

pub unsafe fn webview_bind(w: *mut Webview, name: &str, f: BindFn, arg: *mut c_void) {
    let c_name = CString::new(name).expect("No nul bytes in parameter name");
    ffi::webview_bind(w, c_name.as_ptr(), f.into(), arg)
}

pub unsafe fn webview_return(w: *mut Webview, seq: &str, status: c_int, result: &str) {
    let c_seq = CString::new(seq).expect("No nul bytes in parameter seq");
    let c_result = CString::new(result).expect("No nul bytes in parameter result");
    ffi::webview_return(w, c_seq.as_ptr(), status, c_result.as_ptr())
}
