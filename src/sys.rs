use std::os::raw::{c_char, c_int, c_void};

pub type DispatchFn = extern "C" fn(webview: webview_t, arg: *mut c_void);
pub type BindFn = extern "C" fn(seq: *const c_char, req: *const c_char, arg: *mut c_void);

#[allow(non_camel_case_types)]
pub type webview_t = *mut c_void;

extern "C" {
    pub fn webview_create(debug: c_int, window: *mut c_void) -> webview_t;

    pub fn webview_destroy(w: webview_t);

    pub fn webview_run(w: webview_t);

    pub fn webview_terminate(w: webview_t);

    pub fn webview_dispatch(w: webview_t, fn_: Option<DispatchFn>, arg: *mut c_void);

    pub fn webview_get_window(w: webview_t) -> *mut c_void;

    pub fn webview_set_title(w: webview_t, title: *const c_char);

    pub fn webview_set_size(w: webview_t, width: c_int, height: c_int, hints: c_int);

    pub fn webview_navigate(w: webview_t, url: *const c_char);

    pub fn webview_init(w: webview_t, js: *const c_char);

    pub fn webview_eval(w: webview_t, js: *const c_char);

    pub fn webview_bind(w: webview_t, name: *const c_char, fn_: Option<BindFn>, arg: *mut c_void);

    pub fn webview_return(w: webview_t, seq: *const c_char, status: c_int, result: *const c_char);
}
