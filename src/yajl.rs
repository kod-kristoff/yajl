use ::libc;

use crate::{
    yajl_buf::yajl_buf_t,
    yajl_lex::yajl_lexer_t,
    yajl_option::{yajl_allow_comments, yajl_dont_validate_strings, yajl_option},
};
extern "C" {
    // pub type yajl_buf_t;
    // pub type yajl_lexer_t;
    fn yajl_lex_alloc(
        alloc: *mut yajl_alloc_funcs,
        allowComments: libc::c_uint,
        validateUTF8: libc::c_uint,
    ) -> yajl_lexer;
    fn yajl_lex_free(lexer: yajl_lexer);
    fn yajl_render_error_string(
        hand: yajl_handle,
        jsonText: *const libc::c_uchar,
        jsonTextLen: size_t,
        verbose: libc::c_int,
    ) -> *mut libc::c_uchar;
    fn yajl_do_finish(handle: yajl_handle) -> yajl_status;
    fn yajl_do_parse(
        handle: yajl_handle,
        jsonText: *const libc::c_uchar,
        jsonTextLen: size_t,
    ) -> yajl_status;
    fn yajl_buf_free(buf: yajl_buf);
    fn yajl_set_default_alloc_funcs(yaf: *mut yajl_alloc_funcs);
    fn yajl_buf_alloc(alloc: *mut yajl_alloc_funcs) -> yajl_buf;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void>;
pub type yajl_free_func = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub type yajl_realloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_handle_t {
    pub callbacks: *const yajl_callbacks,
    pub ctx: *mut libc::c_void,
    pub lexer: yajl_lexer,
    pub parseError: *const libc::c_char,
    pub bytesConsumed: size_t,
    pub decodeBuf: yajl_buf,
    pub stateStack: yajl_bytestack,
    pub alloc: yajl_alloc_funcs,
    pub flags: libc::c_uint,
}
pub type yajl_bytestack = yajl_bytestack_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_bytestack_t {
    pub stack: *mut libc::c_uchar,
    pub size: size_t,
    pub used: size_t,
    pub yaf: *mut yajl_alloc_funcs,
}
pub type yajl_buf = *mut yajl_buf_t;
pub type yajl_lexer = *mut yajl_lexer_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_boolean: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int>,
    pub yajl_integer:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_longlong) -> libc::c_int>,
    pub yajl_double: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> libc::c_int>,
    pub yajl_number:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, size_t) -> libc::c_int>,
    pub yajl_string: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, size_t) -> libc::c_int,
    >,
    pub yajl_start_map: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_map_key: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, size_t) -> libc::c_int,
    >,
    pub yajl_end_map: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_start_array: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_end_array: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub type yajl_handle = *mut yajl_handle_t;
pub const yajl_state_start: C2RustUnnamed = 0;

