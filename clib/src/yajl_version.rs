use ::libc;
#[no_mangle]
pub extern "C" fn yajl_version() -> libc::c_int {
    yajl::yajl_version::yajl_version() as libc::c_int
}
