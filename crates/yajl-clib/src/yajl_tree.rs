#![allow(clippy::missing_safety_doc)]
#![allow(unused_unsafe)]
#![allow(clippy::nonminimal_bool)]

use core::ptr;
use core::{ffi::c_uint, slice};

use ::libc;

pub use yajl::tree::{Value, ValueType};

pub type yajl_type = c_uint;
pub type yajl_val = *mut Value;

#[no_mangle]
pub unsafe extern "C" fn yajl_tree_parse(
    mut input: *const libc::c_char,
    mut error_buffer: *mut libc::c_char,
    mut error_buffer_size: usize,
) -> yajl_val {
    if input.is_null() {
        if !error_buffer.is_null() {
            let msgs = [
                &b"'input' is NULL which isn't allowed\0"[..],
                &b"'input' is NULL\0"[..],
                &b"NULL\0"[..],
                &b"nil\0"[..],
                &b"\0"[..],
            ];
            let mut msg = msgs[0];
            let mut curr_idx = 0;
            while msg.len() > error_buffer_size {
                curr_idx += 1;
                if curr_idx >= msgs.len() {
                    return ptr::null_mut();
                }
                msg = msgs[curr_idx];
            }
            let err_buf = slice::from_raw_parts_mut(error_buffer, error_buffer_size);
            err_buf[..msg.len()].copy_from_slice(&*(msg as *const [u8] as *const [i8]));
        }
        return ptr::null_mut();
    }
    match yajl::tree::yajl_tree_parse(input, error_buffer, error_buffer_size) {
        Some(value) => value,
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn yajl_tree_get(
    mut n: yajl_val,
    mut path: *mut *const libc::c_char,
    mut type_0: yajl_type,
) -> yajl_val {
    ffi_helpers::null_pointer_check!(n);
    if let Some(r#type) = ValueType::from_repr(type_0) {
        match yajl::tree::yajl_tree_get(n, path, r#type) {
            Some(value) => value,
            None => ptr::null_mut(),
        }
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn yajl_tree_free(mut v: yajl_val) {
    Value::tree_free(v)
}
