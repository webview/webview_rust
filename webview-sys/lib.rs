use std::os::raw::*;

type DispatchFn = extern "C" fn(webview: *mut Webview, arg: *mut c_void);
type BindFn = extern "C" fn(seq: *const c_char, req: *const c_char , arg: *mut c_void);

#[repr(C)]
pub struct Window {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct Webview {
    _unused: [u8; 0],
}

extern "C" {
    pub fn webview_create(debug: c_int, window: *mut Window) -> *mut Webview;
    pub fn webview_destroy(w: *mut Webview);
    pub fn webview_run(w: *mut Webview);
    pub fn webview_terminate(w: *mut Webview);
    pub fn webview_dispatch(w: *mut Webview, f: DispatchFn, arg: *mut c_void);
    pub fn webview_get_window(w: *mut Webview) -> *mut Window;
    pub fn webview_set_title(w: *mut Webview, title: *const c_char);
    pub fn webview_set_size(w: *mut Webview, width: c_int, height: c_int, hints: c_int);
    pub fn webview_navigate(w: *mut Webview, title: *const c_char);
    pub fn webview_init(w: *mut Webview, js: *const c_char);
    pub fn webview_eval(w: *mut Webview, js: *const c_char);
    pub fn webview_bind(w: *mut Webview, name: *const c_char, f: BindFn, arg: *mut c_void);
    pub fn webview_return(w: *mut Webview, seq: *const c_char, status: c_int, result: *const c_char);
}
