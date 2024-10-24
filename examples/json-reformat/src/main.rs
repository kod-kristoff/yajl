use ::libc;
extern "C" {
    pub type yajl_handle_t;
    pub type yajl_gen_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn yajl_alloc(
        callbacks_0: *const yajl_callbacks,
        afs: *mut yajl_alloc_funcs,
        ctx: *mut libc::c_void,
    ) -> yajl_handle;
    fn yajl_config(h: yajl_handle, opt: yajl_option, _: ...) -> libc::c_int;
    fn yajl_free(handle: yajl_handle);
    fn yajl_parse(
        hand: yajl_handle,
        jsonText: *const libc::c_uchar,
        jsonTextLength: size_t,
    ) -> yajl_status;
    fn yajl_complete_parse(hand: yajl_handle) -> yajl_status;
    fn yajl_get_error(
        hand: yajl_handle,
        verbose: libc::c_int,
        jsonText: *const libc::c_uchar,
        jsonTextLength: size_t,
    ) -> *mut libc::c_uchar;
    fn yajl_free_error(hand: yajl_handle, str: *mut libc::c_uchar);
    fn yajl_gen_config(g: yajl_gen, opt: yajl_gen_option, _: ...) -> libc::c_int;
    fn yajl_gen_alloc(allocFuncs: *const yajl_alloc_funcs) -> yajl_gen;
    fn yajl_gen_free(handle: yajl_gen);
    fn yajl_gen_number(
        hand: yajl_gen,
        num: *const libc::c_char,
        len: size_t,
    ) -> yajl_gen_status;
    fn yajl_gen_string(
        hand: yajl_gen,
        str: *const libc::c_uchar,
        len: size_t,
    ) -> yajl_gen_status;
    fn yajl_gen_null(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_bool(hand: yajl_gen, boolean: libc::c_int) -> yajl_gen_status;
    fn yajl_gen_map_open(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_map_close(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_array_open(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_array_close(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_get_buf(
        hand: yajl_gen,
        buf: *mut *const libc::c_uchar,
        len: *mut size_t,
    ) -> yajl_gen_status;
    fn yajl_gen_clear(hand: yajl_gen);
    fn yajl_gen_reset(hand: yajl_gen, sep: *const libc::c_char);
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub type yajl_handle = *mut yajl_handle_t;
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
pub type yajl_option = libc::c_uint;
pub const yajl_allow_partial_values: yajl_option = 16;
pub const yajl_allow_multiple_values: yajl_option = 8;
pub const yajl_allow_trailing_garbage: yajl_option = 4;
pub const yajl_dont_validate_strings: yajl_option = 2;
pub const yajl_allow_comments: yajl_option = 1;
pub type yajl_gen_status = libc::c_uint;
pub const yajl_gen_invalid_string: yajl_gen_status = 7;
pub const yajl_gen_no_buf: yajl_gen_status = 6;
pub const yajl_gen_invalid_number: yajl_gen_status = 5;
pub const yajl_gen_generation_complete: yajl_gen_status = 4;
pub const yajl_gen_in_error_state: yajl_gen_status = 3;
pub const yajl_max_depth_exceeded: yajl_gen_status = 2;
pub const yajl_gen_keys_must_be_strings: yajl_gen_status = 1;
pub const yajl_gen_status_ok: yajl_gen_status = 0;
pub type yajl_gen = *mut yajl_gen_t;
pub type yajl_gen_option = libc::c_uint;
pub const yajl_gen_escape_solidus: yajl_gen_option = 16;
pub const yajl_gen_validate_utf8: yajl_gen_option = 8;
pub const yajl_gen_print_callback: yajl_gen_option = 4;
pub const yajl_gen_indent_string: yajl_gen_option = 2;
pub const yajl_gen_beautify: yajl_gen_option = 1;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
static mut s_streamReformat: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn reformat_null(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut g: yajl_gen = ctx as yajl_gen;
    let mut __stat: yajl_gen_status = yajl_gen_null(g);
    if __stat as libc::c_uint
        == yajl_gen_generation_complete as libc::c_int as libc::c_uint
        && s_streamReformat != 0
    {
        yajl_gen_reset(g, b"\n\0" as *const u8 as *const libc::c_char);
        __stat = yajl_gen_null(g);
    }
    return (__stat as libc::c_uint == yajl_gen_status_ok as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn reformat_boolean(
    mut ctx: *mut libc::c_void,
    mut boolean: libc::c_int,
) -> libc::c_int {
    let mut g: yajl_gen = ctx as yajl_gen;
    let mut __stat: yajl_gen_status = yajl_gen_bool(g, boolean);
    if __stat as libc::c_uint
        == yajl_gen_generation_complete as libc::c_int as libc::c_uint
        && s_streamReformat != 0
    {
        yajl_gen_reset(g, b"\n\0" as *const u8 as *const libc::c_char);
        __stat = yajl_gen_bool(g, boolean);
    }
    return (__stat as libc::c_uint == yajl_gen_status_ok as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn reformat_number(
    mut ctx: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut l: size_t,
) -> libc::c_int {
    let mut g: yajl_gen = ctx as yajl_gen;
    let mut __stat: yajl_gen_status = yajl_gen_number(g, s, l);
    if __stat as libc::c_uint
        == yajl_gen_generation_complete as libc::c_int as libc::c_uint
        && s_streamReformat != 0
    {
        yajl_gen_reset(g, b"\n\0" as *const u8 as *const libc::c_char);
        __stat = yajl_gen_number(g, s, l);
    }
    return (__stat as libc::c_uint == yajl_gen_status_ok as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn reformat_string(
    mut ctx: *mut libc::c_void,
    mut stringVal: *const libc::c_uchar,
    mut stringLen: size_t,
) -> libc::c_int {
    let mut g: yajl_gen = ctx as yajl_gen;
    let mut __stat: yajl_gen_status = yajl_gen_string(g, stringVal, stringLen);
    if __stat as libc::c_uint
        == yajl_gen_generation_complete as libc::c_int as libc::c_uint
        && s_streamReformat != 0
    {
        yajl_gen_reset(g, b"\n\0" as *const u8 as *const libc::c_char);
        __stat = yajl_gen_string(g, stringVal, stringLen);
    }
    return (__stat as libc::c_uint == yajl_gen_status_ok as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn reformat_map_key(
    mut ctx: *mut libc::c_void,
    mut stringVal: *const libc::c_uchar,
    mut stringLen: size_t,
) -> libc::c_int {
    let mut g: yajl_gen = ctx as yajl_gen;
    let mut __stat: yajl_gen_status = yajl_gen_string(g, stringVal, stringLen);
    if __stat as libc::c_uint
        == yajl_gen_generation_complete as libc::c_int as libc::c_uint
        && s_streamReformat != 0
    {
        yajl_gen_reset(g, b"\n\0" as *const u8 as *const libc::c_char);
        __stat = yajl_gen_string(g, stringVal, stringLen);
    }
    return (__stat as libc::c_uint == yajl_gen_status_ok as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn reformat_start_map(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut g: yajl_gen = ctx as yajl_gen;
    let mut __stat: yajl_gen_status = yajl_gen_map_open(g);
    if __stat as libc::c_uint
        == yajl_gen_generation_complete as libc::c_int as libc::c_uint
        && s_streamReformat != 0
    {
        yajl_gen_reset(g, b"\n\0" as *const u8 as *const libc::c_char);
        __stat = yajl_gen_map_open(g);
    }
    return (__stat as libc::c_uint == yajl_gen_status_ok as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn reformat_end_map(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut g: yajl_gen = ctx as yajl_gen;
    let mut __stat: yajl_gen_status = yajl_gen_map_close(g);
    if __stat as libc::c_uint
        == yajl_gen_generation_complete as libc::c_int as libc::c_uint
        && s_streamReformat != 0
    {
        yajl_gen_reset(g, b"\n\0" as *const u8 as *const libc::c_char);
        __stat = yajl_gen_map_close(g);
    }
    return (__stat as libc::c_uint == yajl_gen_status_ok as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn reformat_start_array(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut g: yajl_gen = ctx as yajl_gen;
    let mut __stat: yajl_gen_status = yajl_gen_array_open(g);
    if __stat as libc::c_uint
        == yajl_gen_generation_complete as libc::c_int as libc::c_uint
        && s_streamReformat != 0
    {
        yajl_gen_reset(g, b"\n\0" as *const u8 as *const libc::c_char);
        __stat = yajl_gen_array_open(g);
    }
    return (__stat as libc::c_uint == yajl_gen_status_ok as libc::c_int as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn reformat_end_array(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut g: yajl_gen = ctx as yajl_gen;
    let mut __stat: yajl_gen_status = yajl_gen_array_close(g);
    if __stat as libc::c_uint
        == yajl_gen_generation_complete as libc::c_int as libc::c_uint
        && s_streamReformat != 0
    {
        yajl_gen_reset(g, b"\n\0" as *const u8 as *const libc::c_char);
        __stat = yajl_gen_array_close(g);
    }
    return (__stat as libc::c_uint == yajl_gen_status_ok as libc::c_int as libc::c_uint)
        as libc::c_int;
}
static mut callbacks: yajl_callbacks = unsafe {
    {
        let mut init = yajl_callbacks {
            yajl_null: Some(
                reformat_null as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_boolean: Some(
                reformat_boolean
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            yajl_integer: None,
            yajl_double: None,
            yajl_number: Some(
                reformat_number
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                    ) -> libc::c_int,
            ),
            yajl_string: Some(
                reformat_string
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_uchar,
                        size_t,
                    ) -> libc::c_int,
            ),
            yajl_start_map: Some(
                reformat_start_map
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_map_key: Some(
                reformat_map_key
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_uchar,
                        size_t,
                    ) -> libc::c_int,
            ),
            yajl_end_map: Some(
                reformat_end_map
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_start_array: Some(
                reformat_start_array
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_end_array: Some(
                reformat_end_array
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
        };
        init
    }
};
unsafe extern "C" fn usage(mut progname: *const libc::c_char) {
    fprintf(
        stderr,
        b"%s: reformat json from stdin\nusage:  json_reformat [options]\n    -e escape any forward slashes (for embedding in HTML)\n    -m minimize json rather than beautify (default)\n    -s reformat a stream of multiple json entites\n    -u allow invalid UTF8 inside strings during parsing\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    exit(1 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut hand: yajl_handle = 0 as *mut yajl_handle_t;
    static mut fileData: [libc::c_uchar; 65536] = [0; 65536];
    let mut g: yajl_gen = 0 as *mut yajl_gen_t;
    let mut stat: yajl_status = yajl_status_ok;
    let mut rd: size_t = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = 1 as libc::c_int;
    g = yajl_gen_alloc(0 as *const yajl_alloc_funcs);
    yajl_gen_config(g, yajl_gen_beautify, 1 as libc::c_int);
    yajl_gen_config(g, yajl_gen_validate_utf8, 1 as libc::c_int);
    hand = yajl_alloc(
        &mut callbacks,
        0 as *mut yajl_alloc_funcs,
        g as *mut libc::c_void,
    );
    yajl_config(hand, yajl_allow_comments, 1 as libc::c_int);
    while a < argc
        && *(*argv.offset(a as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
        && strlen(*argv.offset(a as isize)) > 1 as libc::c_int as libc::c_ulong
    {
        let mut i: libc::c_uint = 0;
        i = 1 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < strlen(*argv.offset(a as isize)) {
            match *(*argv.offset(a as isize)).offset(i as isize) as libc::c_int {
                109 => {
                    yajl_gen_config(g, yajl_gen_beautify, 0 as libc::c_int);
                }
                115 => {
                    yajl_config(hand, yajl_allow_multiple_values, 1 as libc::c_int);
                    s_streamReformat = 1 as libc::c_int;
                }
                117 => {
                    yajl_config(hand, yajl_dont_validate_strings, 1 as libc::c_int);
                }
                101 => {
                    yajl_gen_config(g, yajl_gen_escape_solidus, 1 as libc::c_int);
                }
                _ => {
                    fprintf(
                        stderr,
                        b"unrecognized option: '%c'\n\n\0" as *const u8
                            as *const libc::c_char,
                        *(*argv.offset(a as isize)).offset(i as isize) as libc::c_int,
                    );
                    usage(*argv.offset(0 as libc::c_int as isize));
                }
            }
            i = i.wrapping_add(1);
        }
        a += 1;
    }
    if a < argc {
        usage(*argv.offset(0 as libc::c_int as isize));
    }
    loop {
        rd = fread(
            fileData.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (::core::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            stdin,
        );
        if rd == 0 as libc::c_int as libc::c_ulong {
            if feof(stdin) == 0 {
                fprintf(
                    stderr,
                    b"error on file read.\n\0" as *const u8 as *const libc::c_char,
                );
                retval = 1 as libc::c_int;
            }
            break;
        } else {
            fileData[rd as usize] = 0 as libc::c_int as libc::c_uchar;
            stat = yajl_parse(hand, fileData.as_mut_ptr(), rd);
            if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
                break;
            }
            let mut buf: *const libc::c_uchar = 0 as *const libc::c_uchar;
            let mut len: size_t = 0;
            yajl_gen_get_buf(g, &mut buf, &mut len);
            fwrite(
                buf as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                len,
                stdout,
            );
            yajl_gen_clear(g);
        }
    }
    stat = yajl_complete_parse(hand);
    if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
        let mut str: *mut libc::c_uchar = yajl_get_error(
            hand,
            1 as libc::c_int,
            fileData.as_mut_ptr(),
            rd,
        );
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const libc::c_char,
            str as *const libc::c_char,
        );
        yajl_free_error(hand, str);
        retval = 1 as libc::c_int;
    }
    yajl_gen_free(g);
    yajl_free(hand);
    return retval;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
