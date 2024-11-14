#![allow(clippy::missing_safety_doc)]
use yajl::{
    yajl_alloc::yajl_alloc_funcs,
    yajl_lex::yajl_lex_alloc,
    yajl_option::{yajl_allow_comments, yajl_dont_validate_strings, yajl_option},
    yajl_parser::{yajl_callbacks, yajl_handle_t, yajl_render_error_string},
    yajl_status::{yajl_status, yajl_status_ok},
};

type yajl_handle = *mut yajl::yajl_parser::yajl_handle_t;

/// allocate a parser handle
///
/// # Arguments
///
/// * `callbacks` - a yajl callbacks structure specifying the
///                    functions to call when different JSON entities
///                    are encountered in the input text.  May be NULL,
///                    which is only useful for validation.
/// * `afs` - memory allocation functions, may be NULL for to use
///                    C runtime library routines (malloc and friends)
/// * `ctx` - a context pointer that will be passed to callbacks.
///
/// # Safety
///
/// The caller is responsible for free the handle by calling `yajl_free`
#[no_mangle]
pub unsafe extern "C" fn yajl_alloc(
    mut callbacks: *const yajl_callbacks,
    mut afs: *mut yajl_alloc_funcs,
    mut ctx: *mut libc::c_void,
) -> yajl_handle {
    yajl_handle_t::alloc(callbacks, afs, ctx)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_free(mut handle: yajl_handle) {
    yajl_handle_t::free(handle)
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
    rv
}
#[cfg(not(feature = "nightly"))]
#[no_mangle]
pub unsafe extern "C" fn yajl_config(
    mut h: *mut yajl_handle_t,
    mut opt: yajl_option,
    mut arg: libc::c_int,
) -> libc::c_int {
    let h = unsafe { &mut *h };
    h.config(opt, arg)
}

#[no_mangle]
pub extern "C" fn yajl_status_to_string(mut stat: yajl_status) -> *const libc::c_char {
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

#[no_mangle]
pub unsafe extern "C" fn yajl_parse(
    mut hand: yajl_handle,
    mut jsonText: *const libc::c_uchar,
    mut jsonTextLen: libc::size_t,
) -> yajl_status {
    let hand = unsafe { &mut *hand };
    hand.parse(jsonText, jsonTextLen)
}
#[no_mangle]
pub unsafe extern "C" fn yajl_complete_parse(mut hand: yajl_handle) -> yajl_status {
    let hand = unsafe { &mut *hand };
    hand.complete_parse()
}
#[no_mangle]
pub unsafe extern "C" fn yajl_get_error(
    mut hand: yajl_handle,
    mut verbose: libc::c_int,
    mut jsonText: *const libc::c_uchar,
    mut jsonTextLen: libc::size_t,
) -> *mut libc::c_uchar {
    yajl_render_error_string(hand, jsonText, jsonTextLen, verbose)
}
#[no_mangle]
pub unsafe extern "C" fn yajl_get_bytes_consumed(mut hand: yajl_handle) -> libc::size_t {
    if hand.is_null() {
        0 as libc::c_int as libc::size_t
    } else {
        (*hand).bytesConsumed
    }
}
#[no_mangle]
pub unsafe extern "C" fn yajl_free_error(mut hand: yajl_handle, mut str: *mut libc::c_uchar) {
    ((*hand).alloc.free).expect("non-null function pointer")(
        (*hand).alloc.ctx,
        str as *mut libc::c_void,
    );
}
