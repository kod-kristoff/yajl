use ::libc;

use crate::{
    yajl_alloc::yajl_alloc_funcs,
    yajl_buf::yajl_buf_t,
    yajl_lex::{yajl_lex_alloc, yajl_lexer_t},
    yajl_option::{yajl_allow_comments, yajl_dont_validate_strings, yajl_option},
    yajl_parser::{yajl_handle_t, yajl_render_error_string},
    yajl_status::{yajl_status, yajl_status_ok},
};

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void>;
pub type yajl_free_func = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub type yajl_realloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, usize) -> *mut libc::c_void>;

pub type yajl_bytestack = yajl_bytestack_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_bytestack_t {
    pub stack: *mut libc::c_uchar,
    pub size: usize,
    pub used: usize,
    pub yaf: *mut yajl_alloc_funcs,
}
pub type yajl_buf = *mut yajl_buf_t;
pub type yajl_lexer = *mut yajl_lexer_t;

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
    statStr
}

#[cfg(feature = "nightly")]

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
    rv
}
impl yajl_handle_t {
    pub unsafe fn parse(
        &mut self,
        // mut hand: yajl_handle,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
    ) -> yajl_status {
        if (self.lexer).is_null() {
            self.lexer = yajl_lex_alloc(
                &mut self.alloc,
                self.flags & yajl_allow_comments as libc::c_int as libc::c_uint,
                (self.flags & yajl_dont_validate_strings as libc::c_int as libc::c_uint == 0)
                    as libc::c_int as libc::c_uint,
            );
        }
        self.do_parse(jsonText, jsonTextLen)
    }

    pub unsafe fn complete_parse(&mut self) -> yajl_status {
        if (self.lexer).is_null() {
            self.lexer = yajl_lex_alloc(
                &mut self.alloc,
                self.flags & yajl_allow_comments as libc::c_int as libc::c_uint,
                (self.flags & yajl_dont_validate_strings as libc::c_int as libc::c_uint == 0)
                    as libc::c_int as libc::c_uint,
            );
        }
        self.do_finish()
    }
}
pub unsafe extern "C" fn yajl_get_error(
    mut hand: yajl_handle,
    mut verbose: libc::c_int,
    mut jsonText: *const libc::c_uchar,
    mut jsonTextLen: usize,
) -> *mut libc::c_uchar {
    yajl_render_error_string(hand, jsonText, jsonTextLen, verbose)
}
pub unsafe extern "C" fn yajl_get_bytes_consumed(mut hand: yajl_handle) -> usize {
    if hand.is_null() {
        0 as libc::c_int as usize
    } else {
        (*hand).bytesConsumed
    }
}
pub unsafe extern "C" fn yajl_free_error(mut hand: yajl_handle, mut str: *mut libc::c_uchar) {
    ((*hand).alloc.free).expect("non-null function pointer")(
        (*hand).alloc.ctx,
        str as *mut libc::c_void,
    );
}
