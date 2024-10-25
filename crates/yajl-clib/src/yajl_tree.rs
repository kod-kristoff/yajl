#![allow(clippy::missing_safety_doc)]
#![allow(unused_unsafe)]
#![allow(clippy::nonminimal_bool)]

use ::libc;

pub use yajl::yajl_tree::{yajl_type, yajl_val};

#[no_mangle]
pub unsafe extern "C" fn yajl_tree_parse(
    mut input: *const libc::c_char,
    mut error_buffer: *mut libc::c_char,
    mut error_buffer_size: usize,
) -> yajl_val {
    yajl::yajl_tree::yajl_tree_parse(input, error_buffer, error_buffer_size)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_tree_get(
    mut n: yajl_val,
    mut path: *mut *const libc::c_char,
    mut type_0: yajl_type,
) -> yajl_val {
    yajl::yajl_tree::yajl_tree_get(n, path, type_0)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_tree_free(mut v: yajl_val) {
    yajl::yajl_tree::yajl_tree_free(v)
}