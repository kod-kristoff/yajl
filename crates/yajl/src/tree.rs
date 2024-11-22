#![allow(clippy::missing_safety_doc)]
#![allow(unused_unsafe)]
#![allow(clippy::nonminimal_bool)]
use core::ffi::{c_char, c_void, CStr};
use core::{fmt, ptr};

use ::libc;

use crate::{
    parser::{parse_integer, yajl_callbacks, ParseIntegerError, Parser},
    yajl_alloc::yajl_alloc_funcs,
    ParserOption, Status,
};

/// possible data types that a Value can hold
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ValueType {
    String = 1,
    Number = 2,
    Object = 3,
    Array = 4,
    True = 5,
    False = 6,
    Null = 7,
    /// The any type isn't valid for Value.type, but can be
    /// used as an argument to routines like `yajl_tree_get`.
    Any = 8,
}

impl ValueType {
    pub fn from_repr(r: u32) -> Option<Self> {
        match r {
            1 => Some(Self::String),
            2 => Some(Self::Number),
            3 => Some(Self::Object),
            4 => Some(Self::Array),
            5 => Some(Self::True),
            6 => Some(Self::False),
            7 => Some(Self::Null),
            8 => Some(Self::Any),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Value {
    pub type_0: ValueType,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub string: *mut c_char,
    pub number: Number,
    pub object: Object,
    pub array: Array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Array {
    pub values: *mut *mut Value,
    pub len: usize,
}
impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for i in 0..self.len {
            list.entry(unsafe { &**self.values.add(i) });
        }
        list.finish()
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Object {
    pub keys: *mut *const c_char,
    pub values: *mut *mut Value,
    pub len: usize,
}
impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for i in 0..self.len {
            map.key(unsafe { &CStr::from_ptr(*self.keys.add(i)) });
            map.value(unsafe { &**self.values.add(i) });
        }
        map.finish()
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Number {
    pub i: libc::c_longlong,
    pub d: libc::c_double,
    pub r: *mut c_char,
    pub flags: libc::c_uint,
}
impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.flags & 1 != 0 {
            f.write_fmt(format_args!("Integer({})", self.i))
        } else if self.flags & 2 != 0 {
            f.write_fmt(format_args!("Double({})", self.d))
        } else {
            f.write_fmt(format_args!("Number({:?})", unsafe {
                CStr::from_ptr(self.r)
            }))
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Context {
    pub stack: *mut stack_elem_t,
    pub root: *mut Value,
    pub errbuf: *mut c_char,
    pub errbuf_size: usize,
}
pub type stack_elem_t = stack_elem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_elem_s {
    pub key: *mut c_char,
    pub value: *mut Value,
    pub next: *mut stack_elem_t,
}
pub type yajl_handle = *mut Parser;

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_0 {
            ValueType::Any => f.write_str("Value::Any"),
            ValueType::Null => f.write_str("Value::Null"),
            ValueType::True => f.write_str("Value::True"),
            ValueType::False => f.write_str("Value::False"),
            ValueType::Number => f.write_fmt(format_args!("Value::Number({:?})", unsafe {
                self.u.number
            })),
            ValueType::Array => {
                f.write_fmt(format_args!("Value::Array({:?})", unsafe { self.u.array }))
            }
            ValueType::Object => f.write_fmt(format_args!("Value::Object({:?})", unsafe {
                self.u.object
            })),
            ValueType::String => f.write_fmt(format_args!("Value::String({:?})", unsafe {
                CStr::from_ptr(self.u.string)
            })),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum ContextError {
    OutOfMemory = 12,
    ObjectKeyIsNotAString,
    CantAddValueToNonCompsiteType = 22,
    BottomOfStackReachedPrematurely,
}

impl Value {
    unsafe fn alloc(mut type_0: ValueType) -> *mut Value {
        let mut v: *mut Value = ptr::null_mut();
        v = libc::malloc(::core::mem::size_of::<Value>()) as *mut Value;
        if v.is_null() {
            return 0 as *mut Value;
        }
        ptr::write_bytes(v, 0, 1);

        (*v).type_0 = type_0;
        v
    }
    unsafe fn object_free(mut v: *mut Value) {
        let mut i: usize = 0;
        if !(!v.is_null() && (*v).type_0 == ValueType::Object) {
            return;
        }
        i = 0 as libc::c_int as usize;
        while i < (*v).u.object.len {
            libc::free(*((*v).u.object.keys).add(i) as *mut c_char as *mut c_void);
            let fresh0 = &mut (*((*v).u.object.keys).add(i));
            *fresh0 = ptr::null::<c_char>();
            Value::tree_free(*((*v).u.object.values).add(i));
            let fresh1 = &mut (*((*v).u.object.values).add(i));
            *fresh1 = 0 as *mut Value;
            i = i.wrapping_add(1);
        }
        libc::free((*v).u.object.keys as *mut c_void);
        libc::free((*v).u.object.values as *mut c_void);
        libc::free(v as *mut c_void);
    }
    unsafe fn array_free(mut v: *mut Value) {
        let mut i: usize = 0;
        if !(!v.is_null() && (*v).type_0 == ValueType::Array) {
            return;
        }
        i = 0 as libc::c_int as usize;
        while i < (*v).u.array.len {
            Value::tree_free(*((*v).u.array.values).add(i));
            let fresh2 = &mut (*((*v).u.array.values).add(i));
            *fresh2 = 0 as *mut Value;
            i = i.wrapping_add(1);
        }
        libc::free((*v).u.array.values as *mut c_void);
        libc::free(v as *mut c_void);
    }
}

impl Context {
    unsafe fn push(mut ctx: *mut Context, mut v: *mut Value) -> Result<(), ContextError> {
        eprintln!("Context::push: v={:?}", *v);
        let mut stack: *mut stack_elem_t = ptr::null_mut::<stack_elem_t>();
        stack = libc::malloc(::core::mem::size_of::<stack_elem_t>()) as *mut stack_elem_t;
        if stack.is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"Out of memory\0" as *const u8 as *const c_char,
                );
            }
            return Err(ContextError::OutOfMemory);
        }
        ptr::write_bytes(stack, 0, 1);

        (*stack).value = v;
        (*stack).next = (*ctx).stack;
        (*ctx).stack = stack;
        Ok(())
    }
    unsafe fn pop(mut ctx: *mut Context) -> Result<*mut Value, ContextError> {
        let mut stack: *mut stack_elem_t = ptr::null_mut::<stack_elem_t>();
        let mut v: *mut Value = ptr::null_mut::<Value>();
        if ((*ctx).stack).is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"context_pop: Bottom of stack reached prematurely\0" as *const u8
                        as *const c_char,
                );
            }
            return dbg!(Err(ContextError::BottomOfStackReachedPrematurely));
        }
        stack = (*ctx).stack;
        (*ctx).stack = (*stack).next;
        v = (*stack).value;
        libc::free(stack as *mut c_void);
        eprintln!("Context::pop: v={:?}", *v);

        Ok(v)
    }

