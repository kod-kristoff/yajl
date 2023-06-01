use ::libc;
extern "C" {
    pub type yajl_buf_t;
    pub type yajl_lexer_t;
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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub type yajl_malloc_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type yajl_free_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type yajl_realloc_func = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        size_t,
    ) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut libc::c_void,
}
pub type yajl_status = libc::c_uint;
pub const yajl_status_error: yajl_status = 2;
pub const yajl_status_client_canceled: yajl_status = 1;
pub const yajl_status_ok: yajl_status = 0;
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
    pub yajl_null: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_boolean: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int,
    >,
    pub yajl_integer: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_longlong) -> libc::c_int,
    >,
    pub yajl_double: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> libc::c_int,
    >,
    pub yajl_number: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            size_t,
        ) -> libc::c_int,
    >,
    pub yajl_string: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_uchar,
            size_t,
        ) -> libc::c_int,
    >,
    pub yajl_start_map: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_map_key: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_uchar,
            size_t,
        ) -> libc::c_int,
    >,
    pub yajl_end_map: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_start_array: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
    >,
    pub yajl_end_array: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub type yajl_handle = *mut yajl_handle_t;
pub const yajl_state_start: C2RustUnnamed = 0;
pub type yajl_option = libc::c_uint;
pub const yajl_allow_partial_values: yajl_option = 16;
pub const yajl_allow_multiple_values: yajl_option = 8;
pub const yajl_allow_trailing_garbage: yajl_option = 4;
pub const yajl_dont_validate_strings: yajl_option = 2;
pub const yajl_allow_comments: yajl_option = 1;
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
pub unsafe extern "C" fn yajl_status_to_string(
    mut stat: yajl_status,
) -> *const libc::c_char {
    let mut statStr: *const libc::c_char = b"unknown\0" as *const u8
        as *const libc::c_char;
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
#[no_mangle]
pub unsafe extern "C" fn yajl_alloc(
    mut callbacks: *const yajl_callbacks,
    mut afs: *mut yajl_alloc_funcs,
    mut ctx: *mut libc::c_void,
) -> yajl_handle {
    let mut hand: yajl_handle = 0 as yajl_handle;
    let mut afsBuffer: yajl_alloc_funcs = yajl_alloc_funcs {
        malloc: None,
        realloc: None,
        free: None,
        ctx: 0 as *mut libc::c_void,
    };
    if !afs.is_null() {
        if ((*afs).malloc).is_none() || ((*afs).realloc).is_none()
            || ((*afs).free).is_none()
        {
            return 0 as yajl_handle;
        }
    } else {
        yajl_set_default_alloc_funcs(&mut afsBuffer);
        afs = &mut afsBuffer;
    }
    hand = ((*afs).malloc)
        .expect(
            "non-null function pointer",
        )((*afs).ctx, ::core::mem::size_of::<yajl_handle_t>() as libc::c_ulong)
        as yajl_handle;
    memcpy(
        &mut (*hand).alloc as *mut yajl_alloc_funcs as *mut libc::c_void,
        afs as *mut libc::c_void,
        ::core::mem::size_of::<yajl_alloc_funcs>() as libc::c_ulong,
    );
    (*hand).callbacks = callbacks;
    (*hand).ctx = ctx;
    (*hand).lexer = 0 as yajl_lexer;
    (*hand).bytesConsumed = 0 as libc::c_int as size_t;
    (*hand).decodeBuf = yajl_buf_alloc(&mut (*hand).alloc);
    (*hand).flags = 0 as libc::c_int as libc::c_uint;
    (*hand).stateStack.stack = 0 as *mut libc::c_uchar;
    (*hand).stateStack.size = 0 as libc::c_int as size_t;
    (*hand).stateStack.used = 0 as libc::c_int as size_t;
    (*hand).stateStack.yaf = &mut (*hand).alloc;
    if ((*hand).stateStack.size).wrapping_sub((*hand).stateStack.used)
        == 0 as libc::c_int as libc::c_ulong
    {
        (*hand)
            .stateStack
            .size = ((*hand).stateStack.size as libc::c_ulong)
            .wrapping_add(128 as libc::c_int as libc::c_ulong) as size_t as size_t;
        (*hand)
            .stateStack
            .stack = ((*(*hand).stateStack.yaf).realloc)
            .expect(
                "non-null function pointer",
            )(
            (*(*hand).stateStack.yaf).ctx,
            (*hand).stateStack.stack as *mut libc::c_void,
            (*hand).stateStack.size,
        ) as *mut libc::c_uchar;
    }
    let fresh0 = (*hand).stateStack.used;
    (*hand).stateStack.used = ((*hand).stateStack.used).wrapping_add(1);
    *((*hand).stateStack.stack)
        .offset(fresh0 as isize) = yajl_state_start as libc::c_int as libc::c_uchar;
    return hand;
}
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
#[no_mangle]
pub unsafe extern "C" fn yajl_free(mut handle: yajl_handle) {
    if !((*handle).stateStack.stack).is_null() {
        ((*(*handle).stateStack.yaf).free)
            .expect(
                "non-null function pointer",
            )(
            (*(*handle).stateStack.yaf).ctx,
            (*handle).stateStack.stack as *mut libc::c_void,
        );
    }
    yajl_buf_free((*handle).decodeBuf);
    if !((*handle).lexer).is_null() {
        yajl_lex_free((*handle).lexer);
        (*handle).lexer = 0 as yajl_lexer;
    }
    ((*handle).alloc.free)
        .expect(
            "non-null function pointer",
        )((*handle).alloc.ctx, handle as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn yajl_parse(
    mut hand: yajl_handle,
    mut jsonText: *const libc::c_uchar,
    mut jsonTextLen: size_t,
) -> yajl_status {
    let mut status: yajl_status = yajl_status_ok;
    if ((*hand).lexer).is_null() {
        (*hand)
            .lexer = yajl_lex_alloc(
            &mut (*hand).alloc,
            (*hand).flags & yajl_allow_comments as libc::c_int as libc::c_uint,
            ((*hand).flags & yajl_dont_validate_strings as libc::c_int as libc::c_uint
                == 0) as libc::c_int as libc::c_uint,
        );
    }
    status = yajl_do_parse(hand, jsonText, jsonTextLen);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn yajl_complete_parse(mut hand: yajl_handle) -> yajl_status {
    if ((*hand).lexer).is_null() {
        (*hand)
            .lexer = yajl_lex_alloc(
            &mut (*hand).alloc,
            (*hand).flags & yajl_allow_comments as libc::c_int as libc::c_uint,
            ((*hand).flags & yajl_dont_validate_strings as libc::c_int as libc::c_uint
                == 0) as libc::c_int as libc::c_uint,
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
        return 0 as libc::c_int as size_t
    } else {
        return (*hand).bytesConsumed
    };
}
#[no_mangle]
pub unsafe extern "C" fn yajl_free_error(
    mut hand: yajl_handle,
    mut str: *mut libc::c_uchar,
) {
    ((*hand).alloc.free)
        .expect(
            "non-null function pointer",
        )((*hand).alloc.ctx, str as *mut libc::c_void);
}