pub type va_list = __builtin_va_list;
pub type C2RustUnnamed = libc::c_uint;
pub const yajl_state_got_value: C2RustUnnamed = 12;
pub const yajl_state_array_need_val: C2RustUnnamed = 11;
pub const yajl_state_array_got_val: C2RustUnnamed = 10;
pub const yajl_state_array_start: C2RustUnnamed = 9;
pub const yajl_state_map_need_key: C2RustUnnamed = 8;
pub const yajl_state_map_got_val: C2RustUnnamed = 7;
pub const yajl_state_map_need_val: C2RustUnnamed = 6;
pub const yajl_state_map_sep: C2RustUnnamed = 5;
pub const yajl_state_map_start: C2RustUnnamed = 4;
pub const yajl_state_lexical_error: C2RustUnnamed = 3;
pub const yajl_state_parse_error: C2RustUnnamed = 2;
pub const yajl_state_parse_complete: C2RustUnnamed = 1;
#[no_mangle]
pub unsafe extern "C" fn yajl_status_to_string(mut stat: yajl_status) -> *const libc::c_char {
    let mut statStr: *const libc::c_char = b"unknown\0" as *const u8 as *const libc::c_char;
    match stat as libc::c_uint {
        0 => {
            statStr = b"ok, no error\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            statStr = b"client canceled parse\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            statStr = b"parse error\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    return statStr;
}

#[cfg(feature = "nightly")]
#[no_mangle]
pub unsafe extern "C" fn yajl_config(
    mut h: yajl_handle,
    mut opt: yajl_option,
    mut args: ...
) -> libc::c_int {
    let mut rv: libc::c_int = 1 as libc::c_int;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    match opt as libc::c_uint {
        1 | 2 | 4 | 8 | 16 => {
            if ap.arg::<libc::c_int>() != 0 {
                (*h).flags |= opt as libc::c_uint;
            } else {
                (*h).flags &= !(opt as libc::c_uint);
            }
        }
        _ => {
            rv = 0 as libc::c_int;
        }
    }
    return rv;
}
#[cfg(not(feature = "nightly"))]
#[no_mangle]
pub unsafe extern "C" fn yajl_config(
    mut h: yajl_handle,
    mut opt: yajl_option,
    mut arg: libc::c_int,
) -> libc::c_int {
    let mut rv: libc::c_int = 1 as libc::c_int;
    // let mut ap: ::core::ffi::VaListImpl;
    // ap = args.clone();
    match opt as libc::c_uint {
        1 | 2 | 4 | 8 | 16 => {
            if arg != 0 {
                (*h).flags |= opt as libc::c_uint;
            } else {
                (*h).flags &= !(opt as libc::c_uint);
            }
        }
        _ => {
            rv = 0 as libc::c_int;
        }
    }
    return rv;
}

#[no_mangle]
pub unsafe extern "C" fn yajl_parse(
    mut hand: yajl_handle,
    mut jsonText: *const libc::c_uchar,
    mut jsonTextLen: size_t,
) -> yajl_status {
    let mut status: yajl_status = yajl_status_ok;
    if ((*hand).lexer).is_null() {
        (*hand).lexer = yajl_lex_alloc(
            &mut (*hand).alloc,
            (*hand).flags & yajl_allow_comments as libc::c_int as libc::c_uint,
            ((*hand).flags & yajl_dont_validate_strings as libc::c_int as libc::c_uint == 0)
                as libc::c_int as libc::c_uint,
        );
    }
    status = yajl_do_parse(hand, jsonText, jsonTextLen);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn yajl_complete_parse(mut hand: yajl_handle) -> yajl_status {
    if ((*hand).lexer).is_null() {
        (*hand).lexer = yajl_lex_alloc(
            &mut (*hand).alloc,
            (*hand).flags & yajl_allow_comments as libc::c_int as libc::c_uint,
            ((*hand).flags & yajl_dont_validate_strings as libc::c_int as libc::c_uint == 0)
                as libc::c_int as libc::c_uint,
        );
    }
    return yajl_do_finish(hand);
}
#[no_mangle]
pub unsafe extern "C" fn yajl_get_error(
    mut hand: yajl_handle,
    mut verbose: libc::c_int,
    mut jsonText: *const libc::c_uchar,
    mut jsonTextLen: size_t,
) -> *mut libc::c_uchar {
    return yajl_render_error_string(hand, jsonText, jsonTextLen, verbose);
}
#[no_mangle]
pub unsafe extern "C" fn yajl_get_bytes_consumed(mut hand: yajl_handle) -> size_t {
    if hand.is_null() {
        return 0 as libc::c_int as size_t;
    } else {
        return (*hand).bytesConsumed;
    };
}
#[no_mangle]
pub unsafe extern "C" fn yajl_free_error(mut hand: yajl_handle, mut str: *mut libc::c_uchar) {
    ((*hand).alloc.free).expect("non-null function pointer")(
        (*hand).alloc.ctx,
        str as *mut libc::c_void,
    );
}
