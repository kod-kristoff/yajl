use std::os::raw::c_char;

/* possible data types that a yajl_val_s can hold */
#[repr(C)]
#[derive(Debug)]
pub enum yajl_type {
    yajl_t_string = 1,
    yajl_t_number = 2,
    yajl_t_object = 3,
    yajl_t_array = 4,
    yajl_t_true = 5,
    yajl_t_false = 6,
    yajl_t_null = 7,
    /** The any type isn't valid for yajl_val_s.type, but can be
     *  used as an argument to routines like yajl_tree_get().
     */
    yajl_t_any = 8,
}

// #define YAJL_NUMBER_INT_VALID    0x01
// #define YAJL_NUMBER_DOUBLE_VALID 0x02

#[repr(C)]
#[derive(Debug)]
pub struct yajl_val {
    pub r#type: yajl_type,
}

#[no_mangle]
pub extern "C" fn yajl_tree_parse(
    input: *const c_char,
    error_buffer: *mut c_char,
    error_buffer_size: usize,
) -> *mut yajl_val {
    println!("yajl_tree_parse");
    todo!()
}

#[no_mangle]
pub extern "C" fn yajl_tree_free(v: *mut yajl_val) {
    println!("yajl_tree_free");
    todo!()
}

#[no_mangle]
pub extern "C" fn yajl_tree_get(
    parent: *const yajl_val,
    path: *const *const c_char,
    r#type: yajl_type,
) -> *mut yajl_val {
    println!("yajl_tree_get");
    todo!()
}
