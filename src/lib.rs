use std::ffi::{CString, CStr};
use std::os::raw::*;
use std::ptr::null_mut;

mod sys;

pub enum Window {}

#[repr(i32)]
pub enum SizeHint {
    NONE = 0,
    MIN = 1,
    MAX = 2,
    FIXED = 3,
}

pub struct Webview(sys::webview_t);

impl Drop for Webview {
    fn drop(&mut self) {
        unsafe { sys::webview_destroy(self.0) }
    }
}

impl Webview {
    pub fn create(debug: bool, window: Option<&mut Window>) -> Webview {
        if let Some(w) = window {
            Webview(unsafe { sys::webview_create(debug as c_int, w as *mut Window as *mut _) })
        } else {
            Webview(unsafe { sys::webview_create(debug as c_int, null_mut()) })
        }
    }

    pub fn dispatch<F>(&mut self, f: F)
    where F: FnOnce(&mut Webview) + Send + 'static
    {
        let closure = Box::into_raw(Box::new(f));
        extern "C" fn callback<F>(webview: sys::webview_t, arg: *mut c_void)
        where F: FnOnce(&mut Webview) + Send + 'static
        {
            let mut  webview = Webview(webview);
            let closure: Box<F> = unsafe { Box::from_raw(arg as *mut F) };
            (*closure)(&mut webview);
        }
        unsafe { sys::webview_dispatch(self.0, Some(callback::<F>), closure as *mut _) }
    }
    
    pub fn set_title(&mut self, title: &str) {
        let c_title = CString::new(title).expect("No nul bytes in parameter title");
        unsafe { sys::webview_set_title(self.0, c_title.as_ptr()) }
    }
    
    pub fn navigate(&mut self, url: &str) {
        let c_url = CString::new(url).expect("No nul bytes in parameter url");
        unsafe { sys::webview_navigate(self.0, c_url.as_ptr()) }
    }
    
    pub fn init(&mut self, js: &str) {
        let c_js = CString::new(js).expect("No nul bytes in parameter js");
        unsafe { sys::webview_init(self.0, c_js.as_ptr()) }
    }
    
    pub fn eval(&mut self, js: &str) {
        let c_js = CString::new(js).expect("No nul bytes in parameter js");
        unsafe { sys::webview_eval(self.0, c_js.as_ptr()) }
    }
<<<<<<< HEAD

    pub fn ind<F>(&mut self, name: &str, f: F)
=======
    unsafe { ffi::webview_dispatch(w.0, Some(callback::<F>), closure as *mut _) }
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

pub fn webview_bind<F>(w: &mut Webview, name: &str, f: F)
where F: FnOnce(&str, &str) + 'static
{
    let c_name = CString::new(name).expect("No nul bytes in parameter name");
    let closure = Box::into_raw(Box::new(f));
    extern "C" fn callback<F>(seq: *const c_char, req: *const c_char, arg: *mut c_void)
>>>>>>> edbb1cc... Fix function name on bind
    where F: FnOnce(&str, &str) + 'static
    {
        let c_name = CString::new(name).expect("No null bytes in parameter name");
        let closure = Box::into_raw(Box::new(f));
        extern "C" fn callback<F>(seq: *const c_char, req: *const c_char, arg: *mut c_void)
        where F: FnOnce(&str, &str) + 'static
        {
            let seq = unsafe { CStr::from_ptr(seq).to_str().expect("No null bytes in parameter seq") };
            let req = unsafe { CStr::from_ptr(req).to_str().expect("No null bytes in parameter req") };
            let closure: Box<F> = unsafe { Box::from_raw(arg as *mut F) };
            (*closure)(seq, req);
        }
        unsafe { sys::webview_bind(self.0, c_name.as_ptr(), Some(callback::<F>), closure as *mut _) }
    }
    
    pub fn r#return(&mut self, seq: &str, status: c_int, result: &str) {
        let c_seq = CString::new(seq).expect("No nul bytes in parameter seq");
        let c_result = CString::new(result).expect("No nul bytes in parameter result");
        unsafe { sys::webview_return(self.0, c_seq.as_ptr(), status, c_result.as_ptr()) }
    }
    
    pub fn set_size(&mut self, width: i32, height: i32, hints: SizeHint) {
        unsafe { sys::webview_set_size(self.0, width, height, hints as i32) }
    }
    
    pub fn run(&mut self) {
        unsafe { sys::webview_run(self.0) }
    }
    
    pub fn destroy(self) {
        unsafe { sys::webview_destroy(self.0) }
    }
    
    pub fn get_window(&mut self) -> *mut Window {
        unsafe { sys::webview_get_window(self.0) as *mut Window }
    }
    
    pub fn terminate(&mut self) {
        unsafe { sys::webview_terminate(self.0) }
    }
}
