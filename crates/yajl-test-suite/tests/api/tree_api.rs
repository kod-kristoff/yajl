extern "C" {
    fn yajl_tree_parse(
        input: *const libc::c_char,
        error_buffer: *mut libc::c_char,
        error_buffer_size: usize,
    ) -> yajl_val;
    fn yajl_tree_get(n: yajl_val, path: *mut *const libc::c_char, type_0: yajl_type) -> yajl_val;

    fn yajl_tree_free(v: yajl_val);
}
use std::{fs, ptr};

use yajl_test_suite::{yajl_t_number, yajl_type, yajl_val};

#[test]
fn tree_parse() {
    let mut file_data: Vec<u8> = fs::read("assets/sample.config").unwrap();
    file_data.push(0);

    let mut error_buffer = [0; 1024];

    let node = unsafe {
        yajl_tree_parse(
            file_data.as_ptr() as *const i8,
            error_buffer.as_mut_ptr(),
            error_buffer.len(),
        )
    };
    assert!(!node.is_null());

    let mut path: [*const libc::c_char; 3] = [
        b"Logging\0" as *const u8 as *const libc::c_char,
        b"fileRolloverKB\0" as *const u8 as *const libc::c_char,
        ptr::null(),
    ];
    let val = unsafe { yajl_tree_get(node, path.as_mut_ptr(), yajl_t_number) };

    if val.is_null() {
        unsafe {
            yajl_tree_free(node);
        }
        assert!(!val.is_null());
    }
    let actual = unsafe { (*val).u.number.i };
    unsafe {
        yajl_tree_free(node);
    }
    let expected = 2048;
    assert_eq!(actual, expected);
}
