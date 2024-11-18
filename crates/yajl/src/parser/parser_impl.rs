#![allow(clippy::missing_safety_doc)]
use core::ffi::c_void;
use core::ptr;

use ::libc;

use crate::{
    yajl_alloc::yajl_alloc_funcs,
    yajl_buf::{yajl_buf_append, yajl_buf_clear, yajl_buf_data, yajl_buf_len, yajl_buf_t},
    yajl_encode::yajl_string_decode,
    yajl_lex::{yajl_lex_error_to_string, yajl_lex_get_error, yajl_lex_lex, yajl_lexer_t},
    ParserOption, Status,
};

#[cfg(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "haiku",
    target_os = "illumos",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "solaris"
))]
#[allow(dead_code)]
use crate::util_libc::{get_last_error, set_last_error};

use super::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParseState {
    Start = 0,
    GotValue = 12,
    ArrayNeedVal = 11,
    ArrayGotVal = 10,
    ArrayStart = 9,
    MapNeedKey = 8,
    MapGotVal = 7,
    MapNeedVal = 6,
    MapSep = 5,
    MapStart = 4,
    LexicalError = 3,
    ParseError = 2,
    ParseComplete = 1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ByteStack {
    stack: *mut ParseState,
    cap: usize,
    len: usize,
    yaf: *mut yajl_alloc_funcs,
}

#[derive(Debug, Clone)]
pub enum ByteStackError {
    OutOfMemory,
    NoAllocFuncs,
}
impl ByteStack {
    pub fn new(yaf: *mut yajl_alloc_funcs) -> ByteStack {
        assert!(!yaf.is_null());
        ByteStack {
            stack: ptr::null_mut(),
            cap: 0,
            len: 0,
            yaf,
        }
    }

    pub fn free(&mut self) {
        if !self.stack.is_null() {
            unsafe {
                ((*self.yaf).free).expect("non-null function poiner")(
                    (*self.yaf).ctx,
                    self.stack as *mut c_void,
                );
            }
            self.stack = ptr::null_mut();
        }
    }

    pub fn push(&mut self, state: ParseState) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            *self.stack.add(self.len) = state;
        }
        self.len += 1;
    }
    pub fn pop(&mut self) {
        self.len -= 1;
    }

    pub fn top(&self) -> ParseState {
        debug_assert!(self.len > 0);
        unsafe { *self.stack.add(self.len - 1) }
    }
    pub fn top_mut(&mut self) -> &mut ParseState {
        debug_assert!(self.len > 0);
        unsafe { &mut *self.stack.add(self.len - 1) }
    }

    fn grow(&mut self) {
        self.cap = self.cap.wrapping_add(128);
        unsafe {
            self.stack = ((*self.yaf).realloc).expect("non-null function poiner")(
                (*self.yaf).ctx,
                self.stack as *mut c_void,
                self.cap,
            ) as *mut ParseState;
        }
        debug_assert!(!self.stack.is_null());
    }
}
pub type yajl_buf = *mut yajl_buf_t;
pub type yajl_lexer = *mut yajl_lexer_t;

pub type yajl_handle = *mut Parser;

pub type yajl_tok = libc::c_uint;
pub const yajl_tok_comment: yajl_tok = 14;
pub const yajl_tok_string_with_escapes: yajl_tok = 13;
pub const yajl_tok_string: yajl_tok = 12;
pub const yajl_tok_double: yajl_tok = 11;
pub const yajl_tok_integer: yajl_tok = 10;
pub const yajl_tok_right_bracket: yajl_tok = 9;
pub const yajl_tok_right_brace: yajl_tok = 8;
pub const yajl_tok_null: yajl_tok = 7;
pub const yajl_tok_left_bracket: yajl_tok = 6;
pub const yajl_tok_left_brace: yajl_tok = 5;
pub const yajl_tok_error: yajl_tok = 4;
pub const yajl_tok_eof: yajl_tok = 3;
pub const yajl_tok_comma: yajl_tok = 2;
pub const yajl_tok_colon: yajl_tok = 1;
pub const yajl_tok_bool: yajl_tok = 0;
pub type yajl_lex_error = libc::c_uint;
pub const yajl_lex_unallowed_comment: yajl_lex_error = 10;
pub const yajl_lex_missing_integer_after_minus: yajl_lex_error = 9;
pub const yajl_lex_missing_integer_after_exponent: yajl_lex_error = 8;
pub const yajl_lex_missing_integer_after_decimal: yajl_lex_error = 7;
pub const yajl_lex_invalid_string: yajl_lex_error = 6;
pub const yajl_lex_invalid_char: yajl_lex_error = 5;
pub const yajl_lex_string_invalid_hex_char: yajl_lex_error = 4;
pub const yajl_lex_string_invalid_json_char: yajl_lex_error = 3;
pub const yajl_lex_string_invalid_escaped_char: yajl_lex_error = 2;
pub const yajl_lex_string_invalid_utf8: yajl_lex_error = 1;
pub const yajl_lex_e_ok: yajl_lex_error = 0;

