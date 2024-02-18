use ::libc;
extern "C" {
    pub type yajl_gen_t;
    fn yajl_gen_alloc(allocFuncs: *const yajl_alloc_funcs) -> yajl_gen;
    fn yajl_gen_map_open(hand: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_map_close(hand: yajl_gen) -> yajl_gen_status;
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
unsafe fn main_0() -> libc::c_int {
    let mut yg: yajl_gen = 0 as *mut yajl_gen_t;
    let mut s: yajl_gen_status = yajl_gen_status_ok;
    yg = yajl_gen_alloc(0 as *const yajl_alloc_funcs);
    if yajl_gen_map_open(yg) as libc::c_uint
        != yajl_gen_status_ok as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if yajl_gen_map_close(yg) as libc::c_uint
        != yajl_gen_status_ok as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    s = yajl_gen_map_close(yg);
    return (yajl_gen_generation_complete as libc::c_int as libc::c_uint
        == s as libc::c_uint) as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
