use core::{
    ffi::{c_char, c_double, c_int, c_longlong, c_uchar, c_uint, c_void},
    ptr,
};

use crate::{
    yajl_alloc::{yajl_alloc_funcs, yajl_set_default_alloc_funcs},
    yajl_buf::{
        yajl_buf_alloc, yajl_buf_append, yajl_buf_clear, yajl_buf_data, yajl_buf_free,
        yajl_buf_len, yajl_buf_t,
    },
    yajl_encode::{yajl_string_encode, yajl_string_validate_utf8},
};

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}
pub type yajl_malloc_func = Option<unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void>;
pub type yajl_free_func = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> ()>;
pub type yajl_realloc_func =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> *mut c_void>;

pub type yajl_gen_status = c_uint;
pub const yajl_gen_invalid_string: yajl_gen_status = 7;
pub const yajl_gen_no_buf: yajl_gen_status = 6;
pub const yajl_gen_invalid_number: yajl_gen_status = 5;
pub const yajl_gen_generation_complete: yajl_gen_status = 4;
pub const yajl_gen_in_error_state: yajl_gen_status = 3;
pub const yajl_max_depth_exceeded: yajl_gen_status = 2;
pub const yajl_gen_keys_must_be_strings: yajl_gen_status = 1;
pub const yajl_gen_status_ok: yajl_gen_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_gen_t {
    pub flags: c_uint,
    pub depth: c_uint,
    pub indentString: *const c_char,
    pub state: [yajl_gen_state; 128],
    pub print: yajl_print_t,
    pub ctx: *mut c_void,
    pub alloc: yajl_alloc_funcs,
}
pub type yajl_print_t = Option<unsafe extern "C" fn(*mut c_void, *const c_char, usize) -> ()>;
pub type yajl_gen_state = c_uint;
pub const yajl_gen_error: yajl_gen_state = 7;
pub const yajl_gen_complete: yajl_gen_state = 6;
pub const yajl_gen_in_array: yajl_gen_state = 5;
pub const yajl_gen_array_start: yajl_gen_state = 4;
pub const yajl_gen_map_val: yajl_gen_state = 3;
pub const yajl_gen_map_key: yajl_gen_state = 2;
pub const yajl_gen_map_start: yajl_gen_state = 1;
pub const yajl_gen_start: yajl_gen_state = 0;
pub type yajl_gen = *mut yajl_gen_t;
pub type yajl_gen_option = c_uint;
pub const yajl_gen_escape_solidus: yajl_gen_option = 16;
pub const yajl_gen_validate_utf8: yajl_gen_option = 8;
pub const yajl_gen_print_callback: yajl_gen_option = 4;
pub const yajl_gen_indent_string: yajl_gen_option = 2;
pub const yajl_gen_beautify: yajl_gen_option = 1;
pub type va_list = __builtin_va_list;
pub type yajl_buf = *mut yajl_buf_t;
#[cfg(feature = "nightly")]
pub unsafe extern "C" fn yajl_gen_config(
    mut g: yajl_gen,
    mut opt: yajl_gen_option,
    mut args: ...
) -> c_int {
    let mut rv: c_int = 1 as c_int;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    match opt as c_uint {
        1 | 8 | 16 => {
            if ap.arg::<c_int>() != 0 {
                (*g).flags |= opt as c_uint;
            } else {
                (*g).flags &= !(opt as c_uint);
            }
        }
        2 => {
            let mut indent: *const c_char = ap.arg::<*const c_char>();
            (*g).indentString = indent;
            while *indent != 0 {
                if *indent as c_int != '\n' as i32
                    && *indent as c_int != '\u{b}' as i32
                    && *indent as c_int != '\u{c}' as i32
                    && *indent as c_int != '\t' as i32
                    && *indent as c_int != '\r' as i32
                    && *indent as c_int != ' ' as i32
                {
                    (*g).indentString = 0 as *const c_char;
                    rv = 0 as c_int;
                }
                indent = indent.offset(1);
            }
        }
        4 => {
            yajl_buf_free((*g).ctx as yajl_buf);
            (*g).print = ::core::mem::transmute(ap.arg::<*mut unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                usize,
            ) -> ()>());
            (*g).ctx = ap.arg::<*mut c_void>();
        }
        _ => {
            rv = 0 as c_int;
        }
    }
    return rv;
}
#[cfg(not(feature = "nightly"))]
pub unsafe extern "C" fn yajl_gen_config_set_indent(
    mut g: yajl_gen,
    mut opt: yajl_gen_option,
    mut indent: *const c_char,
) -> c_int {
    let mut rv: c_int = 1;
    match opt as c_uint {
        2 => {
            (*g).indentString = indent;
            while *indent != 0 {
                if *indent as c_int != '\n' as i32
                    && *indent as c_int != '\u{b}' as i32
                    && *indent as c_int != '\u{c}' as i32
                    && *indent as c_int != '\t' as i32
                    && *indent as c_int != '\r' as i32
                    && *indent as c_int != ' ' as i32
                {
                    (*g).indentString = ptr::null::<c_char>();
                    rv = 0 as c_int;
                }
                indent = indent.offset(1);
            }
        }
        _ => {
            rv = 0 as c_int;
        }
    }
    rv
}

