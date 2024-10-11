use ::libc;

use crate::{yajl_buf::yajl_buf_t, yajl_lex::yajl_lexer_t};
extern "C" {
    // pub type yajl_buf_t;
    // pub type yajl_lexer_t;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[cfg_attr(target_os = "android", link_name = "__errno")]
    fn __errno_location() -> *mut libc::c_int;
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
    fn yajl_get_error(
        hand: yajl_handle,
        verbose: libc::c_int,
        jsonText: *const libc::c_uchar,
        jsonTextLength: size_t,
    ) -> *mut libc::c_uchar;
    fn yajl_parse_integer(number: *const libc::c_uchar, length: libc::c_uint) -> libc::c_longlong;
}
pub type size_t = libc::c_ulong;
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void>;
pub type yajl_free_func = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub type yajl_realloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut libc::c_void,
}
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
pub type context_t = context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context_s {
    pub stack: *mut stack_elem_t,
    pub root: yajl_val,
    pub errbuf: *mut libc::c_char,
    pub errbuf_size: size_t,
}
pub type stack_elem_t = stack_elem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_elem_s {
    pub key: *mut libc::c_char,
    pub value: yajl_val,
    pub next: *mut stack_elem_t,
}
pub type yajl_handle = *mut yajl_handle_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_handle_t {
    pub callbacks: *const yajl_callbacks,
    pub ctx: *mut libc::c_void,
    pub lexer: yajl_lexer,
    pub parseError: *const libc::c_char,
    pub bytesConsumed: size_t,
    pub decodeBuf: yajl_buf,
    pub stateStack: yajl_bytestack,
    pub alloc: yajl_alloc_funcs,
    pub flags: libc::c_uint,
}
pub type yajl_bytestack = yajl_bytestack_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_bytestack_t {
    pub stack: *mut libc::c_uchar,
    pub size: size_t,
    pub used: size_t,
    pub yaf: *mut yajl_alloc_funcs,
}
pub type yajl_buf = *mut yajl_buf_t;
pub type yajl_lexer = *mut yajl_lexer_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_boolean: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int>,
    pub yajl_integer:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_longlong) -> libc::c_int>,
    pub yajl_double: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> libc::c_int>,
    pub yajl_number:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, size_t) -> libc::c_int>,
    pub yajl_string: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, size_t) -> libc::c_int,
    >,
    pub yajl_start_map: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_map_key: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, size_t) -> libc::c_int,
    >,
    pub yajl_end_map: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_start_array: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_end_array: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub const yajl_status_ok: yajl_status = 0;
