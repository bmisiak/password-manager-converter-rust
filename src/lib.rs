#![feature(let_chains)]
use std::ffi::{c_char, c_int, CStr};

use conversion::run_conversion;

mod conversion;
mod sinks;
mod sources;

/// # Safety
/// All four arguments need to be valid pointers to null-terminated C strings.
/// The caller is responsible for freeing the strings. The function does not take
/// ownership.
#[no_mangle]
pub unsafe extern "C" fn convert_from_source_to_sink(
    source_name: *const c_char,
    source_path: *const c_char,
    sink_name: *const c_char,
    sink_path: *const c_char,
) -> c_int {
    let source_name = match unsafe { CStr::from_ptr(source_name) }.to_str() {
        Ok(name) => name,
        _ => return 1,
    };
    let source_path = match unsafe { CStr::from_ptr(source_path) }.to_str() {
        Ok(path) => path,
        _ => return 2,
    };
    let sink_name = match unsafe { CStr::from_ptr(sink_name) }.to_str() {
        Ok(name) => name,
        _ => return 3,
    };
    let sink_path = match unsafe { CStr::from_ptr(sink_path) }.to_str() {
        Ok(path) => path,
        _ => return 4,
    };

    match run_conversion(source_name, source_path, sink_name, sink_path) {
        Ok(()) => 0,
        Err(_err) => 5,
    }
}
