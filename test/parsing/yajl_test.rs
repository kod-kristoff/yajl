use ::libc;
extern "C" {
    pub type yajl_handle_t;
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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajlTestMemoryContext {
    pub numFrees: libc::c_uint,
    pub numMallocs: libc::c_uint,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn yajlTestFree(
    mut ctx: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
) {
    let ref mut fresh0 = (*(ctx as *mut yajlTestMemoryContext)).numFrees;
    *fresh0 = (*fresh0).wrapping_add(1);
    free(ptr);
}
unsafe extern "C" fn yajlTestMalloc(
    mut ctx: *mut libc::c_void,
    mut sz: size_t,
) -> *mut libc::c_void {
    let ref mut fresh1 = (*(ctx as *mut yajlTestMemoryContext)).numMallocs;
    *fresh1 = (*fresh1).wrapping_add(1);
    return malloc(sz);
}
unsafe extern "C" fn yajlTestRealloc(
    mut ctx: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut sz: size_t,
) -> *mut libc::c_void {
    if ptr.is_null() {
        let ref mut fresh2 = (*(ctx as *mut yajlTestMemoryContext)).numMallocs;
        *fresh2 = (*fresh2).wrapping_add(1);
    } else if sz == 0 as libc::c_int as libc::c_ulong {
        let ref mut fresh3 = (*(ctx as *mut yajlTestMemoryContext)).numFrees;
        *fresh3 = (*fresh3).wrapping_add(1);
    }
    return realloc(ptr, sz);
}
unsafe extern "C" fn test_yajl_null(mut ctx: *mut libc::c_void) -> libc::c_int {
    printf(b"null\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn test_yajl_boolean(
    mut ctx: *mut libc::c_void,
    mut boolVal: libc::c_int,
) -> libc::c_int {
    printf(
        b"bool: %s\n\0" as *const u8 as *const libc::c_char,
        if boolVal != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn test_yajl_integer(
    mut ctx: *mut libc::c_void,
    mut integerVal: libc::c_longlong,
) -> libc::c_int {
    printf(b"integer: %lld\n\0" as *const u8 as *const libc::c_char, integerVal);
    return 1 as libc::c_int;
}
unsafe extern "C" fn test_yajl_double(
    mut ctx: *mut libc::c_void,
    mut doubleVal: libc::c_double,
) -> libc::c_int {
    printf(b"double: %g\n\0" as *const u8 as *const libc::c_char, doubleVal);
    return 1 as libc::c_int;
}
unsafe extern "C" fn test_yajl_string(
    mut ctx: *mut libc::c_void,
    mut stringVal: *const libc::c_uchar,
    mut stringLen: size_t,
) -> libc::c_int {
    printf(b"string: '\0" as *const u8 as *const libc::c_char);
    fwrite(
        stringVal as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        stringLen,
        stdout,
    );
    printf(b"'\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn test_yajl_map_key(
    mut ctx: *mut libc::c_void,
    mut stringVal: *const libc::c_uchar,
    mut stringLen: size_t,
) -> libc::c_int {
    let mut str: *mut libc::c_char = malloc(
        stringLen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    *str.offset(stringLen as isize) = 0 as libc::c_int as libc::c_char;
    memcpy(str as *mut libc::c_void, stringVal as *const libc::c_void, stringLen);
    printf(b"key: '%s'\n\0" as *const u8 as *const libc::c_char, str);
    free(str as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn test_yajl_start_map(mut ctx: *mut libc::c_void) -> libc::c_int {
    printf(b"map open '{'\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn test_yajl_end_map(mut ctx: *mut libc::c_void) -> libc::c_int {
    printf(b"map close '}'\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn test_yajl_start_array(mut ctx: *mut libc::c_void) -> libc::c_int {
    printf(b"array open '['\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn test_yajl_end_array(mut ctx: *mut libc::c_void) -> libc::c_int {
    printf(b"array close ']'\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
static mut callbacks: yajl_callbacks = unsafe {
    {
        let mut init = yajl_callbacks {
            yajl_null: Some(
                test_yajl_null as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_boolean: Some(
                test_yajl_boolean
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            yajl_integer: Some(
                test_yajl_integer
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_longlong,
                    ) -> libc::c_int,
            ),
            yajl_double: Some(
                test_yajl_double
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_double,
                    ) -> libc::c_int,
            ),
            yajl_number: None,
            yajl_string: Some(
                test_yajl_string
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_uchar,
                        size_t,
                    ) -> libc::c_int,
            ),
            yajl_start_map: Some(
                test_yajl_start_map
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_map_key: Some(
                test_yajl_map_key
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_uchar,
                        size_t,
                    ) -> libc::c_int,
            ),
            yajl_end_map: Some(
                test_yajl_end_map
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_start_array: Some(
                test_yajl_start_array
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_end_array: Some(
                test_yajl_end_array
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
        };
        init
    }
};
unsafe extern "C" fn usage(mut progname: *const libc::c_char) {
    fprintf(
        stderr,
        b"usage:  %s [options]\nParse input from stdin as JSON and ouput parsing details to stdout\n   -b  set the read buffer size\n   -c  allow comments\n   -g  allow *g*arbage after valid JSON text\n   -m  allows the parser to consume multiple JSON values\n       from a single string separated by whitespace\n   -p  partial JSON documents should not cause errors\n\0"
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
    let mut fileName: *const libc::c_char = 0 as *const libc::c_char;
    static mut fileData: *mut libc::c_uchar = 0 as *const libc::c_uchar
        as *mut libc::c_uchar;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut bufSize: size_t = 2048 as libc::c_int as size_t;
    let mut stat: yajl_status = yajl_status_ok;
    let mut rd: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut memCtx: yajlTestMemoryContext = {
        let mut init = yajlTestMemoryContext {
            numFrees: 0 as libc::c_int as libc::c_uint,
            numMallocs: 0 as libc::c_int as libc::c_uint,
        };
        init
    };
    let mut allocFuncs: yajl_alloc_funcs = {
        let mut init = yajl_alloc_funcs {
            malloc: Some(
                yajlTestMalloc
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            realloc: Some(
                yajlTestRealloc
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                yajlTestFree
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
            ctx: 0 as *mut libc::c_void,
        };
        init
    };
    allocFuncs.ctx = &mut memCtx as *mut yajlTestMemoryContext as *mut libc::c_void;
    hand = yajl_alloc(&mut callbacks, &mut allocFuncs, 0 as *mut libc::c_void);
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(b"-c\0" as *const u8 as *const libc::c_char, *argv.offset(i as isize))
            == 0
        {
            yajl_config(hand, yajl_allow_comments, 1 as libc::c_int);
        } else if strcmp(
            b"-b\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        ) == 0
        {
            i += 1;
            if i >= argc {
                usage(*argv.offset(0 as libc::c_int as isize));
            }
            j = 0 as libc::c_int;
            while j < strlen(*argv.offset(i as isize)) as libc::c_int {
                if !(*(*argv.offset(i as isize)).offset(j as isize) as libc::c_int
                    <= '9' as i32
                    && *(*argv.offset(i as isize)).offset(j as isize) as libc::c_int
                        >= '0' as i32)
                {
                    fprintf(
                        stderr,
                        b"-b requires an integer argument.  '%s' is invalid\n\0"
                            as *const u8 as *const libc::c_char,
                        *argv.offset(i as isize),
                    );
                    usage(*argv.offset(0 as libc::c_int as isize));
                }
                j += 1;
            }
            bufSize = atoi(*argv.offset(i as isize)) as size_t;
            if bufSize == 0 {
                fprintf(
                    stderr,
                    b"%zu is an invalid buffer size\n\0" as *const u8
                        as *const libc::c_char,
                    bufSize,
                );
            }
        } else if strcmp(
            b"-g\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        ) == 0
        {
            yajl_config(hand, yajl_allow_trailing_garbage, 1 as libc::c_int);
        } else if strcmp(
            b"-m\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        ) == 0
        {
            yajl_config(hand, yajl_allow_multiple_values, 1 as libc::c_int);
        } else if strcmp(
            b"-p\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        ) == 0
        {
            yajl_config(hand, yajl_allow_partial_values, 1 as libc::c_int);
        } else {
            fileName = *argv.offset(i as isize);
            break;
        }
        i += 1;
    }
    fileData = malloc(bufSize) as *mut libc::c_uchar;
    if fileData.is_null() {
        fprintf(
            stderr,
            b"failed to allocate read buffer of %zu bytes, exiting.\0" as *const u8
                as *const libc::c_char,
            bufSize,
        );
        yajl_free(hand);
        exit(2 as libc::c_int);
    }
    if !fileName.is_null() {
        file = fopen(fileName, b"r\0" as *const u8 as *const libc::c_char);
    } else {
        file = stdin;
    }
    loop {
        rd = fread(
            fileData as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            bufSize,
            file,
        );
        if rd == 0 as libc::c_int as libc::c_ulong {
            if feof(stdin) == 0 {
                fprintf(
                    stderr,
                    b"error reading from '%s'\n\0" as *const u8 as *const libc::c_char,
                    fileName,
                );
            }
            break;
        } else {
            stat = yajl_parse(hand, fileData, rd);
            if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
                break;
            }
        }
    }
    stat = yajl_complete_parse(hand);
    if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
        let mut str: *mut libc::c_uchar = yajl_get_error(
            hand,
            0 as libc::c_int,
            fileData,
            rd,
        );
        fflush(stdout);
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const libc::c_char,
            str as *mut libc::c_char,
        );
        yajl_free_error(hand, str);
    }
    yajl_free(hand);
    free(fileData as *mut libc::c_void);
    if !fileName.is_null() {
        fclose(file);
    }
    fflush(stderr);
    fflush(stdout);
    printf(
        b"memory leaks:\t%u\n\0" as *const u8 as *const libc::c_char,
        (memCtx.numMallocs).wrapping_sub(memCtx.numFrees),
    );
    return 0 as libc::c_int;
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
