extern "C" {
    fn yajl_gen_alloc(afs: *const yajl_alloc_funcs) -> yajl_gen;
    fn yajl_gen_map_open(g: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_map_close(g: yajl_gen) -> yajl_gen_status;
    fn yajl_gen_free(g: yajl_gen);

}

use yajl_ffi_test_suite::{
    yajl_alloc_funcs, yajl_gen, yajl_gen_generation_complete, yajl_gen_status, yajl_gen_status_ok,
};

#[test]
fn gen_extra_close() {
    let yg = unsafe { yajl_gen_alloc(std::ptr::null()) };
    assert_eq!(unsafe { yajl_gen_map_open(yg) }, yajl_gen_status_ok);
    assert_eq!(unsafe { yajl_gen_map_close(yg) }, yajl_gen_status_ok);

    let s = unsafe { yajl_gen_map_close(yg) };
    assert_eq!(yajl_gen_generation_complete, s);
    unsafe { yajl_gen_free(yg) };
}
