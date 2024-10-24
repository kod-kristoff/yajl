use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn yajl_tree_parse(
        input: *const libc::c_char,
        error_buffer: *mut libc::c_char,
        error_buffer_size: size_t,
    ) -> yajl_val;
    fn yajl_tree_free(v: yajl_val);
    fn yajl_tree_get(
        parent: yajl_val,
        path: *mut *const libc::c_char,
        type_0: yajl_type,
    ) -> yajl_val;
}
pub type size_t = libc::c_ulong;
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
pub type yajl_type = libc::c_uint;
pub const yajl_t_any: yajl_type = 8;
pub const yajl_t_null: yajl_type = 7;
pub const yajl_t_false: yajl_type = 6;
pub const yajl_t_true: yajl_type = 5;
pub const yajl_t_array: yajl_type = 4;
pub const yajl_t_object: yajl_type = 3;
pub const yajl_t_number: yajl_type = 2;
pub const yajl_t_string: yajl_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_val_s {
    pub type_0: yajl_type,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub string: *mut libc::c_char,
    pub number: C2RustUnnamed_2,
    pub object: C2RustUnnamed_1,
    pub array: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub values: *mut yajl_val,
    pub len: size_t,
}
pub type yajl_val = *mut yajl_val_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub keys: *mut *const libc::c_char,
    pub values: *mut yajl_val,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub i: libc::c_longlong,
    pub d: libc::c_double,
    pub r: *mut libc::c_char,
    pub flags: libc::c_uint,
}
static mut fileData: [libc::c_uchar; 65536] = [0; 65536];
unsafe fn main_0() -> libc::c_int {
    let mut rd: size_t = 0;
    let mut node: yajl_val = 0 as *mut yajl_val_s;
    let mut errbuf: [libc::c_char; 1024] = [0; 1024];
    errbuf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    fileData[0 as libc::c_int
        as usize] = errbuf[0 as libc::c_int as usize] as libc::c_uchar;
    rd = fread(
        fileData.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        (::core::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        stdin,
    );
    if rd == 0 as libc::c_int as libc::c_ulong && feof(stdin) == 0 {
        fprintf(
            stderr,
            b"error encountered on file read\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    } else {
        if rd
            >= (::core::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            fprintf(
                stderr,
                b"config file too big\n\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    node = yajl_tree_parse(
        fileData.as_mut_ptr() as *const libc::c_char,
        errbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    if node.is_null() {
        fprintf(stderr, b"parse_error: \0" as *const u8 as *const libc::c_char);
        if strlen(errbuf.as_mut_ptr()) != 0 {
            fprintf(
                stderr,
                b" %s\0" as *const u8 as *const libc::c_char,
                errbuf.as_mut_ptr(),
            );
        } else {
            fprintf(stderr, b"unknown error\0" as *const u8 as *const libc::c_char);
        }
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let mut path: [*const libc::c_char; 3] = [
        b"Logging\0" as *const u8 as *const libc::c_char,
        b"timeFormat\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut v: yajl_val = yajl_tree_get(node, path.as_mut_ptr(), yajl_t_string);
    if !v.is_null() {
        printf(
            b"%s/%s: %s\n\0" as *const u8 as *const libc::c_char,
            path[0 as libc::c_int as usize],
            path[1 as libc::c_int as usize],
            if !v.is_null()
                && (*v).type_0 as libc::c_uint
                    == yajl_t_string as libc::c_int as libc::c_uint
            {
                (*v).u.string
            } else {
                0 as *mut libc::c_char
            },
        );
    } else {
        printf(
            b"no such node: %s/%s\n\0" as *const u8 as *const libc::c_char,
            path[0 as libc::c_int as usize],
            path[1 as libc::c_int as usize],
        );
    }
    yajl_tree_free(node);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
