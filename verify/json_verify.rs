use ::libc;
extern "C" {
    pub type yajl_handle_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn yajl_alloc(
        callbacks: *const yajl_callbacks,
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
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
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
unsafe extern "C" fn usage(mut progname: *const libc::c_char) {
    fprintf(
        stderr,
        b"%s: validate json from stdin\nusage: json_verify [options]\n    -c allow comments\n    -q quiet mode\n    -s verify a stream of multiple json entities\n    -u allow invalid utf8 inside strings\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    exit(1 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut stat: yajl_status = yajl_status_ok;
    let mut rd: size_t = 0;
    let mut hand: yajl_handle = 0 as *mut yajl_handle_t;
    static mut fileData: [libc::c_uchar; 65536] = [0; 65536];
    let mut quiet: libc::c_int = 0 as libc::c_int;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = 1 as libc::c_int;
    hand = yajl_alloc(
        0 as *const yajl_callbacks,
        0 as *mut yajl_alloc_funcs,
        0 as *mut libc::c_void,
    );
    while a < argc
        && *(*argv.offset(a as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
        && strlen(*argv.offset(a as isize)) > 1 as libc::c_int as libc::c_ulong
    {
        let mut i: libc::c_uint = 0;
        i = 1 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < strlen(*argv.offset(a as isize)) {
            match *(*argv.offset(a as isize)).offset(i as isize) as libc::c_int {
                113 => {
                    quiet = 1 as libc::c_int;
                }
                99 => {
                    yajl_config(hand, yajl_allow_comments, 1 as libc::c_int);
                }
                117 => {
                    yajl_config(hand, yajl_dont_validate_strings, 1 as libc::c_int);
                }
                115 => {
                    yajl_config(hand, yajl_allow_multiple_values, 1 as libc::c_int);
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
        retval = 0 as libc::c_int;
        if rd == 0 as libc::c_int as libc::c_ulong {
            if feof(stdin) == 0 {
                if quiet == 0 {
                    fprintf(
                        stderr,
                        b"error encountered on file read\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                retval = 1 as libc::c_int;
            }
            break;
        } else {
            fileData[rd as usize] = 0 as libc::c_int as libc::c_uchar;
            stat = yajl_parse(hand, fileData.as_mut_ptr(), rd);
            if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
                break;
            }
        }
    }
    stat = yajl_complete_parse(hand);
    if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
        if quiet == 0 {
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
        }
        retval = 1 as libc::c_int;
    }
    yajl_free(hand);
    if quiet == 0 {
        printf(
            b"JSON is %s\n\0" as *const u8 as *const libc::c_char,
            if retval != 0 {
                b"invalid\0" as *const u8 as *const libc::c_char
            } else {
                b"valid\0" as *const u8 as *const libc::c_char
            },
        );
    }
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
