use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};
use core::ptr;

use crate::yajl_alloc::yajl_alloc_funcs;
use crate::yajl_buf::yajl_buf_t;

use crate::yajl_buf::yajl_buf_alloc;
use crate::yajl_buf::yajl_buf_append;
use crate::yajl_buf::yajl_buf_clear;
use crate::yajl_buf::yajl_buf_data;
use crate::yajl_buf::yajl_buf_free;
use crate::yajl_buf::yajl_buf_len;
use crate::yajl_buf::yajl_buf_truncate;

pub type yajl_tok = c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_lexer_t {
    pub lineOff: usize,
    pub charOff: usize,
    pub error: yajl_lex_error,
    pub buf: yajl_buf,
    pub bufOff: usize,
    pub bufInUse: c_uint,
    pub allowComments: c_uint,
    pub validateUTF8: c_uint,
    pub alloc: *mut yajl_alloc_funcs,
}
pub type yajl_buf = *mut yajl_buf_t;
pub type yajl_lex_error = c_uint;
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
pub type yajl_lexer = *mut yajl_lexer_t;

pub unsafe extern "C" fn yajl_lex_alloc(
    mut alloc: *mut yajl_alloc_funcs,
    mut allowComments: c_uint,
    mut validateUTF8: c_uint,
) -> yajl_lexer {
    let mut lxr: yajl_lexer = ((*alloc).malloc).expect("non-null function pointer")(
        (*alloc).ctx,
        ::core::mem::size_of::<yajl_lexer_t>(),
    ) as yajl_lexer;
    ptr::write_bytes(lxr, 0, 1);

    (*lxr).buf = yajl_buf_alloc(alloc);
    (*lxr).allowComments = allowComments;
    (*lxr).validateUTF8 = validateUTF8;
    (*lxr).alloc = alloc;
    lxr
}