pub unsafe extern "C" fn yajl_parse_integer(
    mut number: *const libc::c_uchar,
    mut length: libc::c_uint,
) -> libc::c_longlong {
    let mut ret: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut sign: libc::c_long = 1 as libc::c_int as libc::c_long;
    let mut pos: *const libc::c_uchar = number;
    if *pos as libc::c_int == '-' as i32 {
        pos = pos.offset(1);
        sign = -(1 as libc::c_int) as libc::c_long;
    }
    if *pos as libc::c_int == '+' as i32 {
        pos = pos.offset(1);
    }
    while pos < number.offset(length as isize) {
        if ret
            > 9223372036854775807 as libc::c_longlong / 10 as libc::c_int as libc::c_longlong
                + 9223372036854775807 as libc::c_longlong % 10 as libc::c_int as libc::c_longlong
        {
            set_last_error(34);
            return if sign == 1 as libc::c_int as libc::c_long {
                9223372036854775807 as libc::c_longlong
            } else {
                -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
            };
        }
        ret *= 10 as libc::c_int as libc::c_longlong;
        if 9223372036854775807 as libc::c_longlong - ret
            < (*pos as libc::c_int - '0' as i32) as libc::c_longlong
        {
            set_last_error(34);
            return if sign == 1 as libc::c_int as libc::c_long {
                9223372036854775807 as libc::c_longlong
            } else {
                -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
            };
        }
        if (*pos as libc::c_int) < '0' as i32 || *pos as libc::c_int > '9' as i32 {
            set_last_error(34);
            return if sign == 1 as libc::c_int as libc::c_long {
                9223372036854775807 as libc::c_longlong
            } else {
                -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
            };
        }
        let fresh0 = pos;
        pos = pos.offset(1);
        ret += (*fresh0 as libc::c_int - '0' as i32) as libc::c_longlong;
    }
    sign as libc::c_longlong * ret
}

