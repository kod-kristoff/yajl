#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::{ptr::addr_of, slice};
use std::{
    ffi::{CStr, OsStr},
    fs::File,
    io,
    os::unix::ffi::OsStrExt,
    path::Path,
    process,
};

use gpoint::GPoint;
use yajl::{
    parser::{yajl_callbacks, Parser},
    yajl_alloc::yajl_alloc_funcs,
    ParserOption, Status,
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajlTestMemoryContext {
    pub numFrees: libc::c_uint,
    pub numMallocs: libc::c_uint,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    libc::strtol(
        __nptr,
        std::ptr::null_mut::<libc::c_void>() as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int
}
unsafe extern "C" fn yajlTestFree(ctx: *mut libc::c_void, ptr: *mut libc::c_void) {
    let fresh0 = &mut (*(ctx as *mut yajlTestMemoryContext)).numFrees;
    *fresh0 = (*fresh0).wrapping_add(1);
    libc::free(ptr);
}
unsafe extern "C" fn yajlTestMalloc(ctx: *mut libc::c_void, sz: usize) -> *mut libc::c_void {
    let fresh1 = &mut (*(ctx as *mut yajlTestMemoryContext)).numMallocs;
    *fresh1 = (*fresh1).wrapping_add(1);
    libc::malloc(sz)
}
unsafe extern "C" fn yajlTestRealloc(
    ctx: *mut libc::c_void,
    ptr: *mut libc::c_void,
    sz: usize,
) -> *mut libc::c_void {
    if ptr.is_null() {
        let fresh2 = &mut (*(ctx as *mut yajlTestMemoryContext)).numMallocs;
        *fresh2 = (*fresh2).wrapping_add(1);
    } else if sz == 0 as libc::c_int as usize {
        let fresh3 = &mut (*(ctx as *mut yajlTestMemoryContext)).numFrees;
        *fresh3 = (*fresh3).wrapping_add(1);
    }
    libc::realloc(ptr, sz)
}
unsafe extern "C" fn test_yajl_null(_ctx: *mut libc::c_void) -> libc::c_int {
    println!("null");
    1 as libc::c_int
}
unsafe extern "C" fn test_yajl_boolean(
    _ctx: *mut libc::c_void,
    boolVal: libc::c_int,
) -> libc::c_int {
    println!("bool: {}", if boolVal != 0 { "true" } else { "false" },);

    1 as libc::c_int
}
unsafe extern "C" fn test_yajl_integer(
    _ctx: *mut libc::c_void,
    integerVal: libc::c_longlong,
) -> libc::c_int {
    println!("integer: {}", integerVal);

    1 as libc::c_int
}
unsafe extern "C" fn test_yajl_double(
    _ctx: *mut libc::c_void,
    doubleVal: libc::c_double,
) -> libc::c_int {
    println!("double: {}", GPoint(doubleVal));

    1 as libc::c_int
}
unsafe extern "C" fn test_yajl_string(
    _ctx: *mut libc::c_void,
    stringVal: *const libc::c_uchar,
    stringLen: usize,
) -> libc::c_int {
    let str_slice = unsafe { slice::from_raw_parts(stringVal, stringLen) };
    match core::str::from_utf8(str_slice) {
        Ok(s) => println!("string: '{}'", s),
        Err(e) => todo!("handle utf8 error: {e}"),
    }

    1 as libc::c_int
}
unsafe extern "C" fn test_yajl_map_key(
    _ctx: *mut libc::c_void,
    stringVal: *const libc::c_uchar,
    stringLen: usize,
) -> libc::c_int {
    let str_slice = unsafe { slice::from_raw_parts(stringVal, stringLen) };
    match core::str::from_utf8(str_slice) {
        Ok(s) => println!("key: '{}'", s),
        Err(e) => todo!("handle utf8 error: {e}"),
    }

    1 as libc::c_int
}
unsafe extern "C" fn test_yajl_start_map(_ctx: *mut libc::c_void) -> libc::c_int {
    println!("map open '{{'");
    1 as libc::c_int
}
unsafe extern "C" fn test_yajl_end_map(_ctx: *mut libc::c_void) -> libc::c_int {
    println!("map close '}}'");
    1 as libc::c_int
}
unsafe extern "C" fn test_yajl_start_array(_ctx: *mut libc::c_void) -> libc::c_int {
    println!("array open '['");
    1 as libc::c_int
}
unsafe extern "C" fn test_yajl_end_array(_ctx: *mut libc::c_void) -> libc::c_int {
    println!("array close ']'");
    1 as libc::c_int
}
static mut callbacks: yajl_callbacks = yajl_callbacks {
    yajl_null: Some(test_yajl_null as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
    yajl_boolean: Some(
        test_yajl_boolean as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int,
    ),
    yajl_integer: Some(
        test_yajl_integer
            as unsafe extern "C" fn(*mut libc::c_void, libc::c_longlong) -> libc::c_int,
    ),
    yajl_double: Some(
        test_yajl_double as unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> libc::c_int,
    ),
    yajl_number: None,
    yajl_string: Some(
        test_yajl_string
            as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, usize) -> libc::c_int,
    ),
    yajl_start_map: Some(
        test_yajl_start_map as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
    ),
    yajl_map_key: Some(
        test_yajl_map_key
            as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, usize) -> libc::c_int,
    ),
    yajl_end_map: Some(test_yajl_end_map as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
    yajl_start_array: Some(
        test_yajl_start_array as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
    ),
    yajl_end_array: Some(
        test_yajl_end_array as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
    ),
};
unsafe extern "C" fn usage(progname: *const libc::c_char) -> ! {
    eprintln!("usage:  {} [options]\nParse input from stdin as JSON and output parsing details to stdout\n   -b  set the read buffer size\n   -c  allow comments\n   -g  allow *g*arbage after valid JSON text\n   -m  allows the parser to consume multiple JSON values\n       from a single string separated by whitespace\n   -p  partial JSON documents should not cause errors\n",
        CStr::from_ptr(progname).to_str().unwrap());

    std::process::exit(1)
}
unsafe fn main_0(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut fileName: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut bufSize: usize = 2048;
    let mut stat;
    let mut rd: usize;
    let mut j: libc::c_int;
    let mut memCtx = yajlTestMemoryContext {
        numFrees: 0,
        numMallocs: 0,
    };
    let mut allocFuncs = yajl_alloc_funcs {
        malloc: Some(
            yajlTestMalloc as unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void,
        ),
        realloc: Some(
            yajlTestRealloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    usize,
                ) -> *mut libc::c_void,
        ),
        free: Some(
            yajlTestFree as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        ctx: std::ptr::null_mut::<libc::c_void>(),
    };
    allocFuncs.ctx = &mut memCtx as *mut yajlTestMemoryContext as *mut libc::c_void;
    let hand = Parser::alloc(
        addr_of!(callbacks),
        &mut allocFuncs,
        std::ptr::null_mut::<libc::c_void>(),
    );
    let parser = unsafe { &mut *hand };
    let mut i = 1 as libc::c_int;
    while i < argc {
        if libc::strcmp(
            b"-c\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        ) == 0
        {
            parser.config(ParserOption::AllowComments, true);
        } else if libc::strcmp(
            b"-b\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        ) == 0
        {
            i += 1;
            if i >= argc {
                usage(*argv.offset(0 as libc::c_int as isize));
            }
            j = 0 as libc::c_int;
            while j < libc::strlen(*argv.offset(i as isize)) as libc::c_int {
                if !(*(*argv.offset(i as isize)).offset(j as isize) as libc::c_int <= '9' as i32
                    && *(*argv.offset(i as isize)).offset(j as isize) as libc::c_int >= '0' as i32)
                {
                    eprintln!(
                        "-b requires an integer argument. '{}' is invalid",
                        CStr::from_ptr(*argv.offset(i as isize)).to_str().unwrap()
                    );

                    usage(*argv.offset(0 as libc::c_int as isize));
                }
                j += 1;
            }
            bufSize = atoi(*argv.offset(i as isize)) as usize;
            if bufSize == 0 {
                eprintln!("{} is an invalid buffer size", bufSize);
            }
        } else if libc::strcmp(
            b"-g\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        ) == 0
        {
            parser.config(ParserOption::AllowTrailingGarbage, true);
        } else if libc::strcmp(
            b"-m\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        ) == 0
        {
            parser.config(ParserOption::AllowMultipleValues, true);
        } else if libc::strcmp(
            b"-p\0" as *const u8 as *const libc::c_char,
            *argv.offset(i as isize),
        ) == 0
        {
            parser.config(ParserOption::AllowPartialValues, true);
        } else {
            fileName = *argv.offset(i as isize);
            break;
        }
        i += 1;
    }
    let fileData = libc::malloc(bufSize) as *mut libc::c_uchar;
    if fileData.is_null() {
        eprintln!(
            "failed to allocate read buffer of {} bytes, exiting",
            bufSize
        );
        Parser::free(hand);
        process::exit(2);
    }
    let file_data = unsafe { slice::from_raw_parts_mut(fileData, bufSize) };
    let slice = CStr::from_ptr(fileName);
    let osstr = OsStr::from_bytes(slice.to_bytes());
    let file_path: &Path = osstr.as_ref();
    let mut file: Box<dyn io::BufRead> = if !fileName.is_null() {
        let file = File::open(file_path).expect("an existing file");
        let reader = io::BufReader::new(file);
        Box::new(reader)
    } else {
        Box::new(io::stdin().lock())
    };

    loop {
        rd = match file.read(file_data) {
            Ok(rd) => rd,
            Err(err) => {
                eprintln!("error reading from '{}': {}", file_path.display(), err);
                process::exit(2);
            }
        };

        if rd == 0 as libc::c_int as usize {
            break;
        } else {
            stat = parser.parse(fileData, rd);
            if stat != Status::Ok {
                break;
            }
        }
    }
    stat = parser.complete_parse();
    if stat != Status::Ok {
        let str: *mut libc::c_uchar = parser.get_error(0 as libc::c_int, fileData, rd);

        eprint!("{}", CStr::from_ptr(str as *const i8).to_str().unwrap());
        parser.free_error(str);
    }
    Parser::free(hand);
    libc::free(fileData as *mut libc::c_void);

    println!("memory leaks:\t{}", memCtx.numMallocs - memCtx.numFrees);
    assert_eq!(memCtx.numMallocs, memCtx.numFrees, "memory leak detected");
    0 as libc::c_int
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
