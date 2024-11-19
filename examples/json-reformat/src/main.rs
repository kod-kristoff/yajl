use std::ptr::{addr_of, addr_of_mut};

use ::libc;
use yajl::{
    parser::yajl_handle,
    parser::{yajl_callbacks, Parser},
    yajl_alloc::yajl_alloc_funcs,
    yajl_gen::{
        yajl_gen, yajl_gen_alloc, yajl_gen_array_close, yajl_gen_array_open, yajl_gen_beautify,
        yajl_gen_bool, yajl_gen_clear, yajl_gen_config, yajl_gen_escape_solidus, yajl_gen_free,
        yajl_gen_generation_complete, yajl_gen_get_buf, yajl_gen_map_close, yajl_gen_map_open,
        yajl_gen_null, yajl_gen_number, yajl_gen_reset, yajl_gen_status, yajl_gen_status_ok,
        yajl_gen_string, yajl_gen_validate_utf8,
    },
    yajl_status::yajl_status,
    yajl_tree::{
        yajl_allow_comments, yajl_allow_multiple_values, yajl_dont_validate_strings, yajl_status_ok,
    },
};
extern "C" {

    static mut stdin: *mut libc::FILE;
    static mut stdout: *mut libc::FILE;
    static mut stderr: *mut libc::FILE;

}

