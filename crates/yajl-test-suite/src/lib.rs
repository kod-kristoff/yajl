#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern "C" {
    #[cfg(feature = "nightly")]
    fn yajl_gen_config(g: yajl_gen, opt: yajl_gen_option, args: ...) -> libc::c_int;
    #[cfg(not(feature = "nightly"))]
    fn yajl_gen_config_set_indent(
        g: yajl_gen,
        opt: yajl_gen_option,
        indent: *const libc::c_char,
    ) -> libc::c_int;

    #[cfg(not(feature = "nightly"))]
    fn yajl_gen_config(g: yajl_gen, opt: yajl_gen_option, arg: libc::c_int) -> libc::c_int;
    #[cfg(not(feature = "nightly"))]
    fn yajl_gen_config_print_callback(
        g: yajl_gen,
        opt: yajl_gen_option,
        arg: libc::c_int,
        print: unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, usize) -> (),
        ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn yajl_gen_alloc(afs: *const yajl_alloc_funcs) -> yajl_gen;
    fn yajl_gen_reset(g: yajl_gen, sep: *const libc::c_char);
    fn yajl_gen_free(g: yajl_gen);

    fn yajl_gen_integer(g: yajl_gen, number: libc::c_longlong) -> yajl_gen_status;

    fn yajl_gen_double(g: yajl_gen, number: libc::c_double) -> yajl_gen_status;

    fn yajl_gen_number(g: yajl_gen, s: *const libc::c_char, l: usize) -> yajl_gen_status;

    fn yajl_gen_string(g: yajl_gen, str: *const libc::c_uchar, len: usize) -> yajl_gen_status;

    fn yajl_gen_null(g: yajl_gen) -> yajl_gen_status;

    fn yajl_gen_bool(g: yajl_gen, boolean: libc::c_int) -> yajl_gen_status;

    fn yajl_gen_map_open(g: yajl_gen) -> yajl_gen_status;

    fn yajl_gen_map_close(g: yajl_gen) -> yajl_gen_status;

    fn yajl_gen_array_open(g: yajl_gen) -> yajl_gen_status;

    fn yajl_gen_array_close(g: yajl_gen) -> yajl_gen_status;

    fn yajl_gen_get_buf(
        g: yajl_gen,
        buf: *mut *const libc::c_uchar,
        len: *const usize,
    ) -> yajl_gen_status;

    fn yajl_gen_clear(g: yajl_gen);
    fn yajl_alloc(
        callbacks: *const yajl_callbacks,
        afs: *mut yajl_alloc_funcs,
        ctx: *mut libc::c_void,
    ) -> yajl_handle;

    fn yajl_free(handle: yajl_handle);

    #[cfg(feature = "nightly")]
    fn yajl_config(h: yajl_handle, opt: yajl_option, args: ...) -> libc::c_int;
    #[cfg(not(feature = "nightly"))]
    fn yajl_config(h: *mut yajl_handle_t, opt: yajl_option, arg: libc::c_int) -> libc::c_int;

    fn yajl_status_to_string(stat: yajl_status) -> *const libc::c_char;

    fn yajl_parse(
        hand: yajl_handle,
        jsonText: *const libc::c_uchar,
        jsonTextLen: libc::size_t,
    ) -> yajl_status;

    fn yajl_complete_parse(hand: yajl_handle) -> yajl_status;

    fn yajl_get_error(
        hand: yajl_handle,
        verbose: libc::c_int,
        jsonText: *const libc::c_uchar,
        jsonTextLen: libc::size_t,
    ) -> *mut libc::c_uchar;
    fn yajl_get_bytes_consumed(hand: yajl_handle) -> libc::size_t;
    fn yajl_free_error(hand: yajl_handle, str: *mut libc::c_uchar);
    fn yajl_tree_parse(
        input: *const libc::c_char,
        error_buffer: *mut libc::c_char,
        error_buffer_size: usize,
    ) -> yajl_val;

    fn yajl_tree_get(n: yajl_val, path: *mut *const libc::c_char, type_0: yajl_type) -> yajl_val;

    fn yajl_tree_free(v: yajl_val);
    fn yajl_version() -> libc::c_int;
}

#[repr(C)]
pub struct yajl_gen_t {
    _private: [u8; 0],
}

pub type yajl_gen = *mut yajl_gen_t;
pub type yajl_gen_option = libc::c_uint;
pub type yajl_option = libc::c_uint;
pub type yajl_gen_status = libc::c_uint;
pub const yajl_gen_invalid_string: yajl_gen_status = 7;
pub const yajl_gen_no_buf: yajl_gen_status = 6;
pub const yajl_gen_invalid_number: yajl_gen_status = 5;
pub const yajl_gen_generation_complete: yajl_gen_status = 4;
pub const yajl_gen_in_error_state: yajl_gen_status = 3;
pub const yajl_max_depth_exceeded: yajl_gen_status = 2;
pub const yajl_gen_keys_must_be_strings: yajl_gen_status = 1;
pub const yajl_gen_status_ok: yajl_gen_status = 0;
pub type yajl_status = libc::c_uint;
pub type yajl_val = *mut yajl_val_s;
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void>;
pub type yajl_free_func = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub type yajl_realloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, usize) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut libc::c_void,
}

#[repr(C)]
pub struct yajl_handle_t {
    _private: [u8; 0],
}

pub type yajl_handle = *mut yajl_handle_t;
#[repr(C)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_boolean: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int>,
    pub yajl_integer:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_longlong) -> libc::c_int>,
    pub yajl_double: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> libc::c_int>,
    pub yajl_number:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, usize) -> libc::c_int>,
    pub yajl_string:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, usize) -> libc::c_int>,
    pub yajl_start_map: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_map_key:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, usize) -> libc::c_int>,
    pub yajl_end_map: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_start_array: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_end_array: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub type yajl_type = libc::c_uint;
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
    pub string: *mut libc::c_char,
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
    pub keys: *mut *const libc::c_char,
    pub values: *mut yajl_val,
    pub len: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Number {
    pub i: libc::c_longlong,
    pub d: libc::c_double,
    pub r: *mut libc::c_char,
    pub flags: libc::c_uint,
}
