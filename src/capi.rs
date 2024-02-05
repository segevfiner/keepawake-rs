use std::{
    ffi::{c_char, CStr},
    ptr,
};

use crate::{Builder, KeepAwake};

/// Create a new [`KeepAwakeBuilder`].
#[no_mangle]
pub extern "C" fn keepawake_new() -> *mut Builder {
    Box::into_raw(Box::new(Builder::default()))
}

/// Prevent the display from turning off.
#[no_mangle]
pub unsafe extern "C" fn keepawake_display(builder: *mut Builder, value: bool) {
    assert!(!builder.is_null());
    (*builder).display(value);
}

/// Prevent the system from sleeping due to idleness.
#[no_mangle]
pub unsafe extern "C" fn keepawake_idle(builder: *mut Builder, value: bool) {
    assert!(!builder.is_null());
    (*builder).idle(value);
}

/// Prevent the system from sleeping. Only works under certain, OS dependant, conditions.
#[no_mangle]
pub unsafe extern "C" fn keepawake_sleep(builder: *mut Builder, value: bool) {
    assert!(!builder.is_null());
    (*builder).sleep(value);
}

/// Reason the consumer is keeping the system awake. Defaults to `"User requested"`. (Used on Linux & macOS)
#[no_mangle]
pub unsafe extern "C" fn keepawake_reason(builder: *mut Builder, value: *const c_char) {
    assert!(!builder.is_null());
    (*builder).reason(CStr::from_ptr(value).to_string_lossy());
}

/// Name of the program keeping the system awake. Defaults to `"keepawake-rs"`. (Used on Linux)
#[no_mangle]
pub unsafe extern "C" fn keepawake_app_name(builder: *mut Builder, value: *const c_char) {
    assert!(!builder.is_null());
    (*builder).app_name(CStr::from_ptr(value).to_string_lossy());
}

/// Reverse domain name of the program keeping the system awake. Defaults to `"io.github.segevfiner.keepawake-rs"`. (Used on Linux)
#[no_mangle]
pub unsafe extern "C" fn keepawake_app_reverse_domain(builder: *mut Builder, value: *const c_char) {
    assert!(!builder.is_null());
    (*builder).app_reverse_domain(CStr::from_ptr(value).to_string_lossy());
}

/// Create the [`KeepAwake`]. Optionally destroying the builder.
#[no_mangle]
pub unsafe extern "C" fn keepawake_create(
    builder: *mut Builder,
    free_builder: bool,
) -> *mut KeepAwake {
    assert!(!builder.is_null());
    let result = (*builder).create();
    if free_builder {
        drop(Box::from_raw(builder));
    }
    result.map_or(ptr::null_mut(), |v| Box::into_raw(Box::new(v)))
}

/// Destroy the [`KeepAwakeBuilder`].
#[no_mangle]
pub unsafe extern "C" fn keepawake_builder_destroy(builder: *mut Builder) {
    assert!(!builder.is_null());
    drop(Box::from_raw(builder));
}

/// Destroy the [`KeepAwake`].
#[no_mangle]
pub unsafe extern "C" fn keepawake_destroy(awake: *mut KeepAwake) {
    assert!(!awake.is_null());
    drop(Box::from_raw(awake));
}
