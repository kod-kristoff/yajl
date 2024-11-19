use ::libc;
use core::{ffi::c_char, ptr};
pub(crate) use parser_impl::yajl_parse_integer;
use parser_impl::{ByteStack, ParseState};

use crate::{
    yajl_alloc::{yajl_alloc_funcs, yajl_set_default_alloc_funcs},
    yajl_buf::{yajl_buf_alloc, yajl_buf_free, yajl_buf_t},
    yajl_lex::{yajl_lex_alloc, yajl_lex_free, yajl_lexer_t},
    Status,
};

mod parser_impl;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Parser {
    pub callbacks: *const yajl_callbacks,
    pub ctx: *mut libc::c_void,
    pub lexer: yajl_lexer,
    pub parseError: Option<ParseError>,
    pub bytesConsumed: usize,
    pub decodeBuf: yajl_buf,
    pub stateStack: ByteStack,
    pub alloc: yajl_alloc_funcs,
    pub flags: libc::c_uint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParseError {
    ClientCancelled = 1,
    FloatingPointOverflow,
    IntegerOverflow,
    InvalidArraySeparator,
    InvalidKeyValueSeparator,
    InvalidObjectKey,
    InvalidObjectSeparator,
    InvalidToken,
    PrematureEof,
    TrailingGarbage,
    UnallowedToken,
}

impl ParseError {
    fn to_c_str_ptr(&self) -> *const c_char {
        match self {
            Self::ClientCancelled => {
                b"client cancelled parse via callback return value\0" as *const u8 as *const c_char
            }
            Self::FloatingPointOverflow => {
                b"numeric (floating point) overflow\0" as *const u8 as *const c_char
            }
            Self::IntegerOverflow => b"integer overflow\0" as *const u8 as *const c_char,
            Self::InvalidArraySeparator => {
                b"after array element, I expect ',' or ']'\0" as *const u8 as *const c_char
            }
            Self::InvalidKeyValueSeparator => {
                b"object key and value must be separated by a colon (':')\0" as *const u8
                    as *const c_char
            }
            Self::InvalidObjectKey => {
                b"invalid object key (must be a string)\0" as *const u8 as *const c_char
            }
            Self::InvalidObjectSeparator => {
                b"after key and value, inside map, I expect ',' or '}'\0" as *const u8
                    as *const c_char
            }
            Self::InvalidToken => b"invalid token, internal error\0" as *const u8 as *const c_char,
            Self::PrematureEof => b"premature EOF\0" as *const u8 as *const c_char,
            Self::TrailingGarbage => b"trailing garbage\0" as *const u8 as *const c_char,
            Self::UnallowedToken => {
                b"unallowed token at this point in JSON text\0" as *const u8 as *const c_char
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ParserOption {
    AllowComments = 1,
    DontValidateStrings = 2,
    AllowTrailingGarbage = 4,
    AllowMultipleValues = 8,
    AllowPartialValues = 16,
}

impl ParserOption {
    pub fn from_repr(x: u32) -> Option<ParserOption> {
        match x {
            1 => Some(ParserOption::AllowComments),
            2 => Some(ParserOption::DontValidateStrings),
            4 => Some(ParserOption::AllowTrailingGarbage),
            8 => Some(ParserOption::AllowMultipleValues),
            16 => Some(ParserOption::AllowPartialValues),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_boolean: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int>,
    pub yajl_integer:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_longlong) -> libc::c_int>,
    pub yajl_double: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> libc::c_int>,
    pub yajl_number:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, usize) -> libc::c_int>,
    pub yajl_string:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, usize) -> libc::c_int>,
    pub yajl_start_map: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_map_key:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, usize) -> libc::c_int>,
    pub yajl_end_map: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_start_array: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub yajl_end_array: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void>;
pub type yajl_free_func = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub type yajl_realloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, usize) -> *mut libc::c_void>;

pub type yajl_buf = *mut yajl_buf_t;
pub type yajl_lexer = *mut yajl_lexer_t;

pub type yajl_handle = *mut Parser;
pub const yajl_state_start: C2RustUnnamed = 0;

pub type va_list = __builtin_va_list;
pub type C2RustUnnamed = libc::c_uint;
pub const yajl_state_got_value: C2RustUnnamed = 12;
pub const yajl_state_array_need_val: C2RustUnnamed = 11;
pub const yajl_state_array_got_val: C2RustUnnamed = 10;
pub const yajl_state_array_start: C2RustUnnamed = 9;
pub const yajl_state_map_need_key: C2RustUnnamed = 8;
pub const yajl_state_map_got_val: C2RustUnnamed = 7;
pub const yajl_state_map_need_val: C2RustUnnamed = 6;
pub const yajl_state_map_sep: C2RustUnnamed = 5;
pub const yajl_state_map_start: C2RustUnnamed = 4;
pub const yajl_state_lexical_error: C2RustUnnamed = 3;
pub const yajl_state_parse_error: C2RustUnnamed = 2;
pub const yajl_state_parse_complete: C2RustUnnamed = 1;

pub unsafe extern "C" fn yajl_status_to_string(stat: Status) -> *const libc::c_char {
    let mut statStr: *const libc::c_char = b"unknown\0" as *const u8 as *const libc::c_char;
    match stat as libc::c_uint {
        0 => {
            statStr = b"ok, no error\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            statStr = b"client canceled parse\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            statStr = b"parse error\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    statStr
}

impl Parser {
    /// allocate a parser handle
    ///
    /// # Arguments
    ///
    /// * `callbacks` - a yajl callbacks structure specifying the
    ///                    functions to call when different JSON entities
    ///                    are encountered in the input text.  May be NULL,
    ///                    which is only useful for validation.
    /// * `afs` - memory allocation functions, may be NULL for to use
    ///                    C runtime library routines (malloc and friends)
    /// * `ctx` - a context pointer that will be passed to callbacks.
    ///
    /// # Safety
    ///
    /// The caller is responsible for free the handle by calling `Parser::free`
    pub unsafe fn alloc(
        mut callbacks: *const yajl_callbacks,
        mut afs: *mut yajl_alloc_funcs,
        mut ctx: *mut libc::c_void,
    ) -> *mut Parser {
        let mut hand: *mut Parser = ptr::null_mut();
        let mut afsBuffer: yajl_alloc_funcs = yajl_alloc_funcs {
            malloc: None,
            realloc: None,
            free: None,
            ctx: ptr::null_mut::<libc::c_void>(),
        };
        if !afs.is_null() {
            if ((*afs).malloc).is_none() || ((*afs).realloc).is_none() || ((*afs).free).is_none() {
                return ptr::null_mut();
            }
        } else {
            yajl_set_default_alloc_funcs(&mut afsBuffer);
            afs = &mut afsBuffer;
        }
        hand = ((*afs).malloc).expect("non-null function pointer")(
            (*afs).ctx,
            ::core::mem::size_of::<Parser>(),
        ) as yajl_handle;
        libc::memcpy(
            &mut (*hand).alloc as *mut yajl_alloc_funcs as *mut libc::c_void,
            afs as *mut libc::c_void,
            ::core::mem::size_of::<yajl_alloc_funcs>(),
        );
        (*hand).callbacks = callbacks;
        (*hand).ctx = ctx;
        (*hand).lexer = ptr::null_mut();
        (*hand).bytesConsumed = 0;
        (*hand).decodeBuf = yajl_buf_alloc(&mut (*hand).alloc);
        (*hand).flags = 0;
        (*hand).stateStack = ByteStack::new(&mut (*hand).alloc);
        // (*hand).stateStack.stack = ptr::null_mut::<libc::c_uchar>();
        // (*hand).stateStack.size = 0;
        // (*hand).stateStack.used = 0;
        // (*hand).stateStack.yaf = &mut (*hand).alloc;
        // if ((*hand).stateStack.size).wrapping_sub((*hand).stateStack.used) == 0 {
        //     (*hand).stateStack.size = ((*hand).stateStack.size).wrapping_add(128);
        //     (*hand).stateStack.stack = ((*(*hand).stateStack.yaf).realloc)
        //         .expect("non-null function pointer")(
        //         (*(*hand).stateStack.yaf).ctx,
        //         (*hand).stateStack.stack as *mut libc::c_void,
        //         (*hand).stateStack.size,
        //     ) as *mut libc::c_uchar;
        // }
        // let fresh0 = (*hand).stateStack.used;
        // (*hand).stateStack.used = ((*hand).stateStack.used).wrapping_add(1);
        // *((*hand).stateStack.stack).add(fresh0) = yajl_state_start as u8;
        (*hand).stateStack.push(ParseState::Start);
        hand
    }
    // pub fn new(mut callbacks: *const yajl_callbacks,
    // mut afs: *mut yajl_alloc_funcs,
    // mut ctx: *mut libc::c_void,) -> Self {
    //     Self { callbacks: callbacks, ctx: ctx, lexer: ptr::null_mut(), parseError: ptr::null(), bytesConsumed: 0, decodeBuf: (), stateStack: (), alloc: (), flags: () }
    // }

    pub unsafe fn free(mut handle: yajl_handle) {
        // if !((*handle).stateStack.stack).is_null() {
        //     ((*(*handle).stateStack.yaf).free).expect("non-null function pointer")(
        //         (*(*handle).stateStack.yaf).ctx,
        //         (*handle).stateStack.stack as *mut libc::c_void,
        //     );
        // }
        (*handle).stateStack.free();
        yajl_buf_free((*handle).decodeBuf);
        if !((*handle).lexer).is_null() {
            yajl_lex_free((*handle).lexer);
            (*handle).lexer = ptr::null_mut();
        }
        ((*handle).alloc.free).expect("non-null function pointer")(
            (*handle).alloc.ctx,
            handle as *mut libc::c_void,
        );
    }
}
impl Parser {
    pub fn config(&mut self, opt: ParserOption, arg: bool) -> bool {
        match opt as u32 {
            1 | 2 | 4 | 8 | 16 => {
                if arg {
                    self.flags |= opt as u32;
                } else {
                    self.flags &= !(opt as u32);
                }
                true
            }
            _ => false,
        }
    }
    pub unsafe fn parse(
        &mut self,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
    ) -> Status {
        if (self.lexer).is_null() {
            self.lexer = yajl_lex_alloc(
                &mut self.alloc,
                self.flags & ParserOption::AllowComments as u32,
                (self.flags & ParserOption::DontValidateStrings as u32 == 0) as libc::c_int
                    as libc::c_uint,
            );
        }
        self.do_parse(jsonText, jsonTextLen)
    }

    pub unsafe fn complete_parse(&mut self) -> Status {
        if (self.lexer).is_null() {
            self.lexer = yajl_lex_alloc(
                &mut self.alloc,
                self.flags & ParserOption::AllowComments as u32,
                (self.flags & ParserOption::DontValidateStrings as u32 == 0) as libc::c_int
                    as libc::c_uint,
            );
        }
        self.do_finish()
    }

    pub unsafe fn get_error(
        &mut self,
        mut verbose: bool,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
    ) -> *mut libc::c_uchar {
        self.render_error_string(jsonText, jsonTextLen, verbose)
    }

    pub fn get_bytes_consumed(&self) -> usize {
        self.bytesConsumed
    }
    pub unsafe fn free_error(&self, mut str: *mut libc::c_uchar) {
        (self.alloc.free).expect("non-null function pointer")(
            self.alloc.ctx,
            str as *mut libc::c_void,
        );
    }
}