static mut STREAM_REFORMAT: libc::c_int = 0 as libc::c_int;
unsafe fn with_yajl_gen<F>(ctx: *mut libc::c_void, f: F) -> libc::c_int
where
    F: Fn(yajl_gen) -> yajl_gen_status,
{
    let g: yajl_gen = ctx as yajl_gen;
    let mut stat = f(g);

    if stat as libc::c_uint == yajl_gen_generation_complete as libc::c_int as libc::c_uint
        && STREAM_REFORMAT != 0
    {
        yajl_gen_reset(g, b"\n\0" as *const u8 as *const libc::c_char);
        stat = f(g);
    }

    (stat as libc::c_uint == yajl_gen_status_ok as libc::c_int as libc::c_uint) as libc::c_int
}
unsafe extern "C" fn reformat_null(ctx: *mut libc::c_void) -> libc::c_int {
    with_yajl_gen(ctx, |g| yajl_gen_null(g))
}
unsafe extern "C" fn reformat_boolean(ctx: *mut libc::c_void, boolean: libc::c_int) -> libc::c_int {
    with_yajl_gen(ctx, |g| yajl_gen_bool(g, boolean))
}
unsafe extern "C" fn reformat_number(
    ctx: *mut libc::c_void,
    s: *const libc::c_char,
    l: usize,
) -> libc::c_int {
    with_yajl_gen(ctx, |g| yajl_gen_number(g, s, l))
}
unsafe extern "C" fn reformat_string(
    ctx: *mut libc::c_void,
    string_val: *const libc::c_uchar,
    string_len: usize,
) -> libc::c_int {
    with_yajl_gen(ctx, |g| yajl_gen_string(g, string_val, string_len))
}
unsafe extern "C" fn reformat_map_key(
    ctx: *mut libc::c_void,
    string_val: *const libc::c_uchar,
    string_len: usize,
) -> libc::c_int {
    with_yajl_gen(ctx, |g| yajl_gen_string(g, string_val, string_len))
}
unsafe extern "C" fn reformat_start_map(ctx: *mut libc::c_void) -> libc::c_int {
    with_yajl_gen(ctx, |g| yajl_gen_map_open(g))
}
unsafe extern "C" fn reformat_end_map(ctx: *mut libc::c_void) -> libc::c_int {
    with_yajl_gen(ctx, |g| yajl_gen_map_close(g))
}
unsafe extern "C" fn reformat_start_array(ctx: *mut libc::c_void) -> libc::c_int {
    with_yajl_gen(ctx, |g| yajl_gen_array_open(g))
}
unsafe extern "C" fn reformat_end_array(ctx: *mut libc::c_void) -> libc::c_int {
    with_yajl_gen(ctx, |g| yajl_gen_array_close(g))
}
unsafe extern "C" fn usage(progname: *const libc::c_char) {
    libc::fprintf(
        stderr,
        b"%s: reformat json from stdin\nusage:  json_reformat [options]\n    -e escape any forward slashes (for embedding in HTML)\n    -m minimize json rather than beautify (default)\n    -s reformat a stream of multiple json entites\n    -u allow invalid UTF8 inside strings during parsing\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    libc::exit(1 as libc::c_int);
}
unsafe fn main_0(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    static mut FILEDATA: [libc::c_uchar; 65536] = [0; 65536];

    let mut stat: yajl_status;
    let mut rd: usize;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut a: libc::c_int = 1 as libc::c_int;
    let callbacks: yajl_callbacks = {
        yajl_callbacks {
            yajl_null: Some(
                reformat_null as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_boolean: Some(
                reformat_boolean
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int,
            ),
            yajl_integer: None,
            yajl_double: None,
            yajl_number: Some(
                reformat_number
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        usize,
                    ) -> libc::c_int,
            ),
            yajl_string: Some(
                reformat_string
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_uchar,
                        usize,
                    ) -> libc::c_int,
            ),
            yajl_start_map: Some(
                reformat_start_map as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_map_key: Some(
                reformat_map_key
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_uchar,
                        usize,
                    ) -> libc::c_int,
            ),
            yajl_end_map: Some(
                reformat_end_map as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_start_array: Some(
                reformat_start_array as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            yajl_end_array: Some(
                reformat_end_array as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
        }
    };
    let g: yajl_gen = yajl_gen_alloc(std::ptr::null::<yajl_alloc_funcs>());
    yajl_gen_config(g, yajl_gen_beautify, 1 as libc::c_int);
    yajl_gen_config(g, yajl_gen_validate_utf8, 1 as libc::c_int);
    let hand: yajl_handle = Parser::alloc(
        addr_of!(callbacks),
        std::ptr::null_mut::<yajl_alloc_funcs>(),
        g as *mut libc::c_void,
    );
    let parser = unsafe { &mut *hand };
    parser.config(yajl_allow_comments, 1 as libc::c_int);
    while a < argc
        && *(*argv.offset(a as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
        && libc::strlen(*argv.offset(a as isize)) > 1
    {
        let mut i = 1;
        while (i) < libc::strlen(*argv.offset(a as isize)) {
            match *(*argv.offset(a as isize)).add(i) as libc::c_int {
                109 => {
                    yajl_gen_config(g, yajl_gen_beautify, 0 as libc::c_int);
                }
                115 => {
                    parser.config(yajl_allow_multiple_values, 1 as libc::c_int);
                    STREAM_REFORMAT = 1 as libc::c_int;
                }
                117 => {
                    parser.config(yajl_dont_validate_strings, 1 as libc::c_int);
                }
                101 => {
                    yajl_gen_config(g, yajl_gen_escape_solidus, 1 as libc::c_int);
                }
                _ => {
                    libc::fprintf(
                        stderr,
                        b"unrecognized option: '%c'\n\n\0" as *const u8 as *const libc::c_char,
                        *(*argv.offset(a as isize)).add(i) as libc::c_int,
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
        rd = libc::fread(
            FILEDATA.as_mut_ptr() as *mut libc::c_void,
            1,
            ::core::mem::size_of::<[libc::c_uchar; 65536]>().wrapping_sub(1),
            stdin,
        );
        if rd == 0 {
            if libc::feof(stdin) == 0 {
                libc::fprintf(
                    stderr,
                    b"error on file read.\n\0" as *const u8 as *const libc::c_char,
                );
                retval = 1 as libc::c_int;
            }
            break;
        } else {
            FILEDATA[rd] = 0 as libc::c_int as libc::c_uchar;
            stat = parser.parse(addr_of_mut!(FILEDATA) as *const u8, rd);
            if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
                break;
            }
            let mut buf: *const libc::c_uchar = std::ptr::null::<libc::c_uchar>();
            let mut len: usize = 0;
            yajl_gen_get_buf(g, &mut buf, &mut len);
            libc::fwrite(buf as *const libc::c_void, 1, len, stdout);
            yajl_gen_clear(g);
        }
    }
    stat = parser.complete_parse();
    if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
        let str: *mut libc::c_uchar = parser.get_error(1 as libc::c_int, FILEDATA.as_mut_ptr(), rd);
        libc::fprintf(
            stderr,
            b"%s\0" as *const u8 as *const libc::c_char,
            str as *const libc::c_char,
        );
        parser.free_error(str);
        retval = 1 as libc::c_int;
    }
    yajl_gen_free(g);
    Parser::free(hand);
    retval
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int, args.as_mut_ptr()) as i32)
    }
}
