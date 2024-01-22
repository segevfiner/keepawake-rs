use std::{ptr, ffi::{c_char, CStr}};

use crate::{Builder, KeepAwake};

#[no_mangle]
pub extern "C" fn keepawake_new() -> *mut Builder {
    Box::into_raw(Box::new(Builder::default()))
}

#[no_mangle]
pub unsafe extern "C" fn keepawake_display(builder: *mut Builder, value: bool) {
    assert!(!builder.is_null());
    (*builder).display(value);
}

#[no_mangle]
pub unsafe extern "C" fn keepawake_idle(builder: *mut Builder, value: bool) {
    assert!(!builder.is_null());
    (*builder).idle(value);
}

#[no_mangle]
pub unsafe extern "C" fn keepawake_sleep(builder: *mut Builder, value: bool) {
    assert!(!builder.is_null());
    (*builder).sleep(value);
}

#[no_mangle]
pub unsafe extern "C" fn keepawake_reason(builder: *mut Builder, value: *const c_char) {
    assert!(!builder.is_null());
    (*builder).reason(CStr::from_ptr(value).to_string_lossy());
}

#[no_mangle]
pub unsafe extern "C" fn keepawake_app_name(builder: *mut Builder, value: *const c_char) {
    assert!(!builder.is_null());
    (*builder).app_name(CStr::from_ptr(value).to_string_lossy());
}

#[no_mangle]
pub unsafe extern "C" fn keepawake_app_reverse_domain(builder: *mut Builder, value: *const c_char) {
    assert!(!builder.is_null());
    (*builder).app_reverse_domain(CStr::from_ptr(value).to_string_lossy());
}

#[no_mangle]
pub unsafe extern "C" fn keepawake_create(builder: *mut Builder) -> *mut KeepAwake {
    assert!(!builder.is_null());
    (*builder).create().map_or(ptr::null_mut(), |v| Box::into_raw(Box::new(v)))
}

#[no_mangle]
pub unsafe extern "C" fn keepawake_builder_destroy(builder: *mut Builder) {
    assert!(!builder.is_null());
    drop(Box::from_raw(builder));
}

#[no_mangle]
pub unsafe extern "C" fn keepawake_destroy(awake: *mut KeepAwake) {
    assert!(!awake.is_null());
    drop(Box::from_raw(awake));
}
