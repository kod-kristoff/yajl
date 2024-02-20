use ::libc;

use crate::yajl_alloc::yajl_alloc_funcs;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_buf_t {
    pub len: size_t,
    pub used: size_t,
    pub data: *mut libc::c_uchar,
    pub alloc: *mut yajl_alloc_funcs,
}
pub type yajl_buf = *mut yajl_buf_t;
unsafe extern "C" fn yajl_buf_ensure_available(mut buf: yajl_buf, mut want: size_t) {
    let mut need: size_t = 0;
    if ((*buf).data).is_null() {
        (*buf).len = 2048 as libc::c_int as size_t;
        (*buf).data = ((*(*buf).alloc).malloc).expect("non-null function pointer")(
            (*(*buf).alloc).ctx,
            (*buf).len,
        ) as *mut libc::c_uchar;
        *((*buf).data).offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    }
    need = (*buf).len;
    while want >= need.wrapping_sub((*buf).used) {
        need <<= 1 as libc::c_int;
    }
    if need != (*buf).len {
        (*buf).data = ((*(*buf).alloc).realloc).expect("non-null function pointer")(
            (*(*buf).alloc).ctx,
            (*buf).data as *mut libc::c_void,
            need,
        ) as *mut libc::c_uchar;
        (*buf).len = need;
    }
}
#[no_mangle]
pub unsafe extern "C" fn yajl_buf_alloc(mut alloc: *mut yajl_alloc_funcs) -> yajl_buf {
    let mut b: yajl_buf = ((*alloc).malloc).expect("non-null function pointer")(
        (*alloc).ctx,
        ::core::mem::size_of::<yajl_buf_t>() as libc::c_ulong,
    ) as yajl_buf;
    memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<yajl_buf_t>() as libc::c_ulong,
    );
    (*b).alloc = alloc;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn yajl_buf_free(mut buf: yajl_buf) {
    if !((*buf).data).is_null() {
        ((*(*buf).alloc).free).expect("non-null function pointer")(
            (*(*buf).alloc).ctx,
            (*buf).data as *mut libc::c_void,
        );
    }
    ((*(*buf).alloc).free).expect("non-null function pointer")(
        (*(*buf).alloc).ctx,
        buf as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn yajl_buf_append(
    mut buf: yajl_buf,
    mut data: *const libc::c_void,
    mut len: size_t,
) {
    yajl_buf_ensure_available(buf, len);
    if len > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            ((*buf).data).offset((*buf).used as isize) as *mut libc::c_void,
            data,
            len,
        );
        (*buf).used = ((*buf).used as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        *((*buf).data).offset((*buf).used as isize) = 0 as libc::c_int as libc::c_uchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn yajl_buf_clear(mut buf: yajl_buf) {
    (*buf).used = 0 as libc::c_int as size_t;
    if !((*buf).data).is_null() {
        *((*buf).data).offset((*buf).used as isize) = 0 as libc::c_int as libc::c_uchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn yajl_buf_data(mut buf: yajl_buf) -> *const libc::c_uchar {
    return (*buf).data;
}
#[no_mangle]
pub unsafe extern "C" fn yajl_buf_len(mut buf: yajl_buf) -> size_t {
    return (*buf).used;
}
#[no_mangle]
pub unsafe extern "C" fn yajl_buf_truncate(mut buf: yajl_buf, mut len: size_t) {
    (*buf).used = len;
}
