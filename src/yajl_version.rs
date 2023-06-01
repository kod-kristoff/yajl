use ::libc;
#[no_mangle]
pub unsafe extern "C" fn yajl_version() -> libc::c_int {
    return 2 as libc::c_int * 10000 as libc::c_int
        + 1 as libc::c_int * 100 as libc::c_int + 1 as libc::c_int;
}
