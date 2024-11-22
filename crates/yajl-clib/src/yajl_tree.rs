#![allow(clippy::missing_safety_doc)]
#![allow(unused_unsafe)]
#![allow(clippy::nonminimal_bool)]

use core::ffi::c_uint;
use core::ptr;

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
    yajl::tree::yajl_tree_parse(input, error_buffer, error_buffer_size)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_tree_get(
    mut n: yajl_val,
    mut path: *mut *const libc::c_char,
    mut type_0: yajl_type,
) -> yajl_val {
    if let Some(r#type) = ValueType::from_repr(type_0) {
        yajl::tree::yajl_tree_get(n, path, r#type)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn yajl_tree_free(mut v: yajl_val) {
    yajl::tree::yajl_tree_free(v)
}
