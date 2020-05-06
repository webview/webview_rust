use bitflags::bitflags;

use std::os::raw::*;

pub enum WebView {}
pub enum Window {}

type DispatchFn = extern "C" fn(webview: *mut WebView, arg: *mut c_void);
type BindFn = extern "C" fn(seq: *const c_char, req: *const c_char, arg: *mut c_void);

bitflags! {
    #[repr(C)]
    pub struct SizeHint: u8 {
        const NONE = 0;
        const MIN = 1;
        const MAX = 2;
        const FIXED = 3;
    }
}

extern "C" {
    pub fn webview_create(debug: c_int, window: *mut Window) -> *mut WebView;
    pub fn webview_destroy(w: *mut WebView);
    pub fn webview_run(w: *mut WebView);
    pub fn webview_terminate(w: *mut WebView);
    pub fn webview_dispatch(w: *mut WebView, f: Option<DispatchFn>, arg: *mut c_void);
    pub fn webview_get_window(w: *mut WebView) -> *mut Window;
    pub fn webview_set_title(w: *mut WebView, title: *const c_char);
    pub fn webview_set_size(w: *mut WebView, width: c_int, height: c_int, hints: SizeHint);
    pub fn webview_navigate(w: *mut WebView, title: *const c_char);
    pub fn webview_init(w: *mut WebView, js: *const c_char);
    pub fn webview_eval(w: *mut WebView, js: *const c_char);
    pub fn webview_bind(w: *mut WebView, name: *const c_char, f: Option<BindFn>, arg: *mut c_void);
    pub fn webview_return(
        w: *mut WebView,
        seq: *const c_char,
        status: c_int,
        result: *const c_char,
    );
}
