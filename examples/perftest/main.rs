use ::libc;
extern "C" {
    pub type yajl_handle_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn yajl_get_error(
        hand: yajl_handle,
        verbose: libc::c_int,
        jsonText: *const libc::c_uchar,
        jsonTextLength: size_t,
    ) -> *mut libc::c_uchar;
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
    fn yajl_free_error(hand: yajl_handle, str: *mut libc::c_uchar);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn num_docs() -> libc::c_int;
    fn get_doc(i: libc::c_int) -> *mut *const libc::c_char;
    fn doc_size(i: libc::c_int) -> libc::c_uint;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
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
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
unsafe extern "C" fn mygettime() -> libc::c_double {
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut now, 0 as *mut libc::c_void);
    return now.tv_sec as libc::c_double + now.tv_usec as libc::c_double / 1000000.0f64;
}
unsafe extern "C" fn run(mut validate_utf8: libc::c_int) -> libc::c_int {
    let mut times: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut starttime: libc::c_double = 0.;
    starttime = mygettime();
    loop {
        let mut i: libc::c_int = 0;
        let mut now: libc::c_double = mygettime();
        if now - starttime >= 3 as libc::c_int as libc::c_double {
            break;
        }
        i = 0 as libc::c_int;
        while i < 100 as libc::c_int {
            let mut hand: yajl_handle = yajl_alloc(
                0 as *const yajl_callbacks,
                0 as *mut yajl_alloc_funcs,
                0 as *mut libc::c_void,
            );
            let mut stat: yajl_status = yajl_status_ok;
            let mut d: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
            yajl_config(
                hand,
                yajl_dont_validate_strings,
                if validate_utf8 != 0 { 0 as libc::c_int } else { 1 as libc::c_int },
            );
            d = get_doc((times % num_docs() as libc::c_longlong) as libc::c_int);
            while !(*d).is_null() {
                stat = yajl_parse(hand, *d as *mut libc::c_uchar, strlen(*d));
                if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint
                {
                    break;
                }
                d = d.offset(1);
            }
            stat = yajl_complete_parse(hand);
            if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
                let mut str: *mut libc::c_uchar = yajl_get_error(
                    hand,
                    1 as libc::c_int,
                    *d as *mut libc::c_uchar,
                    if !(*d).is_null() {
                        strlen(*d)
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    },
                );
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    str as *const libc::c_char,
                );
                yajl_free_error(hand, str);
                return 1 as libc::c_int;
            }
            yajl_free(hand);
            times += 1;
            i += 1;
        }
    }
    let mut throughput: libc::c_double = 0.;
    let mut now_0: libc::c_double = 0.;
    let mut all_units: [*const libc::c_char; 4] = [
        b"B/s\0" as *const u8 as *const libc::c_char,
        b"KB/s\0" as *const u8 as *const libc::c_char,
        b"MB/s\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_char as *const libc::c_char,
    ];
    let mut units: *mut *const libc::c_char = all_units.as_mut_ptr();
    let mut i_0: libc::c_int = 0;
    let mut avg_doc_size: libc::c_int = 0 as libc::c_int;
    now_0 = mygettime();
    i_0 = 0 as libc::c_int;
    while i_0 < num_docs() {
        avg_doc_size = (avg_doc_size as libc::c_uint).wrapping_add(doc_size(i_0))
            as libc::c_int as libc::c_int;
        i_0 += 1;
    }
    avg_doc_size /= num_docs();
    throughput = (times * avg_doc_size as libc::c_longlong) as libc::c_double
        / (now_0 - starttime);
    while !(*units.offset(1 as libc::c_int as isize)).is_null()
        && throughput > 1024 as libc::c_int as libc::c_double
    {
        throughput /= 1024 as libc::c_int as libc::c_double;
        units = units.offset(1);
    }
    printf(
        b"Parsing speed: %g %s\n\0" as *const u8 as *const libc::c_char,
        throughput,
        *units,
    );
    return 0 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut rv: libc::c_int = 0 as libc::c_int;
    printf(
        b"-- speed tests determine parsing throughput given %d different sample documents --\n\0"
            as *const u8 as *const libc::c_char,
        num_docs(),
    );
    printf(b"With UTF8 validation:\n\0" as *const u8 as *const libc::c_char);
    rv = run(1 as libc::c_int);
    if rv != 0 as libc::c_int {
        return rv;
    }
    printf(b"Without UTF8 validation:\n\0" as *const u8 as *const libc::c_char);
    rv = run(0 as libc::c_int);
    return rv;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