    unsafe fn object_add_keyval(
        mut ctx: *mut Context,
        mut obj: *mut Value,
        mut key: *mut c_char,
        mut value: *mut Value,
    ) -> Result<(), ContextError> {
        let mut tmpk: *mut *const c_char = ptr::null_mut::<*const c_char>();
        let mut tmpv: *mut *mut Value = ptr::null_mut::<*mut Value>();
        tmpk = libc::realloc(
            (*obj).u.object.keys as *mut c_void,
            (::core::mem::size_of::<*const c_char>())
                .wrapping_mul(((*obj).u.object.len).wrapping_add(1)),
        ) as *mut *const c_char;
        if tmpk.is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"Out of memory\0" as *const u8 as *const c_char,
                );
            }
            return Err(ContextError::OutOfMemory);
        }
        (*obj).u.object.keys = tmpk;
        tmpv = libc::realloc(
            (*obj).u.object.values as *mut c_void,
            (::core::mem::size_of::<*mut Value>())
                .wrapping_mul(((*obj).u.object.len).wrapping_add(1)),
        ) as *mut *mut Value;
        if tmpv.is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"Out of memory\0" as *const u8 as *const c_char,
                );
            }
            return Err(ContextError::OutOfMemory);
        }
        (*obj).u.object.values = tmpv;
        let fresh3 = &mut (*((*obj).u.object.keys).add((*obj).u.object.len));
        *fresh3 = key;
        let fresh4 = &mut (*((*obj).u.object.values).add((*obj).u.object.len));
        *fresh4 = value;
        (*obj).u.object.len = ((*obj).u.object.len).wrapping_add(1);
        Ok(())
    }
    unsafe fn array_add_value(
        mut ctx: *mut Context,
        mut array: *mut Value,
        mut value: *mut Value,
    ) -> Result<(), ContextError> {
        let mut tmp: *mut *mut Value = ptr::null_mut::<*mut Value>();
        tmp = libc::realloc(
            (*array).u.array.values as *mut c_void,
            (::core::mem::size_of::<*mut Value>())
                .wrapping_mul(((*array).u.array.len).wrapping_add(1)),
        ) as *mut *mut Value;
        if tmp.is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"Out of memory\0" as *const u8 as *const c_char,
                );
            }
            return Err(ContextError::OutOfMemory);
        }
        (*array).u.array.values = tmp;
        let fresh5 = &mut (*((*array).u.array.values).add((*array).u.array.len));
        *fresh5 = value;
        (*array).u.array.len = ((*array).u.array.len).wrapping_add(1);
        Ok(())
    }
    unsafe fn add_value(mut ctx: *mut Context, mut v: *mut Value) -> Result<(), ContextError> {
        if ((*ctx).stack).is_null() {
            (*ctx).root = v;
            Ok(())
        } else if !((*(*ctx).stack).value).is_null()
            && (*(*(*ctx).stack).value).type_0 as libc::c_uint
                == ValueType::Object as libc::c_int as libc::c_uint
        {
            if ((*(*ctx).stack).key).is_null() {
                if !(!v.is_null()
                    && (*v).type_0 as libc::c_uint
                        == ValueType::String as libc::c_int as libc::c_uint)
                {
                    if !((*ctx).errbuf).is_null() {
                        libc::snprintf(
                            (*ctx).errbuf,
                            (*ctx).errbuf_size,
                            b"Context::add_value: Object key is not a string (%#04x)\0" as *const u8
                                as *const c_char,
                            (*v).type_0 as libc::c_uint,
                        );
                    }
                    return Err(ContextError::ObjectKeyIsNotAString);
                }
                (*(*ctx).stack).key = (*v).u.string;
                (*v).u.string = ptr::null_mut::<c_char>();
                libc::free(v as *mut c_void);
                return Ok(());
            } else {
                let mut key: *mut c_char = ptr::null_mut::<c_char>();
                key = (*(*ctx).stack).key;
                (*(*ctx).stack).key = ptr::null_mut::<c_char>();
                return Context::object_add_keyval(ctx, (*(*ctx).stack).value, key, v);
            }
        } else if !((*(*ctx).stack).value).is_null()
            && (*(*(*ctx).stack).value).type_0 as libc::c_uint
                == ValueType::Array as libc::c_int as libc::c_uint
        {
            return Context::array_add_value(ctx, (*(*ctx).stack).value, v);
        } else {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"Context::add_value: Cannot add value to a value of type %#04x (not a composite type)\0"
                    as *const u8 as *const c_char,
                (*(*(*ctx).stack).value).type_0 as libc::c_uint,
            );
            }
            return Err(ContextError::CantAddValueToNonCompsiteType);
        }
    }
}
unsafe extern "C" fn handle_string(
    mut ctx: *mut c_void,
    mut string: *const libc::c_uchar,
    mut string_length: usize,
) -> libc::c_int {
    let mut v: *mut Value = ptr::null_mut::<Value>();
    v = Value::alloc(ValueType::String);
    if v.is_null() {
        if !((*(ctx as *mut Context)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut Context)).errbuf,
                (*(ctx as *mut Context)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.string = libc::malloc(string_length.wrapping_add(1)) as *mut c_char;
    if ((*v).u.string).is_null() {
        libc::free(v as *mut c_void);
        if !((*(ctx as *mut Context)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut Context)).errbuf,
                (*(ctx as *mut Context)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const c_char,
            );
        }
        return 0 as libc::c_int;
    }
    libc::memcpy(
        (*v).u.string as *mut c_void,
        string as *const c_void,
        string_length,
    );
    *((*v).u.string).add(string_length) = 0 as libc::c_int as c_char;

    dbg!(CStr::from_ptr((*v).u.string));
    match Context::add_value(ctx as *mut Context, v) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
unsafe extern "C" fn handle_number(
    mut ctx: *mut c_void,
    mut string: *const c_char,
    mut string_length: usize,
) -> libc::c_int {
    let mut v: *mut Value = ptr::null_mut::<Value>();
    let mut endptr: *mut c_char = ptr::null_mut::<c_char>();
    v = Value::alloc(ValueType::Number);
    if v.is_null() {
        if !((*(ctx as *mut Context)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut Context)).errbuf,
                (*(ctx as *mut Context)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.number.r = libc::malloc(string_length.wrapping_add(1)) as *mut c_char;
    if ((*v).u.number.r).is_null() {
        libc::free(v as *mut c_void);
        if !((*(ctx as *mut Context)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut Context)).errbuf,
                (*(ctx as *mut Context)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const c_char,
            );
        }
        return 0 as libc::c_int;
    }
    libc::memcpy(
        (*v).u.number.r as *mut c_void,
        string as *const c_void,
        string_length,
    );
    *((*v).u.number.r).add(string_length) = 0 as libc::c_int as c_char;
    (*v).u.number.flags = 0 as libc::c_int as libc::c_uint;
    (*v).u.number.i =
        match parse_integer((*v).u.number.r as *const u8, libc::strlen((*v).u.number.r)) {
            Ok(integer) => {
                (*v).u.number.flags |= 0x1;
                integer
            }
            Err(ParseIntegerError::Underflow) => i64::MIN,
            _ => i64::MAX,
        };
    if let Some(s) = CStr::from_ptr((*v).u.number.r).to_str().ok() {
        if let Some((d, d_len)) = strtod::strtod(s) {
            (*v).u.number.d = d;
            if d_len == string_length {
                (*v).u.number.flags |= 0x2;
            }
        } else {
            (*v).u.number.d = 0f64;
        };
    } else {
        (*v).u.number.d = 0f64;
    };

    match Context::add_value(ctx as *mut Context, v) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
unsafe extern "C" fn handle_start_map(mut ctx: *mut c_void) -> libc::c_int {
    let mut v: *mut Value = ptr::null_mut::<Value>();
    v = Value::alloc(ValueType::Object);
    if v.is_null() {
        if !((*(ctx as *mut Context)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut Context)).errbuf,
                (*(ctx as *mut Context)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.object.keys = ptr::null_mut::<*const c_char>();
    (*v).u.object.values = ptr::null_mut::<*mut Value>();
    (*v).u.object.len = 0 as libc::c_int as usize;
    match Context::push(ctx as *mut Context, v) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

unsafe extern "C" fn handle_end_map(mut ctx: *mut c_void) -> i32 {
    let Ok(v) = Context::pop(ctx as *mut Context) else {
        return 0;
    };
    match Context::add_value(ctx as *mut Context, v) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
unsafe extern "C" fn handle_start_array(mut ctx: *mut c_void) -> libc::c_int {
    let mut v: *mut Value = ptr::null_mut::<Value>();
    v = Value::alloc(ValueType::Array);
    if v.is_null() {
        if !((*(ctx as *mut Context)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut Context)).errbuf,
                (*(ctx as *mut Context)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.array.values = ptr::null_mut::<*mut Value>();
    (*v).u.array.len = 0 as libc::c_int as usize;
    match Context::add_value(ctx as *mut Context, v) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
unsafe extern "C" fn handle_end_array(mut ctx: *mut c_void) -> i32 {
    let Ok(v) = Context::pop(ctx as *mut Context) else {
        return 0;
    };
    match Context::add_value(ctx as *mut Context, v) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
unsafe extern "C" fn handle_boolean(
    mut ctx: *mut c_void,
    mut boolean_value: libc::c_int,
) -> libc::c_int {
    let mut v: *mut Value = ptr::null_mut::<Value>();
    v = Value::alloc(if boolean_value != 0 {
        ValueType::True
    } else {
        ValueType::False
    });
    if v.is_null() {
        if !((*(ctx as *mut Context)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut Context)).errbuf,
                (*(ctx as *mut Context)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const c_char,
            );
        }
        return 0 as libc::c_int;
    }
    match Context::add_value(ctx as *mut Context, v) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}
unsafe extern "C" fn handle_null(mut ctx: *mut c_void) -> libc::c_int {
    let mut v: *mut Value = ptr::null_mut::<Value>();
    v = Value::alloc(ValueType::Null);
    if v.is_null() {
        if !((*(ctx as *mut Context)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut Context)).errbuf,
                (*(ctx as *mut Context)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const c_char,
            );
        }
        return 0 as libc::c_int;
    }
    match Context::add_value(ctx as *mut Context, v) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

pub unsafe fn yajl_tree_parse(
    mut input: *const c_char,
    mut error_buffer: *mut c_char,
    mut error_buffer_size: usize,
) -> *mut Value {
    static mut callbacks: yajl_callbacks = unsafe {
        {
            yajl_callbacks {
                yajl_null: Some(handle_null as unsafe extern "C" fn(*mut c_void) -> libc::c_int),
                yajl_boolean: Some(
                    handle_boolean as unsafe extern "C" fn(*mut c_void, libc::c_int) -> libc::c_int,
                ),
                yajl_integer: None,
                yajl_double: None,
                yajl_number: Some(
                    handle_number
                        as unsafe extern "C" fn(*mut c_void, *const c_char, usize) -> libc::c_int,
                ),
                yajl_string: Some(
                    handle_string
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const libc::c_uchar,
                            usize,
                        ) -> libc::c_int,
                ),
                yajl_start_map: Some(
                    handle_start_map as unsafe extern "C" fn(*mut c_void) -> libc::c_int,
                ),
                yajl_map_key: Some(
                    handle_string
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const libc::c_uchar,
                            usize,
                        ) -> libc::c_int,
                ),
                yajl_end_map: Some(
                    handle_end_map as unsafe extern "C" fn(*mut c_void) -> libc::c_int,
                ),
                yajl_start_array: Some(
                    handle_start_array as unsafe extern "C" fn(*mut c_void) -> libc::c_int,
                ),
                yajl_end_array: Some(
                    handle_end_array as unsafe extern "C" fn(*mut c_void) -> libc::c_int,
                ),
            }
        }
    };
    let mut handle: yajl_handle = ptr::null_mut::<Parser>();
    let mut status = Status::Ok;
    let mut internal_err_str: *mut c_char = ptr::null_mut::<c_char>();
    let mut ctx: Context = {
        Context {
            stack: ptr::null_mut::<stack_elem_t>(),
            root: 0 as *mut Value,
            errbuf: ptr::null_mut::<c_char>(),
            errbuf_size: 0 as libc::c_int as usize,
        }
    };
    ctx.errbuf = error_buffer;
    ctx.errbuf_size = error_buffer_size;
    if !error_buffer.is_null() {
        ptr::write_bytes(error_buffer as *mut c_void, 0, error_buffer_size)

        //     error_buffer_size,
        // );
    }
    handle = Parser::alloc(
        ptr::addr_of!(callbacks),
        ptr::null_mut::<yajl_alloc_funcs>(),
        &mut ctx as *mut Context as *mut c_void,
    );
    let parser = unsafe { &mut *handle };
    parser.config(ParserOption::AllowComments, true);
    status = parser.parse(input as *mut libc::c_uchar, libc::strlen(input));
    status = parser.complete_parse();
    if status as libc::c_uint != Status::Ok as libc::c_int as libc::c_uint {
        if !error_buffer.is_null() && error_buffer_size > 0 as libc::c_int as usize {
            internal_err_str =
                parser.get_error(true, input as *const libc::c_uchar, libc::strlen(input))
                    as *mut c_char;
            libc::snprintf(
                error_buffer,
                error_buffer_size,
                b"%s\0" as *const u8 as *const c_char,
                internal_err_str,
            );
            ((*handle).alloc.free).expect("non-null function pointer")(
                (*handle).alloc.ctx,
                internal_err_str as *mut c_void,
            );
        }
        Parser::free(handle);
        return 0 as *mut Value;
    }
    Parser::free(handle);
    ctx.root
}

pub unsafe fn yajl_tree_get(
    mut n: *mut Value,
    mut path: *mut *const c_char,
    mut type_0: ValueType,
) -> *mut Value {
    if path.is_null() {
        return 0 as *mut Value;
    }
    while !n.is_null() && !(*path).is_null() {
        let mut i: usize = 0;
        let mut len: usize = 0;
        if (*n).type_0 as libc::c_uint != ValueType::Object as libc::c_int as libc::c_uint {
            return 0 as *mut Value;
        }
        len = (*n).u.object.len;
        i = 0 as libc::c_int as usize;
        while i < len {
            if libc::strcmp(*path, *((*n).u.object.keys).add(i)) == 0 {
                n = *((*n).u.object.values).add(i);
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if i == len {
            return 0 as *mut Value;
        }
        path = path.offset(1);
    }
    if !n.is_null()
        && type_0 as libc::c_uint != ValueType::Any as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint != (*n).type_0 as libc::c_uint
    {
        n = 0 as *mut Value;
    }
    n
}
unsafe fn yajl_is_string(v: *mut Value) -> bool {
    !v.is_null() && (*v).type_0 == ValueType::String
}
unsafe fn yajl_is_number(v: *mut Value) -> bool {
    !v.is_null() && (*v).type_0 == ValueType::Number
}
unsafe fn yajl_is_object(v: *mut Value) -> bool {
    !v.is_null() && (*v).type_0 == ValueType::Object
}
unsafe fn yajl_is_array(v: *mut Value) -> bool {
    !v.is_null() && (*v).type_0 == ValueType::Array
}
unsafe fn yajl_get_object(v: *mut Value) -> *mut Object {
    if yajl_is_object(v) {
        &mut (*v).u.object as *mut Object
    } else {
        ptr::null_mut()
    }
}
unsafe fn yajl_get_array(v: *mut Value) -> *mut Array {
    if yajl_is_array(v) {
        &mut (*v).u.array as *mut Array
    } else {
        ptr::null_mut()
    }
}
impl Value {
    pub unsafe fn tree_free(mut v: *mut Value) {
        if v.is_null() {
            return;
        }
        match (*v).type_0 {
            ValueType::String => {
                libc::free((*v).u.string as *mut c_void);
                libc::free(v as *mut c_void);
            }
            ValueType::Number => {
                libc::free((*v).u.number.r as *mut c_void);
                libc::free(v as *mut c_void);
            }
            ValueType::Object => {
                Value::object_free(v);
            }
            ValueType::Array => Value::array_free(v),
            _ => {
                libc::free(v as *mut c_void);
            }
        }
        // if yajl_is_string(v) {
        //     libc::free((*v).u.string as *mut c_void);
        //     libc::free(v as *mut c_void);
        // } else if yajl_is_number(v) {
        //     libc::free((*v).u.number.r as *mut c_void);
        //     libc::free(v as *mut c_void);
        // } else if !yajl_get_object(v).is_null() {
        //     yajl_object_free(v);
        // } else if !yajl_get_array(v).is_null() {
        //     yajl_array_free(v);
        // } else {
        //     libc::free(v as *mut c_void);
        // };
    }
}
