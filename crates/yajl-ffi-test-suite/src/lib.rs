#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern "C" {
    #[cfg(feature = "nightly")]
    fn yajl_gen_config(g: yajl_gen, opt: yajl_gen_option, args: ...) -> c_int;
    #[cfg(not(feature = "nightly"))]
    fn yajl_gen_config_set_indent(
        g: yajl_gen,
        opt: yajl_gen_option,
        indent: *const c_char,
    ) -> c_int;

    #[cfg(not(feature = "nightly"))]
    fn yajl_gen_config(g: yajl_gen, opt: yajl_gen_option, arg: c_int) -> c_int;
    #[cfg(not(feature = "nightly"))]
    fn yajl_gen_config_print_callback(
        g: yajl_gen,
        opt: yajl_gen_option,
        arg: c_int,
        print: unsafe extern "C" fn(*mut c_void, *const c_char, usize) -> (),
        ctx: *mut c_void,
    ) -> c_int;
    fn yajl_gen_alloc(afs: *const yajl_alloc_funcs) -> yajl_gen;
    fn yajl_gen_reset(g: yajl_gen, sep: *const c_char);
    fn yajl_gen_free(g: yajl_gen);

    fn yajl_gen_integer(g: yajl_gen, number: c_longlong) -> yajl_gen_status;

    fn yajl_gen_double(g: yajl_gen, number: c_double) -> yajl_gen_status;

    fn yajl_gen_number(g: yajl_gen, s: *const c_char, l: usize) -> yajl_gen_status;

    fn yajl_gen_string(g: yajl_gen, str: *const c_uchar, len: usize) -> yajl_gen_status;

    fn yajl_gen_null(g: yajl_gen) -> yajl_gen_status;

    fn yajl_gen_bool(g: yajl_gen, boolean: c_int) -> yajl_gen_status;

    fn yajl_gen_map_open(g: yajl_gen) -> yajl_gen_status;

    fn yajl_gen_map_close(g: yajl_gen) -> yajl_gen_status;

    fn yajl_gen_array_open(g: yajl_gen) -> yajl_gen_status;

    fn yajl_gen_array_close(g: yajl_gen) -> yajl_gen_status;

    fn yajl_gen_get_buf(
        g: yajl_gen,
        buf: *mut *const c_uchar,
        len: *const usize,
    ) -> yajl_gen_status;

    fn yajl_gen_clear(g: yajl_gen);
    fn yajl_alloc(
        callbacks: *const yajl_callbacks,
        afs: *mut yajl_alloc_funcs,
        ctx: *mut c_void,
    ) -> yajl_handle;

    fn yajl_free(handle: yajl_handle);

    #[cfg(feature = "nightly")]
    fn yajl_config(h: yajl_handle, opt: yajl_option, args: ...) -> c_int;
    #[cfg(not(feature = "nightly"))]
    fn yajl_config(h: *mut yajl_handle_t, opt: yajl_option, arg: c_int) -> c_int;

    fn yajl_status_to_string(stat: yajl_status) -> *const c_char;

    fn yajl_parse(hand: yajl_handle, jsonText: *const c_uchar, jsonTextLen: usize) -> yajl_status;

    fn yajl_complete_parse(hand: yajl_handle) -> yajl_status;

    fn yajl_get_error(
        hand: yajl_handle,
        verbose: c_int,
        jsonText: *const c_uchar,
        jsonTextLen: usize,
    ) -> *mut c_uchar;
    fn yajl_get_bytes_consumed(hand: yajl_handle) -> usize;
    fn yajl_free_error(hand: yajl_handle, str: *mut c_uchar);
    fn yajl_tree_parse(
        input: *const c_char,
        error_buffer: *mut c_char,
        error_buffer_size: usize,
    ) -> yajl_val;

    fn yajl_tree_get(n: yajl_val, path: *mut *const c_char, type_0: yajl_type) -> yajl_val;

    fn yajl_tree_free(v: yajl_val);
    fn yajl_version() -> c_int;
}

use std::{
    ffi::{c_char, c_double, c_int, c_longlong, c_uchar, c_uint, c_void},
    ptr,
};

#[repr(C)]
pub struct yajl_gen_t {
    _private: [u8; 0],
}

pub type yajl_gen = *mut yajl_gen_t;
pub type yajl_gen_option = c_uint;
pub type yajl_option = c_uint;
pub type yajl_gen_status = c_uint;
pub const yajl_gen_invalid_string: yajl_gen_status = 7;
pub const yajl_gen_no_buf: yajl_gen_status = 6;
pub const yajl_gen_invalid_number: yajl_gen_status = 5;
pub const yajl_gen_generation_complete: yajl_gen_status = 4;
pub const yajl_gen_in_error_state: yajl_gen_status = 3;
pub const yajl_max_depth_exceeded: yajl_gen_status = 2;
pub const yajl_gen_keys_must_be_strings: yajl_gen_status = 1;
pub const yajl_gen_status_ok: yajl_gen_status = 0;
pub type yajl_status = c_uint;
pub type yajl_val = *mut yajl_val_s;
pub type yajl_malloc_func = Option<unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void>;
pub type yajl_free_func = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>;
pub type yajl_realloc_func =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> *mut c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut c_void,
}

#[repr(C)]
pub struct yajl_handle_t {
    _private: [u8; 0],
}

pub type yajl_handle = *mut yajl_handle_t;
#[repr(C)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub yajl_boolean: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    pub yajl_integer: Option<unsafe extern "C" fn(*mut c_void, c_longlong) -> c_int>,
    pub yajl_double: Option<unsafe extern "C" fn(*mut c_void, c_double) -> c_int>,
    pub yajl_number: Option<unsafe extern "C" fn(*mut c_void, *const c_char, usize) -> c_int>,
    pub yajl_string: Option<unsafe extern "C" fn(*mut c_void, *const c_uchar, usize) -> c_int>,
    pub yajl_start_map: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub yajl_map_key: Option<unsafe extern "C" fn(*mut c_void, *const c_uchar, usize) -> c_int>,
    pub yajl_end_map: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub yajl_start_array: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub yajl_end_array: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
}
pub type yajl_type = c_uint;
pub const yajl_t_any: yajl_type = 8;
pub const yajl_t_null: yajl_type = 7;
pub const yajl_t_false: yajl_type = 6;
pub const yajl_t_true: yajl_type = 5;
pub const yajl_t_array: yajl_type = 4;
pub const yajl_t_object: yajl_type = 3;
pub const yajl_t_number: yajl_type = 2;
pub const yajl_t_string: yajl_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_val_s {
    pub type_0: yajl_type,
    pub u: U,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union U {
    pub string: *mut c_char,
    pub number: Number,
    pub object: Object,
    pub array: Array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Array {
    pub values: *mut yajl_val,
    pub len: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Object {
    pub keys: *mut *const c_char,
    pub values: *mut yajl_val,
    pub len: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Number {
    pub i: c_longlong,
    pub d: c_double,
    pub r: *mut c_char,
    pub flags: c_uint,
}

pub struct FreeGuard<T> {
    p: *mut T,
    free: unsafe extern "C" fn(*mut T),
}
impl<T> FreeGuard<T> {
    pub fn new(p: *mut T, free: unsafe extern "C" fn(*mut T)) -> Self {
        Self { p, free }
    }
}

impl<T> Drop for FreeGuard<T> {
    fn drop(&mut self) {
        if !self.p.is_null() {
            unsafe {
                (self.free)(self.p);
            }
            self.p = ptr::null_mut();
        }
    }
}
