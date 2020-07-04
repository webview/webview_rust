use std::ffi::CString;
use std::os::raw::*;
use std::ptr::null_mut;

pub struct Webview(*mut ffi::Webview);

impl Drop for Webview {
    fn drop(&mut self) {
        unsafe { ffi::webview_destroy(self.0) }
    }
}

pub enum Window {}

#[repr(i32)]
pub enum SizeHint {
    NONE = 0,
    MIN = 1,
    MAX = 2,
    FIXED = 3,
}

mod ffi {
    use crate::{SizeHint, Window};
    use std::os::raw::*;

    pub type DispatchFn = extern "C" fn(webview: *mut Webview, arg: *mut c_void);
    pub type BindFn = extern "C" fn(seq: *const c_char, req: *const c_char, arg: *mut c_void);

    pub enum Webview {}

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

pub fn webview_create(debug: bool, window: Option<&mut Window>) -> Webview {
    if let Some(w) = window {
        Webview(unsafe { ffi::webview_create(debug as c_int, w as *mut Window) })
    } else {
        Webview(unsafe { ffi::webview_create(debug as c_int, null_mut()) })
    }
}

pub fn webview_dispatch(w: &mut Webview, f: ffi::DispatchFn, arg: &str) {
    let c_arg = CString::new(arg).expect("No nul bytes in parameter arg");
    unsafe { ffi::webview_dispatch(w.0, f.into(), c_arg.as_ptr() as *mut _) }
}

pub fn webview_set_title(w: &mut Webview, title: &str) {
    let c_title = CString::new(title).expect("No nul bytes in parameter title");
    unsafe { ffi::webview_set_title(w.0, c_title.as_ptr()) }
}

pub fn webview_navigate(w: &mut Webview, url: &str) {
    let c_url = CString::new(url).expect("No nul bytes in parameter url");
    unsafe { ffi::webview_navigate(w.0, c_url.as_ptr()) }
}

pub fn webview_init(w: &mut Webview, js: &str) {
    let c_js = CString::new(js).expect("No nul bytes in parameter js");
    unsafe { ffi::webview_init(w.0, c_js.as_ptr()) }
}

pub fn webview_eval(w: &mut Webview, js: &str) {
    let c_js = CString::new(js).expect("No nul bytes in parameter js");
    unsafe { ffi::webview_eval(w.0, c_js.as_ptr()) }
}

pub fn webview_bind(w: &mut Webview, name: &str, f: ffi::BindFn, arg: &str) {
    let c_name = CString::new(name).expect("No nul bytes in parameter name");
    let c_arg = CString::new(arg).expect("No nul bytes in parameter arg");
    unsafe { ffi::webview_bind(w.0, c_name.as_ptr(), f.into(), c_arg.as_ptr() as *mut _) }
}

pub fn webview_return(w: &mut Webview, seq: &str, status: c_int, result: &str) {
    let c_seq = CString::new(seq).expect("No nul bytes in parameter seq");
    let c_result = CString::new(result).expect("No nul bytes in parameter result");
    unsafe { ffi::webview_return(w.0, c_seq.as_ptr(), status, c_result.as_ptr()) }
}

pub fn webview_set_size(w: &mut Webview, width: i32, height: i32, hints: SizeHint) {
    unsafe { ffi::webview_set_size(w.0, width, height, hints) }
}

pub fn webview_run(w: &mut Webview) {
    unsafe { ffi::webview_run(w.0) }
}

pub fn webview_destroy(w: Webview) {
    unsafe { ffi::webview_destroy(w.0) }
}

pub fn webview_get_window(w: &mut Webview) -> *mut Window {
    unsafe { ffi::webview_get_window(w.0) }
}

pub fn webview_terminate(w: &mut Webview) {
    unsafe { ffi::webview_terminate(w.0) }
}