#[cfg(not(feature = "nightly"))]
pub unsafe extern "C" fn yajl_gen_config(
    mut g: yajl_gen,
    mut opt: yajl_gen_option,
    mut arg: c_int,
) -> c_int {
    let mut rv: c_int = 1 as c_int;
    match opt as c_uint {
        1 | 8 | 16 => {
            if arg != 0 {
                (*g).flags |= opt as c_uint;
            } else {
                (*g).flags &= !(opt as c_uint);
            }
        }
        _ => {
            rv = 0 as c_int;
        }
    }
    rv
}
#[cfg(not(feature = "nightly"))]
pub unsafe extern "C" fn yajl_gen_config_print_callback(
    mut g: yajl_gen,
    mut opt: yajl_gen_option,
    mut arg: c_int,
    print: unsafe extern "C" fn(*mut c_void, *const c_char, usize) -> (),
    ctx: *mut c_void,
) -> c_int {
    let mut rv: c_int = 1 as c_int;
    match opt as c_uint {
        4 => {
            yajl_buf_free((*g).ctx as yajl_buf);
            (*g).print = Some(print);
            (*g).ctx = ctx;
        }
        _ => {
            rv = 0 as c_int;
        }
    }
    rv
}
pub unsafe extern "C" fn yajl_gen_alloc(mut afs: *const yajl_alloc_funcs) -> yajl_gen {
    let mut afsBuffer: yajl_alloc_funcs = yajl_alloc_funcs {
        malloc: None,
        realloc: None,
        free: None,
        ctx: ptr::null_mut::<c_void>(),
    };
    if !afs.is_null() {
        if ((*afs).malloc).is_none() || ((*afs).realloc).is_none() || ((*afs).free).is_none() {
            return 0 as yajl_gen;
        }
    } else {
        yajl_set_default_alloc_funcs(&mut afsBuffer);
        afs = &mut afsBuffer;
    }
    let g = ((*afs).malloc).expect("non-null function pointer")(
        (*afs).ctx,
        ::core::mem::size_of::<yajl_gen_t>(),
    ) as yajl_gen;
    if g.is_null() {
        return ptr::null_mut();
    }

    ptr::write_bytes(g, 0, 1);

    libc::memcpy(
        &mut (*g).alloc as *mut yajl_alloc_funcs as *mut c_void,
        afs as *mut c_void,
        ::core::mem::size_of::<yajl_alloc_funcs>(),
    );
    (*g).print = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(yajl_buf, *const c_void, usize) -> ()>,
        yajl_print_t,
    >(Some(
        yajl_buf_append as unsafe extern "C" fn(yajl_buf, *const c_void, usize) -> (),
    ));
    (*g).ctx = yajl_buf_alloc(&mut (*g).alloc) as *mut c_void;
    (*g).indentString = b"    \0" as *const u8 as *const c_char;
    g
}
pub unsafe extern "C" fn yajl_gen_reset(mut g: yajl_gen, mut sep: *const c_char) {
    (*g).depth = 0 as c_int as c_uint;
    (*g).state.fill(0);

    if !sep.is_null() {
        ((*g).print).expect("non-null function pointer")((*g).ctx, sep, libc::strlen(sep));
    }
}
pub unsafe extern "C" fn yajl_gen_free(mut g: yajl_gen) {
    if (*g).print
        == ::core::mem::transmute::<
            Option<unsafe extern "C" fn(yajl_buf, *const c_void, usize) -> ()>,
            yajl_print_t,
        >(Some(
            yajl_buf_append as unsafe extern "C" fn(yajl_buf, *const c_void, usize) -> (),
        ))
    {
        yajl_buf_free((*g).ctx as yajl_buf);
    }
    ((*g).alloc.free).expect("non-null function pointer")((*g).alloc.ctx, g as *mut c_void);
}