pub unsafe extern "C" fn yajl_lex_free(mut lxr: yajl_lexer) {
    yajl_buf_free((*lxr).buf);
    ((*(*lxr).alloc).free).expect("non-null function pointer")(
        (*(*lxr).alloc).ctx,
        lxr as *mut c_void,
    );
}
static mut charLookupTable: [c_char; 256] = [
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0,
    0,
    (0x8 | 0x1 | 0x2) as c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x1,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    (0x8 | 0x1 | 0x2) as c_char,
    0,
    0,
    0,
    0,
    0x4,
    (0x1 | 0x4) as c_char,
    0x4,
    0x4,
    0x4,
    (0x1 | 0x4) as c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x1,
    0,
    0,
    0,
    0x1,
    0,
    0x1,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
];
unsafe extern "C" fn yajl_lex_utf8_char(
    mut lexer: yajl_lexer,
    mut jsonText: *const c_uchar,
    mut jsonTextLen: usize,
    mut offset: *mut usize,
    mut curChar: c_uchar,
) -> yajl_tok {
    if curChar <= 0x7f {
        return yajl_tok_string;
    } else if curChar as c_int >> 5 as c_int == 0x6 as c_int {
        if *offset >= jsonTextLen {
            return yajl_tok_eof;
        }
        curChar = (if (*lexer).bufInUse != 0
            && yajl_buf_len((*lexer).buf) != 0
            && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
        {
            let fresh0 = (*lexer).bufOff;
            (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
            *(yajl_buf_data((*lexer).buf)).add(fresh0) as c_int
        } else {
            let fresh1 = *offset;
            *offset = (*offset).wrapping_add(1);
            *jsonText.add(fresh1) as c_int
        }) as c_uchar;
        if curChar as c_int >> 6 as c_int == 0x2 as c_int {
            return yajl_tok_string;
        }
    } else if curChar as c_int >> 4 as c_int == 0xe as c_int {
        if *offset >= jsonTextLen {
            return yajl_tok_eof;
        }
        curChar = (if (*lexer).bufInUse != 0
            && yajl_buf_len((*lexer).buf) != 0
            && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
        {
            let fresh2 = (*lexer).bufOff;
            (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
            *(yajl_buf_data((*lexer).buf)).add(fresh2) as c_int
        } else {
            let fresh3 = *offset;
            *offset = (*offset).wrapping_add(1);
            *jsonText.add(fresh3) as c_int
        }) as c_uchar;
        if curChar as c_int >> 6 as c_int == 0x2 as c_int {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            curChar = (if (*lexer).bufInUse != 0
                && yajl_buf_len((*lexer).buf) != 0
                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
            {
                let fresh4 = (*lexer).bufOff;
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                *(yajl_buf_data((*lexer).buf)).add(fresh4) as c_int
            } else {
                let fresh5 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh5) as c_int
            }) as c_uchar;
            if curChar as c_int >> 6 as c_int == 0x2 as c_int {
                return yajl_tok_string;
            }
        }
    } else if curChar as c_int >> 3 as c_int == 0x1e as c_int {
        if *offset >= jsonTextLen {
            return yajl_tok_eof;
        }
        curChar = (if (*lexer).bufInUse != 0
            && yajl_buf_len((*lexer).buf) != 0
            && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
        {
            let fresh6 = (*lexer).bufOff;
            (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
            *(yajl_buf_data((*lexer).buf)).add(fresh6) as c_int
        } else {
            let fresh7 = *offset;
            *offset = (*offset).wrapping_add(1);
            *jsonText.add(fresh7) as c_int
        }) as c_uchar;
        if curChar as c_int >> 6 as c_int == 0x2 as c_int {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            curChar = (if (*lexer).bufInUse != 0
                && yajl_buf_len((*lexer).buf) != 0
                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
            {
                let fresh8 = (*lexer).bufOff;
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                *(yajl_buf_data((*lexer).buf)).add(fresh8) as c_int
            } else {
                let fresh9 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh9) as c_int
            }) as c_uchar;
            if curChar as c_int >> 6 as c_int == 0x2 as c_int {
                if *offset >= jsonTextLen {
                    return yajl_tok_eof;
                }
                curChar = (if (*lexer).bufInUse != 0
                    && yajl_buf_len((*lexer).buf) != 0
                    && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
                {
                    let fresh10 = (*lexer).bufOff;
                    (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                    *(yajl_buf_data((*lexer).buf)).add(fresh10) as c_int
                } else {
                    let fresh11 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh11) as c_int
                }) as c_uchar;
                if curChar as c_int >> 6 as c_int == 0x2 as c_int {
                    return yajl_tok_string;
                }
            }
        }
    }
    yajl_tok_error
}
unsafe extern "C" fn yajl_string_scan(
    mut buf: *const c_uchar,
    mut len: usize,
    mut utf8check: c_int,
) -> usize {
    let mut mask: c_uchar = (0x2 as c_int
        | 0x8 as c_int
        | (if utf8check != 0 {
            0x10 as c_int
        } else {
            0 as c_int
        })) as c_uchar;
    let mut skip: usize = 0 as c_int as usize;
    while skip < len && charLookupTable[*buf as usize] as c_int & mask as c_int == 0 {
        skip = skip.wrapping_add(1);
        buf = buf.offset(1);
    }
    skip
}
unsafe extern "C" fn yajl_lex_string(
    mut lexer: yajl_lexer,
    mut jsonText: *const c_uchar,
    mut jsonTextLen: usize,
    mut offset: *mut usize,
) -> yajl_tok {
    let mut tok: yajl_tok = yajl_tok_error;
    let mut hasEscapes: c_int = 0 as c_int;
    's_10: loop {
        let mut curChar: c_uchar = 0;
        let mut p: *const c_uchar = ptr::null::<c_uchar>();
        let mut len: usize = 0;
        if (*lexer).bufInUse != 0
            && yajl_buf_len((*lexer).buf) != 0
            && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
        {
            p = (yajl_buf_data((*lexer).buf)).add((*lexer).bufOff);
            len = (yajl_buf_len((*lexer).buf)).wrapping_sub((*lexer).bufOff);
            (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(yajl_string_scan(
                p,
                len,
                (*lexer).validateUTF8 as c_int,
            )) as usize;
        } else if *offset < jsonTextLen {
            p = jsonText.add(*offset);
            len = jsonTextLen.wrapping_sub(*offset);
            *offset =
                (*offset).wrapping_add(yajl_string_scan(p, len, (*lexer).validateUTF8 as c_int))
                    as usize;
        }
        if *offset >= jsonTextLen {
            tok = yajl_tok_eof;
            break;
        } else {
            curChar = (if (*lexer).bufInUse != 0
                && yajl_buf_len((*lexer).buf) != 0
                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
            {
                let fresh12 = (*lexer).bufOff;
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                *(yajl_buf_data((*lexer).buf)).add(fresh12) as c_int
            } else {
                let fresh13 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh13) as c_int
            }) as c_uchar;
            if curChar as c_int == '"' as i32 {
                tok = yajl_tok_string;
                break;
            } else if curChar as c_int == '\\' as i32 {
                hasEscapes = 1 as c_int;
                if *offset >= jsonTextLen {
                    tok = yajl_tok_eof;
                    break;
                } else {
                    curChar = (if (*lexer).bufInUse != 0
                        && yajl_buf_len((*lexer).buf) != 0
                        && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
                    {
                        let fresh14 = (*lexer).bufOff;
                        (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                        *(yajl_buf_data((*lexer).buf)).add(fresh14) as c_int
                    } else {
                        let fresh15 = *offset;
                        *offset = (*offset).wrapping_add(1);
                        *jsonText.add(fresh15) as c_int
                    }) as c_uchar;
                    if curChar as c_int == 'u' as i32 {
                        let mut i: c_uint = 0 as c_int as c_uint;
                        i = 0 as c_int as c_uint;
                        while i < 4 as c_int as c_uint {
                            if *offset >= jsonTextLen {
                                tok = yajl_tok_eof;
                                break 's_10;
                            } else {
                                curChar = (if (*lexer).bufInUse != 0
                                    && yajl_buf_len((*lexer).buf) != 0
                                    && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
                                {
                                    let fresh16 = (*lexer).bufOff;
                                    (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                                    *(yajl_buf_data((*lexer).buf)).add(fresh16) as c_int
                                } else {
                                    let fresh17 = *offset;
                                    *offset = (*offset).wrapping_add(1);
                                    *jsonText.add(fresh17) as c_int
                                }) as c_uchar;
                                if charLookupTable[curChar as usize] as c_int & 0x4 as c_int == 0 {
                                    if *offset > 0 {
                                        *offset = (*offset).wrapping_sub(1);
                                    } else {
                                        (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
                                    };
                                    (*lexer).error = yajl_lex_string_invalid_hex_char;
                                    break 's_10;
                                } else {
                                    i = i.wrapping_add(1);
                                }
                            }
                        }
                    } else {
                        if charLookupTable[curChar as usize] as c_int & 0x1 as c_int != 0 {
                            continue;
                        }
                        if *offset > 0 {
                            *offset = (*offset).wrapping_sub(1);
                        } else {
                            (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
                        };
                        (*lexer).error = yajl_lex_string_invalid_escaped_char;
                        break;
                    }
                }
            } else if charLookupTable[curChar as usize] as c_int & 0x2 as c_int != 0 {
                if *offset > 0 {
                    *offset = (*offset).wrapping_sub(1);
                } else {
                    (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
                };
                (*lexer).error = yajl_lex_string_invalid_json_char;
                break;
            } else {
                if (*lexer).validateUTF8 == 0 {
                    continue;
                }
                let mut t: yajl_tok =
                    yajl_lex_utf8_char(lexer, jsonText, jsonTextLen, offset, curChar);
                if t as c_uint == yajl_tok_eof as c_int as c_uint {
                    tok = yajl_tok_eof;
                    break;
                } else {
                    if t as c_uint != yajl_tok_error as c_int as c_uint {
                        continue;
                    }
                    (*lexer).error = yajl_lex_string_invalid_utf8;
                    break;
                }
            }
        }
    }
    if hasEscapes != 0 && tok as c_uint == yajl_tok_string as c_int as c_uint {
        tok = yajl_tok_string_with_escapes;
    }
    tok
}
unsafe extern "C" fn yajl_lex_number(
    mut lexer: yajl_lexer,
    mut jsonText: *const c_uchar,
    mut jsonTextLen: usize,
    mut offset: *mut usize,
) -> yajl_tok {
    let mut c: c_uchar = 0;
    let mut tok: yajl_tok = yajl_tok_integer;
    if *offset >= jsonTextLen {
        return yajl_tok_eof;
    }
    c = (if (*lexer).bufInUse != 0
        && yajl_buf_len((*lexer).buf) != 0
        && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
    {
        let fresh18 = (*lexer).bufOff;
        (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
        *(yajl_buf_data((*lexer).buf)).add(fresh18) as c_int
    } else {
        let fresh19 = *offset;
        *offset = (*offset).wrapping_add(1);
        *jsonText.add(fresh19) as c_int
    }) as c_uchar;
    if c as c_int == '-' as i32 {
        if *offset >= jsonTextLen {
            return yajl_tok_eof;
        }
        c = (if (*lexer).bufInUse != 0
            && yajl_buf_len((*lexer).buf) != 0
            && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
        {
            let fresh20 = (*lexer).bufOff;
            (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
            *(yajl_buf_data((*lexer).buf)).add(fresh20) as c_int
        } else {
            let fresh21 = *offset;
            *offset = (*offset).wrapping_add(1);
            *jsonText.add(fresh21) as c_int
        }) as c_uchar;
    }
    if c as c_int == '0' as i32 {
        if *offset >= jsonTextLen {
            return yajl_tok_eof;
        }
        c = (if (*lexer).bufInUse != 0
            && yajl_buf_len((*lexer).buf) != 0
            && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
        {
            let fresh22 = (*lexer).bufOff;
            (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
            *(yajl_buf_data((*lexer).buf)).add(fresh22) as c_int
        } else {
            let fresh23 = *offset;
            *offset = (*offset).wrapping_add(1);
            *jsonText.add(fresh23) as c_int
        }) as c_uchar;
    } else if c as c_int >= '1' as i32 && c as c_int <= '9' as i32 {
        loop {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            c = (if (*lexer).bufInUse != 0
                && yajl_buf_len((*lexer).buf) != 0
                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
            {
                let fresh24 = (*lexer).bufOff;
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                *(yajl_buf_data((*lexer).buf)).add(fresh24) as c_int
            } else {
                let fresh25 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh25) as c_int
            }) as c_uchar;
            if !(c as c_int >= '0' as i32 && c as c_int <= '9' as i32) {
                break;
            }
        }
    } else {
        if *offset > 0 {
            *offset = (*offset).wrapping_sub(1);
        } else {
            (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
        };
        (*lexer).error = yajl_lex_missing_integer_after_minus;
        return yajl_tok_error;
    }
    if c as c_int == '.' as i32 {
        let mut numRd: c_int = 0 as c_int;
        if *offset >= jsonTextLen {
            return yajl_tok_eof;
        }
        c = (if (*lexer).bufInUse != 0
            && yajl_buf_len((*lexer).buf) != 0
            && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
        {
            let fresh26 = (*lexer).bufOff;
            (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
            *(yajl_buf_data((*lexer).buf)).add(fresh26) as c_int
        } else {
            let fresh27 = *offset;
            *offset = (*offset).wrapping_add(1);
            *jsonText.add(fresh27) as c_int
        }) as c_uchar;
        while c as c_int >= '0' as i32 && c as c_int <= '9' as i32 {
            numRd += 1;
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            c = (if (*lexer).bufInUse != 0
                && yajl_buf_len((*lexer).buf) != 0
                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
            {
                let fresh28 = (*lexer).bufOff;
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                *(yajl_buf_data((*lexer).buf)).add(fresh28) as c_int
            } else {
                let fresh29 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh29) as c_int
            }) as c_uchar;
        }
        if numRd == 0 {
            if *offset > 0 {
                *offset = (*offset).wrapping_sub(1);
            } else {
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
            };
            (*lexer).error = yajl_lex_missing_integer_after_decimal;
            return yajl_tok_error;
        }
        tok = yajl_tok_double;
    }
    if c as c_int == 'e' as i32 || c as c_int == 'E' as i32 {
        if *offset >= jsonTextLen {
            return yajl_tok_eof;
        }
        c = (if (*lexer).bufInUse != 0
            && yajl_buf_len((*lexer).buf) != 0
            && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
        {
            let fresh30 = (*lexer).bufOff;
            (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
            *(yajl_buf_data((*lexer).buf)).add(fresh30) as c_int
        } else {
            let fresh31 = *offset;
            *offset = (*offset).wrapping_add(1);
            *jsonText.add(fresh31) as c_int
        }) as c_uchar;
        if c as c_int == '+' as i32 || c as c_int == '-' as i32 {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            c = (if (*lexer).bufInUse != 0
                && yajl_buf_len((*lexer).buf) != 0
                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
            {
                let fresh32 = (*lexer).bufOff;
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                *(yajl_buf_data((*lexer).buf)).add(fresh32) as c_int
            } else {
                let fresh33 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh33) as c_int
            }) as c_uchar;
        }
        if c as c_int >= '0' as i32 && c as c_int <= '9' as i32 {
            loop {
                if *offset >= jsonTextLen {
                    return yajl_tok_eof;
                }
                c = (if (*lexer).bufInUse != 0
                    && yajl_buf_len((*lexer).buf) != 0
                    && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
                {
                    let fresh34 = (*lexer).bufOff;
                    (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                    *(yajl_buf_data((*lexer).buf)).add(fresh34) as c_int
                } else {
                    let fresh35 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh35) as c_int
                }) as c_uchar;
                if !(c as c_int >= '0' as i32 && c as c_int <= '9' as i32) {
                    break;
                }
            }
        } else {
            if *offset > 0 {
                *offset = (*offset).wrapping_sub(1);
            } else {
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
            };
            (*lexer).error = yajl_lex_missing_integer_after_exponent;
            return yajl_tok_error;
        }
        tok = yajl_tok_double;
    }
    if *offset > 0 {
        *offset = (*offset).wrapping_sub(1);
    } else {
        (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
    };
    tok
}
unsafe extern "C" fn yajl_lex_comment(
    mut lexer: yajl_lexer,
    mut jsonText: *const c_uchar,
    mut jsonTextLen: usize,
    mut offset: *mut usize,
) -> yajl_tok {
    let mut c: c_uchar = 0;
    let mut tok: yajl_tok = yajl_tok_comment;
    if *offset >= jsonTextLen {
        return yajl_tok_eof;
    }
    c = (if (*lexer).bufInUse != 0
        && yajl_buf_len((*lexer).buf) != 0
        && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
    {
        let fresh36 = (*lexer).bufOff;
        (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
        *(yajl_buf_data((*lexer).buf)).add(fresh36) as c_int
    } else {
        let fresh37 = *offset;
        *offset = (*offset).wrapping_add(1);
        *jsonText.add(fresh37) as c_int
    }) as c_uchar;
    if c as c_int == '/' as i32 {
        loop {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            c = (if (*lexer).bufInUse != 0
                && yajl_buf_len((*lexer).buf) != 0
                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
            {
                let fresh38 = (*lexer).bufOff;
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                *(yajl_buf_data((*lexer).buf)).add(fresh38) as c_int
            } else {
                let fresh39 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh39) as c_int
            }) as c_uchar;
            if c as c_int == '\n' as i32 {
                break;
            }
        }
    } else if c as c_int == '*' as i32 {
        loop {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            c = (if (*lexer).bufInUse != 0
                && yajl_buf_len((*lexer).buf) != 0
                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
            {
                let fresh40 = (*lexer).bufOff;
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                *(yajl_buf_data((*lexer).buf)).add(fresh40) as c_int
            } else {
                let fresh41 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh41) as c_int
            }) as c_uchar;
            if c as c_int != '*' as i32 {
                continue;
            }
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            c = (if (*lexer).bufInUse != 0
                && yajl_buf_len((*lexer).buf) != 0
                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
            {
                let fresh42 = (*lexer).bufOff;
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                *(yajl_buf_data((*lexer).buf)).add(fresh42) as c_int
            } else {
                let fresh43 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh43) as c_int
            }) as c_uchar;
            if c as c_int == '/' as i32 {
                break;
            }
            if *offset > 0 {
                *offset = (*offset).wrapping_sub(1);
            } else {
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
            };
        }
    } else {
        (*lexer).error = yajl_lex_invalid_char;
        tok = yajl_tok_error;
    }
    tok
}

pub unsafe extern "C" fn yajl_lex_lex(
    mut lexer: yajl_lexer,
    mut jsonText: *const c_uchar,
    mut jsonTextLen: usize,
    mut offset: *mut usize,
    mut outBuf: *mut *const c_uchar,
    mut outLen: *mut usize,
) -> yajl_tok {
    let mut tok: yajl_tok = yajl_tok_error;
    let mut c: c_uchar = 0;
    let mut startOffset: usize = *offset;
    *outBuf = ptr::null::<c_uchar>();
    *outLen = 0 as c_int as usize;
    's_21: loop {
        if *offset >= jsonTextLen {
            tok = yajl_tok_eof;
            break;
        } else {
            c = (if (*lexer).bufInUse != 0
                && yajl_buf_len((*lexer).buf) != 0
                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
            {
                let fresh44 = (*lexer).bufOff;
                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                *(yajl_buf_data((*lexer).buf)).add(fresh44) as c_int
            } else {
                let fresh45 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh45) as c_int
            }) as c_uchar;
            match c as c_int {
                123 => {
                    tok = yajl_tok_left_bracket;
                    break;
                }
                125 => {
                    tok = yajl_tok_right_bracket;
                    break;
                }
                91 => {
                    tok = yajl_tok_left_brace;
                    break;
                }
                93 => {
                    tok = yajl_tok_right_brace;
                    break;
                }
                44 => {
                    tok = yajl_tok_comma;
                    break;
                }
                58 => {
                    tok = yajl_tok_colon;
                    break;
                }
                9 | 10 | 11 | 12 | 13 | 32 => {
                    startOffset = startOffset.wrapping_add(1);
                }
                116 => {
                    let mut want: *const c_char = b"rue\0" as *const u8 as *const c_char;
                    loop {
                        if *offset >= jsonTextLen {
                            tok = yajl_tok_eof;
                            break 's_21;
                        } else {
                            c = (if (*lexer).bufInUse != 0
                                && yajl_buf_len((*lexer).buf) != 0
                                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
                            {
                                let fresh46 = (*lexer).bufOff;
                                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                                *(yajl_buf_data((*lexer).buf)).add(fresh46) as c_int
                            } else {
                                let fresh47 = *offset;
                                *offset = (*offset).wrapping_add(1);
                                *jsonText.add(fresh47) as c_int
                            }) as c_uchar;
                            if c as c_int != *want as c_int {
                                if *offset > 0 {
                                    *offset = (*offset).wrapping_sub(1);
                                } else {
                                    (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
                                };
                                (*lexer).error = yajl_lex_invalid_string;
                                tok = yajl_tok_error;
                                break 's_21;
                            } else {
                                want = want.offset(1);
                                if *want == 0 {
                                    break;
                                }
                            }
                        }
                    }
                    tok = yajl_tok_bool;
                    break;
                }
                102 => {
                    let mut want_0: *const c_char = b"alse\0" as *const u8 as *const c_char;
                    loop {
                        if *offset >= jsonTextLen {
                            tok = yajl_tok_eof;
                            break 's_21;
                        } else {
                            c = (if (*lexer).bufInUse != 0
                                && yajl_buf_len((*lexer).buf) != 0
                                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
                            {
                                let fresh48 = (*lexer).bufOff;
                                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                                *(yajl_buf_data((*lexer).buf)).add(fresh48) as c_int
                            } else {
                                let fresh49 = *offset;
                                *offset = (*offset).wrapping_add(1);
                                *jsonText.add(fresh49) as c_int
                            }) as c_uchar;
                            if c as c_int != *want_0 as c_int {
                                if *offset > 0 {
                                    *offset = (*offset).wrapping_sub(1);
                                } else {
                                    (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
                                };
                                (*lexer).error = yajl_lex_invalid_string;
                                tok = yajl_tok_error;
                                break 's_21;
                            } else {
                                want_0 = want_0.offset(1);
                                if *want_0 == 0 {
                                    break;
                                }
                            }
                        }
                    }
                    tok = yajl_tok_bool;
                    break;
                }
                110 => {
                    let mut want_1: *const c_char = b"ull\0" as *const u8 as *const c_char;
                    loop {
                        if *offset >= jsonTextLen {
                            tok = yajl_tok_eof;
                            break 's_21;
                        } else {
                            c = (if (*lexer).bufInUse != 0
                                && yajl_buf_len((*lexer).buf) != 0
                                && (*lexer).bufOff < yajl_buf_len((*lexer).buf)
                            {
                                let fresh50 = (*lexer).bufOff;
                                (*lexer).bufOff = ((*lexer).bufOff).wrapping_add(1);
                                *(yajl_buf_data((*lexer).buf)).add(fresh50) as c_int
                            } else {
                                let fresh51 = *offset;
                                *offset = (*offset).wrapping_add(1);
                                *jsonText.add(fresh51) as c_int
                            }) as c_uchar;
                            if c as c_int != *want_1 as c_int {
                                if *offset > 0 {
                                    *offset = (*offset).wrapping_sub(1);
                                } else {
                                    (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
                                };
                                (*lexer).error = yajl_lex_invalid_string;
                                tok = yajl_tok_error;
                                break 's_21;
                            } else {
                                want_1 = want_1.offset(1);
                                if *want_1 == 0 {
                                    break;
                                }
                            }
                        }
                    }
                    tok = yajl_tok_null;
                    break;
                }
                34 => {
                    tok = yajl_lex_string(lexer, jsonText, jsonTextLen, offset);
                    break;
                }
                45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    if *offset > 0 {
                        *offset = (*offset).wrapping_sub(1);
                    } else {
                        (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
                    };
                    tok = yajl_lex_number(lexer, jsonText, jsonTextLen, offset);
                    break;
                }
                47 => {
                    if (*lexer).allowComments == 0 {
                        if *offset > 0 {
                            *offset = (*offset).wrapping_sub(1);
                        } else {
                            (*lexer).bufOff = ((*lexer).bufOff).wrapping_sub(1);
                        };
                        (*lexer).error = yajl_lex_unallowed_comment;
                        tok = yajl_tok_error;
                        break;
                    } else {
                        tok = yajl_lex_comment(lexer, jsonText, jsonTextLen, offset);
                        if tok as c_uint != yajl_tok_comment as c_int as c_uint {
                            break;
                        }
                        tok = yajl_tok_error;
                        yajl_buf_clear((*lexer).buf);
                        (*lexer).bufInUse = 0 as c_int as c_uint;
                        startOffset = *offset;
                    }
                }
                _ => {
                    (*lexer).error = yajl_lex_invalid_char;
                    tok = yajl_tok_error;
                    break;
                }
            }
        }
    }
    if tok as c_uint == yajl_tok_eof as c_int as c_uint || (*lexer).bufInUse != 0 {
        if (*lexer).bufInUse == 0 {
            yajl_buf_clear((*lexer).buf);
        }
        (*lexer).bufInUse = 1 as c_int as c_uint;
        yajl_buf_append(
            (*lexer).buf,
            jsonText.add(startOffset) as *const c_void,
            (*offset).wrapping_sub(startOffset),
        );
        (*lexer).bufOff = 0 as c_int as usize;
        if tok as c_uint != yajl_tok_eof as c_int as c_uint {
            *outBuf = yajl_buf_data((*lexer).buf);
            *outLen = yajl_buf_len((*lexer).buf);
            (*lexer).bufInUse = 0 as c_int as c_uint;
        }
    } else if tok as c_uint != yajl_tok_error as c_int as c_uint {
        *outBuf = jsonText.add(startOffset);
        *outLen = (*offset).wrapping_sub(startOffset);
    }
    if tok as c_uint == yajl_tok_string as c_int as c_uint
        || tok as c_uint == yajl_tok_string_with_escapes as c_int as c_uint
    {
        *outBuf = (*outBuf).offset(1);
        *outLen = { *outLen }.wrapping_sub(2 as c_int as usize);
    }
    tok
}

pub unsafe extern "C" fn yajl_lex_error_to_string(mut error: yajl_lex_error) -> *const c_char {
    match error as c_uint {
        0 => return b"ok, no error\0" as *const u8 as *const c_char,
        1 => {
            return b"invalid bytes in UTF8 string.\0" as *const u8 as *const c_char;
        }
        2 => {
            return b"inside a string, '\\' occurs before a character which it may not.\0"
                as *const u8 as *const c_char;
        }
        3 => {
            return b"invalid character inside string.\0" as *const u8 as *const c_char;
        }
        4 => {
            return b"invalid (non-hex) character occurs after '\\u' inside string.\0" as *const u8
                as *const c_char;
        }
        5 => return b"invalid char in json text.\0" as *const u8 as *const c_char,
        6 => return b"invalid string in json text.\0" as *const u8 as *const c_char,
        8 => {
            return b"malformed number, a digit is required after the exponent.\0" as *const u8
                as *const c_char;
        }
        7 => {
            return b"malformed number, a digit is required after the decimal point.\0" as *const u8
                as *const c_char;
        }
        9 => {
            return b"malformed number, a digit is required after the minus sign.\0" as *const u8
                as *const c_char;
        }
        10 => {
            return b"probable comment found in input text, comments are not enabled.\0" as *const u8
                as *const c_char;
        }
        _ => {}
    }
    b"unknown error code\0" as *const u8 as *const c_char
}

pub unsafe extern "C" fn yajl_lex_get_error(mut lexer: yajl_lexer) -> yajl_lex_error {
    if lexer.is_null() {
        return 4294967295 as yajl_lex_error;
    }
    (*lexer).error
}

pub unsafe extern "C" fn yajl_lex_current_line(mut lexer: yajl_lexer) -> usize {
    (*lexer).lineOff
}

pub unsafe extern "C" fn yajl_lex_current_char(mut lexer: yajl_lexer) -> usize {
    (*lexer).charOff
}

pub unsafe extern "C" fn yajl_lex_peek(
    mut lexer: yajl_lexer,
    mut jsonText: *const c_uchar,
    mut jsonTextLen: usize,
    mut offset: usize,
) -> yajl_tok {
    let mut outBuf: *const c_uchar = ptr::null::<c_uchar>();
    let mut outLen: usize = 0;
    let mut bufLen: usize = yajl_buf_len((*lexer).buf);
    let mut bufOff: usize = (*lexer).bufOff;
    let mut bufInUse: c_uint = (*lexer).bufInUse;
    let mut tok: yajl_tok = yajl_tok_bool;
    tok = yajl_lex_lex(
        lexer,
        jsonText,
        jsonTextLen,
        &mut offset,
        &mut outBuf,
        &mut outLen,
    );
    (*lexer).bufOff = bufOff;
    (*lexer).bufInUse = bufInUse;
    yajl_buf_truncate((*lexer).buf, bufLen);
    tok
}
