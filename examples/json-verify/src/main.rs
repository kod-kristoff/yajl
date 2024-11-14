use std::{
    env::args,
    io::{self, Read},
};

use ::libc;
use yajl::{
    yajl::{yajl_config, yajl_free_error, yajl_get_error, yajl_handle},
    yajl_alloc::yajl_alloc_funcs,
    yajl_parser::{yajl_callbacks, yajl_handle_t},
    yajl_status::{yajl_status, yajl_status_ok},
    yajl_tree::{yajl_allow_comments, yajl_allow_multiple_values, yajl_dont_validate_strings},
};

fn usage(progname: Option<&str>) {
    eprintln!(
        "{}: validate json from stdin",
        progname.unwrap_or("json-verify")
    );
    eprintln!(
        "\nusage: json_verify [options]\n    -c allow comments\n    -q quiet mode\n    -s verify a stream of multiple json entities\n    -u allow invalid utf8 inside strings\n"
    );
    std::process::exit(1);
}
unsafe fn main_0(_argc: libc::c_int, _argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut stat: yajl_status;
    let mut rd: usize;

    let mut filedata: [libc::c_uchar; 65536] = [0; 65536];
    let mut quiet: libc::c_int = 0 as libc::c_int;
    let mut retval: libc::c_int;
    let hand: yajl_handle = yajl_handle_t::alloc(
        std::ptr::null::<yajl_callbacks>(),
        std::ptr::null_mut::<yajl_alloc_funcs>(),
        std::ptr::null_mut::<libc::c_void>(),
    );
    let parser = unsafe { &mut *hand };
    let argv: Vec<String> = std::env::args().collect();
    for a in argv.iter().skip(1) {
        match a.as_str() {
            "-q" => quiet = 1,
            "-c" => {
                yajl_config(hand, yajl_allow_comments, 1 as libc::c_int);
            }
            "-u" => {
                yajl_config(hand, yajl_dont_validate_strings, 1 as libc::c_int);
            }
            "-s" => {
                yajl_config(hand, yajl_allow_multiple_values, 1 as libc::c_int);
            }
            c => {
                eprintln!("unrecognized option: '{c}'\n");
                usage(args().next().as_deref());
            }
        }
    }

    let mut stdin = io::stdin();
    loop {
        rd = match stdin.read(&mut filedata) {
            Ok(rd) => rd,
            Err(err) => {
                if quiet == 0 {
                    eprintln!("error encountered on file read: {err:?}");
                }
                return 1;
            }
        };
        retval = 0 as libc::c_int;
        if rd == 0 {
            break;
        } else {
            filedata[rd] = 0 as libc::c_int as libc::c_uchar;
            stat = parser.parse(filedata.as_mut_ptr(), rd);
            if stat != yajl_status_ok {
                break;
            }
        }
    }
    stat = parser.complete_parse();
    if stat != yajl_status_ok {
        if quiet == 0 {
            let str: *mut libc::c_uchar =
                yajl_get_error(hand, 1 as libc::c_int, filedata.as_mut_ptr(), rd);

            libc::write(
                libc::STDERR_FILENO,
                str as *mut libc::c_void,
                libc::strlen(str as *const libc::c_char),
            );
            // eprintln!(
            //     "{}",
            //     String::from_utf8_lossy(unsafe { &*(str as *const [u8]) })
            // );
            yajl_free_error(hand, str);
        }
        retval = 1 as libc::c_int;
    }
    yajl_handle_t::free(hand);
    if quiet == 0 {
        println!("JSON is {}", if retval != 0 { "invalid" } else { "valid" },);
    }
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