pub unsafe extern "C" fn yajl_gen_integer(
    mut g: yajl_gen,
    mut number: c_longlong,
) -> yajl_gen_status {
    let mut i: [c_char; 32] = [0; 32];
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_error as c_int as c_uint {
        return yajl_gen_in_error_state;
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_start as c_int as c_uint
    {
        return yajl_gen_keys_must_be_strings;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_in_array as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b",\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b"\n\0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_val as c_int as c_uint {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b":\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b" \0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint != yajl_gen_map_val as c_int as c_uint
    {
        let mut _i: c_uint = 0;
        _i = 0 as c_int as c_uint;
        while _i < (*g).depth {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                (*g).indentString,
                libc::strlen((*g).indentString) as c_uint as usize,
            );
            _i = _i.wrapping_add(1);
        }
    }
    libc::sprintf(
        i.as_mut_ptr(),
        b"%lld\0" as *const u8 as *const c_char,
        number,
    );
    ((*g).print).expect("non-null function pointer")(
        (*g).ctx,
        i.as_mut_ptr(),
        libc::strlen(i.as_mut_ptr()) as c_uint as usize,
    );
    match (*g).state[(*g).depth as usize] as c_uint {
        0 => {
            (*g).state[(*g).depth as usize] = yajl_gen_complete;
        }
        1 | 2 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_val;
        }
        4 => {
            (*g).state[(*g).depth as usize] = yajl_gen_in_array;
        }
        3 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_key;
        }
        _ => {}
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_double(mut g: yajl_gen, mut number: c_double) -> yajl_gen_status {
    let mut i: [c_char; 32] = [0; 32];
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_error as c_int as c_uint {
        return yajl_gen_in_error_state;
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_start as c_int as c_uint
    {
        return yajl_gen_keys_must_be_strings;
    }
    if number.is_nan() as i32 != 0
        || if number.is_infinite() {
            if number.is_sign_positive() {
                1
            } else {
                -1
            }
        } else {
            0
        } != 0
    {
        return yajl_gen_invalid_number;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_in_array as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b",\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b"\n\0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_val as c_int as c_uint {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b":\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b" \0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint != yajl_gen_map_val as c_int as c_uint
    {
        let mut _i: c_uint = 0;
        _i = 0 as c_int as c_uint;
        while _i < (*g).depth {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                (*g).indentString,
                libc::strlen((*g).indentString) as c_uint as usize,
            );
            _i = _i.wrapping_add(1);
        }
    }
    libc::sprintf(
        i.as_mut_ptr(),
        b"%.20g\0" as *const u8 as *const c_char,
        number,
    );
    if libc::strspn(
        i.as_mut_ptr(),
        b"0123456789-\0" as *const u8 as *const c_char,
    ) == libc::strlen(i.as_mut_ptr())
    {
        libc::strcat(i.as_mut_ptr(), b".0\0" as *const u8 as *const c_char);
    }
    ((*g).print).expect("non-null function pointer")(
        (*g).ctx,
        i.as_mut_ptr(),
        libc::strlen(i.as_mut_ptr()),
    );
    match (*g).state[(*g).depth as usize] as c_uint {
        0 => {
            (*g).state[(*g).depth as usize] = yajl_gen_complete;
        }
        1 | 2 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_val;
        }
        4 => {
            (*g).state[(*g).depth as usize] = yajl_gen_in_array;
        }
        3 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_key;
        }
        _ => {}
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_number(
    mut g: yajl_gen,
    mut s: *const c_char,
    mut l: usize,
) -> yajl_gen_status {
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_error as c_int as c_uint {
        return yajl_gen_in_error_state;
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_start as c_int as c_uint
    {
        return yajl_gen_keys_must_be_strings;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_in_array as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b",\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b"\n\0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_val as c_int as c_uint {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b":\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b" \0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint != yajl_gen_map_val as c_int as c_uint
    {
        let mut _i: c_uint = 0;
        _i = 0 as c_int as c_uint;
        while _i < (*g).depth {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                (*g).indentString,
                libc::strlen((*g).indentString) as c_uint as usize,
            );
            _i = _i.wrapping_add(1);
        }
    }
    ((*g).print).expect("non-null function pointer")((*g).ctx, s, l);
    match (*g).state[(*g).depth as usize] as c_uint {
        0 => {
            (*g).state[(*g).depth as usize] = yajl_gen_complete;
        }
        1 | 2 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_val;
        }
        4 => {
            (*g).state[(*g).depth as usize] = yajl_gen_in_array;
        }
        3 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_key;
        }
        _ => {}
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_string(
    mut g: yajl_gen,
    mut str: *const c_uchar,
    mut len: usize,
) -> yajl_gen_status {
    if (*g).flags & yajl_gen_validate_utf8 as c_int as c_uint != 0
        && yajl_string_validate_utf8(str, len) == 0
    {
        return yajl_gen_invalid_string;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_error as c_int as c_uint {
        return yajl_gen_in_error_state;
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_in_array as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b",\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b"\n\0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_val as c_int as c_uint {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b":\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b" \0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint != yajl_gen_map_val as c_int as c_uint
    {
        let mut _i: c_uint = 0;
        _i = 0 as c_int as c_uint;
        while _i < (*g).depth {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                (*g).indentString,
                libc::strlen((*g).indentString) as c_uint as usize,
            );
            _i = _i.wrapping_add(1);
        }
    }
    ((*g).print).expect("non-null function pointer")(
        (*g).ctx,
        b"\"\0" as *const u8 as *const c_char,
        1 as c_int as usize,
    );
    yajl_string_encode(
        (*g).print,
        (*g).ctx,
        str,
        len,
        ((*g).flags & yajl_gen_escape_solidus as c_int as c_uint) as c_int,
    );
    ((*g).print).expect("non-null function pointer")(
        (*g).ctx,
        b"\"\0" as *const u8 as *const c_char,
        1 as c_int as usize,
    );
    match (*g).state[(*g).depth as usize] as c_uint {
        0 => {
            (*g).state[(*g).depth as usize] = yajl_gen_complete;
        }
        1 | 2 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_val;
        }
        4 => {
            (*g).state[(*g).depth as usize] = yajl_gen_in_array;
        }
        3 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_key;
        }
        _ => {}
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_null(mut g: yajl_gen) -> yajl_gen_status {
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_error as c_int as c_uint {
        return yajl_gen_in_error_state;
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_start as c_int as c_uint
    {
        return yajl_gen_keys_must_be_strings;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_in_array as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b",\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b"\n\0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_val as c_int as c_uint {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b":\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b" \0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint != yajl_gen_map_val as c_int as c_uint
    {
        let mut _i: c_uint = 0;
        _i = 0 as c_int as c_uint;
        while _i < (*g).depth {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                (*g).indentString,
                strlen((*g).indentString) as c_uint as usize,
            );
            _i = _i.wrapping_add(1);
        }
    }
    ((*g).print).expect("non-null function pointer")(
        (*g).ctx,
        b"null\0" as *const u8 as *const c_char,
        libc::strlen(b"null\0" as *const u8 as *const c_char),
    );
    match (*g).state[(*g).depth as usize] as c_uint {
        0 => {
            (*g).state[(*g).depth as usize] = yajl_gen_complete;
        }
        1 | 2 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_val;
        }
        4 => {
            (*g).state[(*g).depth as usize] = yajl_gen_in_array;
        }
        3 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_key;
        }
        _ => {}
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_bool(mut g: yajl_gen, mut boolean: c_int) -> yajl_gen_status {
    let mut val: *const c_char = if boolean != 0 {
        b"true\0" as *const u8 as *const c_char
    } else {
        b"false\0" as *const u8 as *const c_char
    };
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_error as c_int as c_uint {
        return yajl_gen_in_error_state;
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_start as c_int as c_uint
    {
        return yajl_gen_keys_must_be_strings;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_in_array as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b",\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b"\n\0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_val as c_int as c_uint {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b":\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b" \0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint != yajl_gen_map_val as c_int as c_uint
    {
        let mut _i: c_uint = 0;
        _i = 0 as c_int as c_uint;
        while _i < (*g).depth {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                (*g).indentString,
                libc::strlen((*g).indentString) as c_uint as usize,
            );
            _i = _i.wrapping_add(1);
        }
    }
    ((*g).print).expect("non-null function pointer")(
        (*g).ctx,
        val,
        libc::strlen(val) as c_uint as usize,
    );
    match (*g).state[(*g).depth as usize] as c_uint {
        0 => {
            (*g).state[(*g).depth as usize] = yajl_gen_complete;
        }
        1 | 2 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_val;
        }
        4 => {
            (*g).state[(*g).depth as usize] = yajl_gen_in_array;
        }
        3 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_key;
        }
        _ => {}
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_map_open(mut g: yajl_gen) -> yajl_gen_status {
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_error as c_int as c_uint {
        return yajl_gen_in_error_state;
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_start as c_int as c_uint
    {
        return yajl_gen_keys_must_be_strings;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_in_array as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b",\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b"\n\0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_val as c_int as c_uint {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b":\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b" \0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint != yajl_gen_map_val as c_int as c_uint
    {
        let mut _i: c_uint = 0;
        _i = 0 as c_int as c_uint;
        while _i < (*g).depth {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                (*g).indentString,
                libc::strlen((*g).indentString) as c_uint as usize,
            );
            _i = _i.wrapping_add(1);
        }
    }
    (*g).depth = ((*g).depth).wrapping_add(1);
    if (*g).depth >= 128 as c_int as c_uint {
        return yajl_max_depth_exceeded;
    }
    (*g).state[(*g).depth as usize] = yajl_gen_map_start;
    ((*g).print).expect("non-null function pointer")(
        (*g).ctx,
        b"{\0" as *const u8 as *const c_char,
        1 as c_int as usize,
    );
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_map_close(mut g: yajl_gen) -> yajl_gen_status {
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_error as c_int as c_uint {
        return yajl_gen_in_error_state;
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    (*g).depth = ((*g).depth).wrapping_sub(1);
    if (*g).depth >= 128 as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    match (*g).state[(*g).depth as usize] as c_uint {
        0 => {
            (*g).state[(*g).depth as usize] = yajl_gen_complete;
        }
        1 | 2 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_val;
        }
        4 => {
            (*g).state[(*g).depth as usize] = yajl_gen_in_array;
        }
        3 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_key;
        }
        _ => {}
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint != yajl_gen_map_val as c_int as c_uint
    {
        let mut _i: c_uint = 0;
        _i = 0 as c_int as c_uint;
        while _i < (*g).depth {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                (*g).indentString,
                libc::strlen((*g).indentString) as c_uint as usize,
            );
            _i = _i.wrapping_add(1);
        }
    }
    ((*g).print).expect("non-null function pointer")(
        (*g).ctx,
        b"}\0" as *const u8 as *const c_char,
        1 as c_int as usize,
    );
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_array_open(mut g: yajl_gen) -> yajl_gen_status {
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_error as c_int as c_uint {
        return yajl_gen_in_error_state;
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_start as c_int as c_uint
    {
        return yajl_gen_keys_must_be_strings;
    }
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_key as c_int as c_uint
        || (*g).state[(*g).depth as usize] as c_uint == yajl_gen_in_array as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b",\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b"\n\0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_map_val as c_int as c_uint {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b":\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
        if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                b" \0" as *const u8 as *const c_char,
                1 as c_int as usize,
            );
        }
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint != yajl_gen_map_val as c_int as c_uint
    {
        let mut _i: c_uint = 0;
        _i = 0 as c_int as c_uint;
        while _i < (*g).depth {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                (*g).indentString,
                libc::strlen((*g).indentString) as c_uint as usize,
            );
            _i = _i.wrapping_add(1);
        }
    }
    (*g).depth = ((*g).depth).wrapping_add(1);
    if (*g).depth >= 128 as c_int as c_uint {
        return yajl_max_depth_exceeded;
    }
    (*g).state[(*g).depth as usize] = yajl_gen_array_start;
    ((*g).print).expect("non-null function pointer")(
        (*g).ctx,
        b"[\0" as *const u8 as *const c_char,
        1 as c_int as usize,
    );
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_array_close(mut g: yajl_gen) -> yajl_gen_status {
    if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_error as c_int as c_uint {
        return yajl_gen_in_error_state;
    } else if (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    (*g).depth = ((*g).depth).wrapping_sub(1);
    if (*g).depth >= 128 as c_int as c_uint {
        return yajl_gen_generation_complete;
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0 {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    match (*g).state[(*g).depth as usize] as c_uint {
        0 => {
            (*g).state[(*g).depth as usize] = yajl_gen_complete;
        }
        1 | 2 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_val;
        }
        4 => {
            (*g).state[(*g).depth as usize] = yajl_gen_in_array;
        }
        3 => {
            (*g).state[(*g).depth as usize] = yajl_gen_map_key;
        }
        _ => {}
    }
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint != yajl_gen_map_val as c_int as c_uint
    {
        let mut _i: c_uint = 0;
        _i = 0 as c_int as c_uint;
        while _i < (*g).depth {
            ((*g).print).expect("non-null function pointer")(
                (*g).ctx,
                (*g).indentString,
                libc::strlen((*g).indentString) as c_uint as usize,
            );
            _i = _i.wrapping_add(1);
        }
    }
    ((*g).print).expect("non-null function pointer")(
        (*g).ctx,
        b"]\0" as *const u8 as *const c_char,
        1 as c_int as usize,
    );
    if (*g).flags & yajl_gen_beautify as c_int as c_uint != 0
        && (*g).state[(*g).depth as usize] as c_uint == yajl_gen_complete as c_int as c_uint
    {
        ((*g).print).expect("non-null function pointer")(
            (*g).ctx,
            b"\n\0" as *const u8 as *const c_char,
            1 as c_int as usize,
        );
    }
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_get_buf(
    mut g: yajl_gen,
    mut buf: *mut *const c_uchar,
    mut len: *mut usize,
) -> yajl_gen_status {
    if (*g).print
        != ::core::mem::transmute::<
            Option<unsafe extern "C" fn(yajl_buf, *const c_void, usize) -> ()>,
            yajl_print_t,
        >(Some(
            yajl_buf_append as unsafe extern "C" fn(yajl_buf, *const c_void, usize) -> (),
        ))
    {
        return yajl_gen_no_buf;
    }
    *buf = yajl_buf_data((*g).ctx as yajl_buf);
    *len = yajl_buf_len((*g).ctx as yajl_buf);
    yajl_gen_status_ok
}

pub unsafe extern "C" fn yajl_gen_clear(mut g: yajl_gen) {
    if (*g).print
        == ::core::mem::transmute::<
            Option<unsafe extern "C" fn(yajl_buf, *const c_void, usize) -> ()>,
            yajl_print_t,
        >(Some(
            yajl_buf_append as unsafe extern "C" fn(yajl_buf, *const c_void, usize) -> (),
        ))
    {
        yajl_buf_clear((*g).ctx as yajl_buf);
    }
}