pub unsafe extern "C" fn yajl_render_error_string(
    mut hand: yajl_handle,
    mut jsonText: *const libc::c_uchar,
    mut jsonTextLen: usize,
    mut verbose: libc::c_int,
) -> *mut libc::c_uchar {
    let mut offset: usize = (*hand).bytesConsumed;
    let mut str: *mut libc::c_uchar = std::ptr::null_mut::<libc::c_uchar>();
    let mut errorType: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut errorText: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut text: [libc::c_char; 72] = [0; 72];
    let mut arrow: *const libc::c_char =
        b"                     (right here) ------^\n\0" as *const u8 as *const libc::c_char;
    match (*hand).stateStack.top() {
        ParseState::ParseError => {
            errorType = b"parse\0" as *const u8 as *const libc::c_char;
            errorText = (*hand).parseError;
        }
        ParseState::LexicalError => {
            errorType = b"lexical\0" as *const u8 as *const libc::c_char;
            errorText = yajl_lex_error_to_string(yajl_lex_get_error((*hand).lexer));
        }
        _ => {
            errorType = b"unknown\0" as *const u8 as *const libc::c_char;
        }
    }
    let mut memneeded: usize = 0;
    memneeded = (memneeded).wrapping_add(libc::strlen(errorType));
    memneeded = (memneeded).wrapping_add(libc::strlen(
        b" error\0" as *const u8 as *const libc::c_char,
    ));
    if !errorText.is_null() {
        memneeded =
            memneeded.wrapping_add(libc::strlen(b": \0" as *const u8 as *const libc::c_char));
        memneeded = memneeded.wrapping_add(libc::strlen(errorText));
    }
    str = ((*hand).alloc.malloc).expect("non-null function pointer")(
        (*hand).alloc.ctx,
        memneeded.wrapping_add(2 as libc::c_int as usize),
    ) as *mut libc::c_uchar;
    if str.is_null() {
        return ptr::null_mut::<libc::c_uchar>();
    }
    *str.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    libc::strcat(str as *mut libc::c_char, errorType);
    libc::strcat(
        str as *mut libc::c_char,
        b" error\0" as *const u8 as *const libc::c_char,
    );
    if !errorText.is_null() {
        libc::strcat(
            str as *mut libc::c_char,
            b": \0" as *const u8 as *const libc::c_char,
        );
        libc::strcat(str as *mut libc::c_char, errorText);
    }
    libc::strcat(
        str as *mut libc::c_char,
        b"\n\0" as *const u8 as *const libc::c_char,
    );
    if verbose != 0 {
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut i: usize = 0;
        let mut spacesNeeded: usize = 0;
        spacesNeeded = if offset < 30 as libc::c_int as usize {
            (40 as libc::c_int as usize).wrapping_sub(offset)
        } else {
            10 as libc::c_int as usize
        };
        start = if offset >= 30 as libc::c_int as usize {
            offset.wrapping_sub(30 as libc::c_int as usize)
        } else {
            0 as libc::c_int as usize
        };
        end = if offset.wrapping_add(30 as libc::c_int as usize) > jsonTextLen {
            jsonTextLen
        } else {
            offset.wrapping_add(30 as libc::c_int as usize)
        };
        i = 0 as libc::c_int as usize;
        while i < spacesNeeded {
            text[i] = ' ' as i32 as libc::c_char;
            i = i.wrapping_add(1);
        }
        while start < end {
            if *jsonText.add(start) as libc::c_int != '\n' as i32
                && *jsonText.add(start) as libc::c_int != '\r' as i32
            {
                text[i] = *jsonText.add(start) as libc::c_char;
            } else {
                text[i] = ' ' as i32 as libc::c_char;
            }
            start = start.wrapping_add(1);
            i = i.wrapping_add(1);
        }
        let fresh1 = i;
        i = i.wrapping_add(1);
        text[fresh1] = '\n' as i32 as libc::c_char;
        text[i] = 0 as libc::c_int as libc::c_char;
        let mut newStr: *mut libc::c_char = ((*hand).alloc.malloc)
            .expect("non-null function pointer")(
            (*hand).alloc.ctx,
            (libc::strlen(str as *mut libc::c_char))
                .wrapping_add(libc::strlen(text.as_mut_ptr()))
                .wrapping_add(libc::strlen(arrow))
                .wrapping_add(1),
        ) as *mut libc::c_char;
        if !newStr.is_null() {
            *newStr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
            libc::strcat(newStr, str as *mut libc::c_char);
            libc::strcat(newStr, text.as_mut_ptr());
            libc::strcat(newStr, arrow);
        }
        ((*hand).alloc.free).expect("non-null function pointer")(
            (*hand).alloc.ctx,
            str as *mut libc::c_void,
        );
        str = newStr as *mut libc::c_uchar;
    }
    str
}
impl Parser {
    pub unsafe fn do_finish(&mut self) -> Status {
        let stat = self.do_parse(b" \0" as *const u8 as *const libc::c_uchar, 1 as usize);
        if stat != Status::Ok {
            return stat;
        }
        match self.stateStack.top() {
            ParseState::ParseError | ParseState::LexicalError => Status::Error,
            ParseState::GotValue | ParseState::ParseComplete => Status::Ok,
            _ => {
                if self.flags & ParserOption::AllowPartialValues as u32 == 0 {
                    *self.stateStack.top_mut() = ParseState::ParseError;
                    self.parseError = b"premature EOF\0" as *const u8 as *const libc::c_char;
                    return Status::Error;
                }
                Status::Ok
            }
        }
    }