pub type yajl_status = libc::c_uint;
pub const yajl_status_error: yajl_status = 2;
pub const yajl_status_client_canceled: yajl_status = 1;
pub type yajl_option = libc::c_uint;
pub const yajl_allow_partial_values: yajl_option = 16;
pub const yajl_allow_multiple_values: yajl_option = 8;
pub const yajl_allow_trailing_garbage: yajl_option = 4;
pub const yajl_dont_validate_strings: yajl_option = 2;
pub const yajl_allow_comments: yajl_option = 1;
unsafe extern "C" fn value_alloc(mut type_0: yajl_type) -> yajl_val {
    let mut v: yajl_val = 0 as *mut yajl_val_s;
    v = malloc(::core::mem::size_of::<yajl_val_s>() as libc::c_ulong) as yajl_val;
    if v.is_null() {
        return 0 as yajl_val;
    }
    memset(
        v as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<yajl_val_s>() as libc::c_ulong,
    );
    (*v).type_0 = type_0;
    return v;
}
unsafe extern "C" fn yajl_object_free(mut v: yajl_val) {
    let mut i: size_t = 0;
    if !(!v.is_null()
        && (*v).type_0 as libc::c_uint == yajl_t_object as libc::c_int as libc::c_uint)
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*v).u.object.len {
        free(*((*v).u.object.keys).offset(i as isize) as *mut libc::c_char as *mut libc::c_void);
        let ref mut fresh0 = *((*v).u.object.keys).offset(i as isize);
        *fresh0 = 0 as *const libc::c_char;
        yajl_tree_free(*((*v).u.object.values).offset(i as isize));
        let ref mut fresh1 = *((*v).u.object.values).offset(i as isize);
        *fresh1 = 0 as yajl_val;
        i = i.wrapping_add(1);
    }
    free((*v).u.object.keys as *mut libc::c_void);
    free((*v).u.object.values as *mut libc::c_void);
    free(v as *mut libc::c_void);
}
unsafe extern "C" fn yajl_array_free(mut v: yajl_val) {
    let mut i: size_t = 0;
    if !(!v.is_null() && (*v).type_0 as libc::c_uint == yajl_t_array as libc::c_int as libc::c_uint)
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*v).u.array.len {
        yajl_tree_free(*((*v).u.array.values).offset(i as isize));
        let ref mut fresh2 = *((*v).u.array.values).offset(i as isize);
        *fresh2 = 0 as yajl_val;
        i = i.wrapping_add(1);
    }
    free((*v).u.array.values as *mut libc::c_void);
    free(v as *mut libc::c_void);
}
unsafe extern "C" fn context_push(mut ctx: *mut context_t, mut v: yajl_val) -> libc::c_int {
    let mut stack: *mut stack_elem_t = 0 as *mut stack_elem_t;
    stack = malloc(::core::mem::size_of::<stack_elem_t>() as libc::c_ulong) as *mut stack_elem_t;
    if stack.is_null() {
        if !((*ctx).errbuf).is_null() {
            snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 12 as libc::c_int;
    }
    memset(
        stack as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<stack_elem_t>() as libc::c_ulong,
    );
    (*stack).value = v;
    (*stack).next = (*ctx).stack;
    (*ctx).stack = stack;
    return 0 as libc::c_int;
}
unsafe extern "C" fn context_pop(mut ctx: *mut context_t) -> yajl_val {
    let mut stack: *mut stack_elem_t = 0 as *mut stack_elem_t;
    let mut v: yajl_val = 0 as *mut yajl_val_s;
    if ((*ctx).stack).is_null() {
        if !((*ctx).errbuf).is_null() {
            snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"context_pop: Bottom of stack reached prematurely\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as yajl_val;
    }
    stack = (*ctx).stack;
    (*ctx).stack = (*stack).next;
    v = (*stack).value;
    free(stack as *mut libc::c_void);
    return v;
}
unsafe extern "C" fn object_add_keyval(
    mut ctx: *mut context_t,
    mut obj: yajl_val,
    mut key: *mut libc::c_char,
    mut value: yajl_val,
) -> libc::c_int {
    let mut tmpk: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut tmpv: *mut yajl_val = 0 as *mut yajl_val;
    tmpk = realloc(
        (*obj).u.object.keys as *mut libc::c_void,
        (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_mul(((*obj).u.object.len).wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut *const libc::c_char;
    if tmpk.is_null() {
        if !((*ctx).errbuf).is_null() {
            snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 12 as libc::c_int;
    }
    (*obj).u.object.keys = tmpk;
    tmpv = realloc(
        (*obj).u.object.values as *mut libc::c_void,
        (::core::mem::size_of::<yajl_val>() as libc::c_ulong)
            .wrapping_mul(((*obj).u.object.len).wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut yajl_val;
    if tmpv.is_null() {
        if !((*ctx).errbuf).is_null() {
            snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 12 as libc::c_int;
    }
    (*obj).u.object.values = tmpv;
    let ref mut fresh3 = *((*obj).u.object.keys).offset((*obj).u.object.len as isize);
    *fresh3 = key;
    let ref mut fresh4 = *((*obj).u.object.values).offset((*obj).u.object.len as isize);
    *fresh4 = value;
    (*obj).u.object.len = ((*obj).u.object.len).wrapping_add(1);
    return 0 as libc::c_int;
}
unsafe extern "C" fn array_add_value(
    mut ctx: *mut context_t,
    mut array: yajl_val,
    mut value: yajl_val,
) -> libc::c_int {
    let mut tmp: *mut yajl_val = 0 as *mut yajl_val;
    tmp = realloc(
        (*array).u.array.values as *mut libc::c_void,
        (::core::mem::size_of::<yajl_val>() as libc::c_ulong)
            .wrapping_mul(((*array).u.array.len).wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut yajl_val;
    if tmp.is_null() {
        if !((*ctx).errbuf).is_null() {
            snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 12 as libc::c_int;
    }
    (*array).u.array.values = tmp;
    let ref mut fresh5 = *((*array).u.array.values).offset((*array).u.array.len as isize);
    *fresh5 = value;
    (*array).u.array.len = ((*array).u.array.len).wrapping_add(1);
    return 0 as libc::c_int;
}
unsafe extern "C" fn context_add_value(mut ctx: *mut context_t, mut v: yajl_val) -> libc::c_int {
    if ((*ctx).stack).is_null() {
        (*ctx).root = v;
        return 0 as libc::c_int;
    } else if !((*(*ctx).stack).value).is_null()
        && (*(*(*ctx).stack).value).type_0 as libc::c_uint
            == yajl_t_object as libc::c_int as libc::c_uint
    {
        if ((*(*ctx).stack).key).is_null() {
            if !(!v.is_null()
                && (*v).type_0 as libc::c_uint == yajl_t_string as libc::c_int as libc::c_uint)
            {
                if !((*ctx).errbuf).is_null() {
                    snprintf(
                        (*ctx).errbuf,
                        (*ctx).errbuf_size,
                        b"context_add_value: Object key is not a string (%#04x)\0" as *const u8
                            as *const libc::c_char,
                        (*v).type_0 as libc::c_uint,
                    );
                }
                return 22 as libc::c_int;
            }
            (*(*ctx).stack).key = (*v).u.string;
            (*v).u.string = 0 as *mut libc::c_char;
            free(v as *mut libc::c_void);
            return 0 as libc::c_int;
        } else {
            let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
            key = (*(*ctx).stack).key;
            (*(*ctx).stack).key = 0 as *mut libc::c_char;
            return object_add_keyval(ctx, (*(*ctx).stack).value, key, v);
        }
    } else if !((*(*ctx).stack).value).is_null()
        && (*(*(*ctx).stack).value).type_0 as libc::c_uint
            == yajl_t_array as libc::c_int as libc::c_uint
    {
        return array_add_value(ctx, (*(*ctx).stack).value, v);
    } else {
        if !((*ctx).errbuf).is_null() {
            snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"context_add_value: Cannot add value to a value of type %#04x (not a composite type)\0"
                    as *const u8 as *const libc::c_char,
                (*(*(*ctx).stack).value).type_0 as libc::c_uint,
            );
        }
        return 22 as libc::c_int;
    };
}
unsafe extern "C" fn handle_string(
    mut ctx: *mut libc::c_void,
    mut string: *const libc::c_uchar,
    mut string_length: size_t,
) -> libc::c_int {
    let mut v: yajl_val = 0 as *mut yajl_val_s;
    v = value_alloc(yajl_t_string);
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.string =
        malloc(string_length.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    if ((*v).u.string).is_null() {
        free(v as *mut libc::c_void);
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    memcpy(
        (*v).u.string as *mut libc::c_void,
        string as *const libc::c_void,
        string_length,
    );
    *((*v).u.string).offset(string_length as isize) = 0 as libc::c_int as libc::c_char;
    return if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn handle_number(
    mut ctx: *mut libc::c_void,
    mut string: *const libc::c_char,
    mut string_length: size_t,
) -> libc::c_int {
    let mut v: yajl_val = 0 as *mut yajl_val_s;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    v = value_alloc(yajl_t_number);
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.number.r =
        malloc(string_length.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    if ((*v).u.number.r).is_null() {
        free(v as *mut libc::c_void);
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    memcpy(
        (*v).u.number.r as *mut libc::c_void,
        string as *const libc::c_void,
        string_length,
    );
    *((*v).u.number.r).offset(string_length as isize) = 0 as libc::c_int as libc::c_char;
    (*v).u.number.flags = 0 as libc::c_int as libc::c_uint;
    *__errno_location() = 0 as libc::c_int;
    (*v).u.number.i = yajl_parse_integer(
        (*v).u.number.r as *const libc::c_uchar,
        strlen((*v).u.number.r) as libc::c_uint,
    );
    if *__errno_location() == 0 as libc::c_int {
        (*v).u.number.flags |= 0x1 as libc::c_int as libc::c_uint;
    }
    endptr = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    (*v).u.number.d = strtod((*v).u.number.r, &mut endptr);
    if *__errno_location() == 0 as libc::c_int
        && !endptr.is_null()
        && *endptr as libc::c_int == 0 as libc::c_int
    {
        (*v).u.number.flags |= 0x2 as libc::c_int as libc::c_uint;
    }
    return if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn handle_start_map(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut v: yajl_val = 0 as *mut yajl_val_s;
    v = value_alloc(yajl_t_object);
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.object.keys = 0 as *mut *const libc::c_char;
    (*v).u.object.values = 0 as *mut yajl_val;
    (*v).u.object.len = 0 as libc::c_int as size_t;
    return if context_push(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn handle_end_map(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut v: yajl_val = 0 as *mut yajl_val_s;
    v = context_pop(ctx as *mut context_t);
    if v.is_null() {
        return 0 as libc::c_int;
    }
    return if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn handle_start_array(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut v: yajl_val = 0 as *mut yajl_val_s;
    v = value_alloc(yajl_t_array);
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.array.values = 0 as *mut yajl_val;
    (*v).u.array.len = 0 as libc::c_int as size_t;
    return if context_push(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn handle_end_array(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut v: yajl_val = 0 as *mut yajl_val_s;
    v = context_pop(ctx as *mut context_t);
    if v.is_null() {
        return 0 as libc::c_int;
    }
    return if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn handle_boolean(
    mut ctx: *mut libc::c_void,
    mut boolean_value: libc::c_int,
) -> libc::c_int {
    let mut v: yajl_val = 0 as *mut yajl_val_s;
    v = value_alloc(
        (if boolean_value != 0 {
            yajl_t_true as libc::c_int
        } else {
            yajl_t_false as libc::c_int
        }) as yajl_type,
    );
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    return if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn handle_null(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut v: yajl_val = 0 as *mut yajl_val_s;
    v = value_alloc(yajl_t_null);
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    return if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn yajl_tree_parse(
    mut input: *const libc::c_char,
    mut error_buffer: *mut libc::c_char,
    mut error_buffer_size: size_t,
) -> yajl_val {
    static mut callbacks: yajl_callbacks = unsafe {
        {
            let mut init = yajl_callbacks {
                yajl_null: Some(
                    handle_null as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
                yajl_boolean: Some(
                    handle_boolean
                        as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int,
                ),
                yajl_integer: None,
                yajl_double: None,
                yajl_number: Some(
                    handle_number
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            size_t,
                        ) -> libc::c_int,
                ),
                yajl_string: Some(
                    handle_string
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_uchar,
                            size_t,
                        ) -> libc::c_int,
                ),
                yajl_start_map: Some(
                    handle_start_map as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
                yajl_map_key: Some(
                    handle_string
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_uchar,
                            size_t,
                        ) -> libc::c_int,
                ),
                yajl_end_map: Some(
                    handle_end_map as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
                yajl_start_array: Some(
                    handle_start_array as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
                yajl_end_array: Some(
                    handle_end_array as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
            };
            init
        }
    };
    let mut handle: yajl_handle = 0 as *mut yajl_handle_t;
    let mut status: yajl_status = yajl_status_ok;
    let mut internal_err_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ctx: context_t = {
        let mut init = context_s {
            stack: 0 as *mut stack_elem_t,
            root: 0 as yajl_val,
            errbuf: 0 as *mut libc::c_char,
            errbuf_size: 0 as libc::c_int as size_t,
        };
        init
    };
    ctx.errbuf = error_buffer;
    ctx.errbuf_size = error_buffer_size;
    if !error_buffer.is_null() {
        memset(
            error_buffer as *mut libc::c_void,
            0 as libc::c_int,
            error_buffer_size,
        );
    }
    handle = yajl_alloc(
        &callbacks,
        0 as *mut yajl_alloc_funcs,
        &mut ctx as *mut context_t as *mut libc::c_void,
    );
    yajl_config(handle, yajl_allow_comments, 1 as libc::c_int);
    status = yajl_parse(handle, input as *mut libc::c_uchar, strlen(input));
    status = yajl_complete_parse(handle);
    if status as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
        if !error_buffer.is_null() && error_buffer_size > 0 as libc::c_int as libc::c_ulong {
            internal_err_str = yajl_get_error(
                handle,
                1 as libc::c_int,
                input as *const libc::c_uchar,
                strlen(input),
            ) as *mut libc::c_char;
            snprintf(
                error_buffer,
                error_buffer_size,
                b"%s\0" as *const u8 as *const libc::c_char,
                internal_err_str,
            );
            ((*handle).alloc.free).expect("non-null function pointer")(
                (*handle).alloc.ctx,
                internal_err_str as *mut libc::c_void,
            );
        }
        yajl_free(handle);
        return 0 as yajl_val;
    }
    yajl_free(handle);
    return ctx.root;
}
#[no_mangle]
pub unsafe extern "C" fn yajl_tree_get(
    mut n: yajl_val,
    mut path: *mut *const libc::c_char,
    mut type_0: yajl_type,
) -> yajl_val {
    if path.is_null() {
        return 0 as yajl_val;
    }
    while !n.is_null() && !(*path).is_null() {
        let mut i: size_t = 0;
        let mut len: size_t = 0;
        if (*n).type_0 as libc::c_uint != yajl_t_object as libc::c_int as libc::c_uint {
            return 0 as yajl_val;
        }
        len = (*n).u.object.len;
        i = 0 as libc::c_int as size_t;
        while i < len {
            if strcmp(*path, *((*n).u.object.keys).offset(i as isize)) == 0 {
                n = *((*n).u.object.values).offset(i as isize);
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if i == len {
            return 0 as yajl_val;
        }
        path = path.offset(1);
    }
    if !n.is_null()
        && type_0 as libc::c_uint != yajl_t_any as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint != (*n).type_0 as libc::c_uint
    {
        n = 0 as yajl_val;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn yajl_tree_free(mut v: yajl_val) {
    if v.is_null() {
        return;
    }
    if !v.is_null() && (*v).type_0 as libc::c_uint == yajl_t_string as libc::c_int as libc::c_uint {
        free((*v).u.string as *mut libc::c_void);
        free(v as *mut libc::c_void);
    } else if !v.is_null()
        && (*v).type_0 as libc::c_uint == yajl_t_number as libc::c_int as libc::c_uint
    {
        free((*v).u.number.r as *mut libc::c_void);
        free(v as *mut libc::c_void);
    } else if !if !v.is_null()
        && (*v).type_0 as libc::c_uint == yajl_t_object as libc::c_int as libc::c_uint
    {
        &mut (*v).u.object as *mut C2RustUnnamed_1
    } else {
        0 as *mut C2RustUnnamed_1
    }
    .is_null()
    {
        yajl_object_free(v);
    } else if !if !v.is_null()
        && (*v).type_0 as libc::c_uint == yajl_t_array as libc::c_int as libc::c_uint
    {
        &mut (*v).u.array as *mut C2RustUnnamed_0
    } else {
        0 as *mut C2RustUnnamed_0
    }
    .is_null()
    {
        yajl_array_free(v);
    } else {
        free(v as *mut libc::c_void);
    };
}