    pub unsafe fn do_parse(
        &mut self,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
    ) -> Status {
        let mut current_block: u64;
        let mut tok: yajl_tok = yajl_tok_bool;
        let mut buf: *const libc::c_uchar = ptr::null::<libc::c_uchar>();
        let mut bufLen: usize = 0;
        let mut offset: *mut usize = &mut self.bytesConsumed;
        *offset = 0;
        loop {
            match self.stateStack.top() {
                ParseState::ParseComplete => {
                    if self.flags & ParserOption::AllowMultipleValues as u32 != 0 {
                        *self.stateStack.top_mut() = ParseState::GotValue;
                    } else {
                        if self.flags & ParserOption::AllowTrailingGarbage as libc::c_uint != 0 {
                            break;
                        }
                        if *offset == jsonTextLen {
                            break;
                        }
                        tok = yajl_lex_lex(
                            self.lexer,
                            jsonText,
                            jsonTextLen,
                            offset,
                            &mut buf,
                            &mut bufLen,
                        );
                        if tok as libc::c_uint != yajl_tok_eof as libc::c_int as libc::c_uint {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError =
                                b"trailing garbage\0" as *const u8 as *const libc::c_char;
                        }
                    }
                }
                ParseState::ParseError | ParseState::LexicalError => return Status::Error,
                ParseState::Start
                | ParseState::GotValue
                | ParseState::MapNeedVal
                | ParseState::ArrayNeedVal
                | ParseState::ArrayStart => {
                    let mut stateToPush = ParseState::Start;
                    tok = yajl_lex_lex(
                        self.lexer,
                        jsonText,
                        jsonTextLen,
                        offset,
                        &mut buf,
                        &mut bufLen,
                    );
                    match tok as libc::c_uint {
                        3 => return Status::Ok,
                        4 => {
                            *self.stateStack.top_mut() = ParseState::LexicalError;
                            continue;
                        }
                        12 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_string).is_some()
                                && ((*self.callbacks).yajl_string)
                                    .expect("non-null function pointer")(
                                    self.ctx, buf, bufLen
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const libc::c_char;
                                return Status::ClientCanceled;
                            }
                            current_block = 6407515180622463684;
                        }
                        13 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_string).is_some()
                            {
                                yajl_buf_clear(self.decodeBuf);
                                yajl_string_decode(self.decodeBuf, buf, bufLen);
                                if ((*self.callbacks).yajl_string)
                                    .expect("non-null function pointer")(
                                    self.ctx,
                                    yajl_buf_data(self.decodeBuf),
                                    yajl_buf_len(self.decodeBuf),
                                ) == 0
                                {
                                    *self.stateStack.top_mut() = ParseState::ParseError;
                                    self.parseError =
                                        b"client cancelled parse via callback return value\0"
                                            as *const u8
                                            as *const libc::c_char;
                                    return Status::ClientCanceled;
                                }
                            }
                            current_block = 6407515180622463684;
                        }
                        0 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_boolean).is_some()
                                && ((*self.callbacks).yajl_boolean)
                                    .expect("non-null function pointer")(
                                    self.ctx,
                                    (*buf as libc::c_int == 't' as i32) as libc::c_int,
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const libc::c_char;
                                return Status::ClientCanceled;
                            }
                            current_block = 6407515180622463684;
                        }
                        7 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_null).is_some()
                                && ((*self.callbacks).yajl_null).expect("non-null function pointer")(
                                    self.ctx,
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const libc::c_char;
                                return Status::ClientCanceled;
                            }
                            current_block = 6407515180622463684;
                        }
                        6 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_start_map).is_some()
                                && ((*self.callbacks).yajl_start_map)
                                    .expect("non-null function pointer")(
                                    self.ctx
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const libc::c_char;
                                return Status::ClientCanceled;
                            }
                            stateToPush = ParseState::MapStart;
                            current_block = 6407515180622463684;
                        }
                        5 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_start_array).is_some()
                                && ((*self.callbacks).yajl_start_array)
                                    .expect("non-null function pointer")(
                                    self.ctx
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const libc::c_char;
                                return Status::ClientCanceled;
                            }
                            stateToPush = ParseState::ArrayStart;
                            current_block = 6407515180622463684;
                        }
                        10 => {
                            if !(self.callbacks).is_null() {
                                if ((*self.callbacks).yajl_number).is_some() {
                                    if ((*self.callbacks).yajl_number)
                                        .expect("non-null function pointer")(
                                        self.ctx,
                                        buf as *const libc::c_char,
                                        bufLen,
                                    ) == 0
                                    {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError =
                                            b"client cancelled parse via callback return value\0"
                                                as *const u8
                                                as *const libc::c_char;
                                        return Status::ClientCanceled;
                                    }
                                } else if ((*self.callbacks).yajl_integer).is_some() {
                                    let mut i: libc::c_longlong =
                                        0 as libc::c_int as libc::c_longlong;
                                    set_last_error(0);
                                    i = yajl_parse_integer(buf, bufLen as libc::c_uint);
                                    if (i
                                        == -(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong
                                        || i == 9223372036854775807 as libc::c_longlong)
                                        && get_last_error() == 34 as libc::c_int
                                    {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError = b"integer overflow\0" as *const u8
                                            as *const libc::c_char;
                                        if *offset >= bufLen {
                                            *offset = { *offset }.wrapping_sub(bufLen);
                                        } else {
                                            *offset = 0 as libc::c_int as usize;
                                        }
                                        continue;
                                    } else if ((*self.callbacks).yajl_integer)
                                        .expect("non-null function pointer")(
                                        self.ctx, i
                                    ) == 0
                                    {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError =
                                            b"client cancelled parse via callback return value\0"
                                                as *const u8
                                                as *const libc::c_char;
                                        return Status::ClientCanceled;
                                    }
                                }
                                current_block = 6407515180622463684;
                            } else {
                                current_block = 6407515180622463684;
                            }
                        }
                        11 => {
                            if !(self.callbacks).is_null() {
                                if ((*self.callbacks).yajl_number).is_some() {
                                    if ((*self.callbacks).yajl_number)
                                        .expect("non-null function pointer")(
                                        self.ctx,
                                        buf as *const libc::c_char,
                                        bufLen,
                                    ) == 0
                                    {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError =
                                            b"client cancelled parse via callback return value\0"
                                                as *const u8
                                                as *const libc::c_char;
                                        return Status::ClientCanceled;
                                    }
                                } else if ((*self.callbacks).yajl_double).is_some() {
                                    let mut d: libc::c_double = 0.0f64;
                                    yajl_buf_clear(self.decodeBuf);
                                    yajl_buf_append(
                                        self.decodeBuf,
                                        buf as *const libc::c_void,
                                        bufLen,
                                    );
                                    buf = yajl_buf_data(self.decodeBuf);
                                    set_last_error(0);
                                    d = libc::strtod(
                                        buf as *mut libc::c_char,
                                        std::ptr::null_mut::<*mut libc::c_char>(),
                                    );
                                    if d.is_infinite() && get_last_error() == 34 as libc::c_int {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError = b"numeric (floating point) overflow\0"
                                            as *const u8
                                            as *const libc::c_char;
                                        if *offset >= bufLen {
                                            *offset = { *offset }.wrapping_sub(bufLen);
                                        } else {
                                            *offset = 0 as libc::c_int as usize;
                                        }
                                        continue;
                                    } else if ((*self.callbacks).yajl_double)
                                        .expect("non-null function pointer")(
                                        self.ctx, d
                                    ) == 0
                                    {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError =
                                            b"client cancelled parse via callback return value\0"
                                                as *const u8
                                                as *const libc::c_char;
                                        return Status::ClientCanceled;
                                    }
                                }
                                current_block = 6407515180622463684;
                            } else {
                                current_block = 6407515180622463684;
                            }
                        }
                        8 => {
                            if self.stateStack.top() == ParseState::ArrayStart {
                                if !(self.callbacks).is_null()
                                    && ((*self.callbacks).yajl_end_array).is_some()
                                    && ((*self.callbacks).yajl_end_array)
                                        .expect("non-null function pointer")(
                                        self.ctx
                                    ) == 0
                                {
                                    *self.stateStack.top_mut() = ParseState::ParseError;
                                    self.parseError =
                                        b"client cancelled parse via callback return value\0"
                                            as *const u8
                                            as *const libc::c_char;
                                    return Status::ClientCanceled;
                                }
                                self.stateStack.pop();
                                continue;
                            } else {
                                current_block = 13495271385072242379;
                            }
                        }
                        1 | 2 | 9 => {
                            current_block = 13495271385072242379;
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = b"invalid token, internal error\0" as *const u8
                                as *const libc::c_char;
                            continue;
                        }
                    }
                    match current_block {
                        6407515180622463684 => {
                            let mut s = self.stateStack.top();
                            if s == ParseState::Start || s == ParseState::GotValue {
                                *self.stateStack.top_mut() = ParseState::ParseComplete;
                            } else if s == ParseState::MapNeedVal {
                                *self.stateStack.top_mut() = ParseState::MapGotVal;
                            } else {
                                *self.stateStack.top_mut() = ParseState::ArrayGotVal;
                            }
                            if stateToPush != ParseState::Start {
                                self.stateStack.push(stateToPush);
                            }
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = b"unallowed token at this point in JSON text\0"
                                as *const u8
                                as *const libc::c_char;
                        }
                    }
                }
                ParseState::MapStart | ParseState::MapNeedKey => {
                    tok = yajl_lex_lex(
                        self.lexer,
                        jsonText,
                        jsonTextLen,
                        offset,
                        &mut buf,
                        &mut bufLen,
                    );
                    match tok as libc::c_uint {
                        3 => return Status::Ok,
                        4 => {
                            *self.stateStack.top_mut() = ParseState::LexicalError;
                            continue;
                        }
                        13 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_map_key).is_some()
                            {
                                yajl_buf_clear(self.decodeBuf);
                                yajl_string_decode(self.decodeBuf, buf, bufLen);
                                buf = yajl_buf_data(self.decodeBuf);
                                bufLen = yajl_buf_len(self.decodeBuf);
                            }
                            current_block = 5544887021832600539;
                        }
                        12 => {
                            current_block = 5544887021832600539;
                        }
                        9 => {
                            if self.stateStack.top() == ParseState::MapStart {
                                if !(self.callbacks).is_null()
                                    && ((*self.callbacks).yajl_end_map).is_some()
                                    && ((*self.callbacks).yajl_end_map)
                                        .expect("non-null function pointer")(
                                        self.ctx
                                    ) == 0
                                {
                                    *self.stateStack.top_mut() = ParseState::ParseError;
                                    self.parseError =
                                        b"client cancelled parse via callback return value\0"
                                            as *const u8
                                            as *const libc::c_char;
                                    return Status::ClientCanceled;
                                }
                                self.stateStack.pop();
                                continue;
                            } else {
                                current_block = 17513148302838498461;
                            }
                        }
                        _ => {
                            current_block = 17513148302838498461;
                        }
                    }
                    match current_block {
                        5544887021832600539 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_map_key).is_some()
                                && ((*self.callbacks).yajl_map_key)
                                    .expect("non-null function pointer")(
                                    self.ctx, buf, bufLen
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const libc::c_char;
                                return Status::ClientCanceled;
                            }
                            *self.stateStack.top_mut() = ParseState::MapSep;
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = b"invalid object key (must be a string)\0"
                                as *const u8
                                as *const libc::c_char;
                        }
                    }
                }
                ParseState::MapSep => {
                    tok = yajl_lex_lex(
                        self.lexer,
                        jsonText,
                        jsonTextLen,
                        offset,
                        &mut buf,
                        &mut bufLen,
                    );
                    match tok as libc::c_uint {
                        1 => {
                            *self.stateStack.top_mut() = ParseState::MapNeedVal;
                        }
                        3 => return Status::Ok,
                        4 => {
                            *self.stateStack.top_mut() = ParseState::LexicalError;
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError =
                                b"object key and value must be separated by a colon (':')\0"
                                    as *const u8
                                    as *const libc::c_char;
                        }
                    }
                }
                ParseState::MapGotVal => {
                    tok = yajl_lex_lex(
                        self.lexer,
                        jsonText,
                        jsonTextLen,
                        offset,
                        &mut buf,
                        &mut bufLen,
                    );
                    match tok as libc::c_uint {
                        9 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_end_map).is_some()
                                && ((*self.callbacks).yajl_end_map)
                                    .expect("non-null function pointer")(
                                    self.ctx
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const libc::c_char;
                                return Status::ClientCanceled;
                            }
                            self.stateStack.pop();
                        }
                        2 => {
                            *self.stateStack.top_mut() = ParseState::MapNeedKey;
                        }
                        3 => return Status::Ok,
                        4 => {
                            *self.stateStack.top_mut() = ParseState::LexicalError;
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError =
                                b"after key and value, inside map, I expect ',' or '}'\0"
                                    as *const u8
                                    as *const libc::c_char;
                            if *offset >= bufLen {
                                *offset = { *offset }.wrapping_sub(bufLen);
                            } else {
                                *offset = 0 as libc::c_int as usize;
                            }
                        }
                    }
                }
                ParseState::ArrayGotVal => {
                    tok = yajl_lex_lex(
                        self.lexer,
                        jsonText,
                        jsonTextLen,
                        offset,
                        &mut buf,
                        &mut bufLen,
                    );
                    match tok as libc::c_uint {
                        8 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_end_array).is_some()
                                && ((*self.callbacks).yajl_end_array)
                                    .expect("non-null function pointer")(
                                    self.ctx
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const libc::c_char;
                                return Status::ClientCanceled;
                            }
                            self.stateStack.pop();
                        }
                        2 => {
                            *self.stateStack.top_mut() = ParseState::ArrayNeedVal;
                        }
                        3 => return Status::Ok,
                        4 => {
                            *self.stateStack.top_mut() = ParseState::LexicalError;
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = b"after array element, I expect ',' or ']'\0"
                                as *const u8
                                as *const libc::c_char;
                        }
                    }
                }
            }
        }
        Status::Ok
    }
}
