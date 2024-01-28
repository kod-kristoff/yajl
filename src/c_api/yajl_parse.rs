#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(non_snake_case)]
/*
 * Copyright (c) 2007-2014, Lloyd Hilaiel <me@lloyd.io>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

/**
 * \file yajl_parse.h
 * Interface to YAJL's JSON stream parsing facilities.
 */
// #include <yajl/yajl_common.h>

// #include <stddef.h>
use std::os::raw::{c_char, c_int, c_uchar, c_void};

/** error codes returned from this interface */
#[repr(C)]
#[derive(Debug)]
pub enum yajl_status {
    /** no error was encountered */
    yajl_status_ok,
    /** a client callback returned zero, stopping the parse */
    yajl_status_client_canceled,
    /** An error occured during the parse.  Call yajl_get_error for
     *  more information about the encountered error */
    yajl_status_error,
}

/** attain a human readable, english, string for an error */
#[no_mangle]
pub extern "C" fn yajl_status_to_string(code: yajl_status) -> *const c_char {
    todo!()
}

/** an opaque handle to a parser */
#[repr(C)]
pub struct yajl_handle {
    callbacks: *const yajl_callbacks,
    // alloc: *mut yajl_alloc_funcs,
    ctx: *mut c_void,
    // _data: [u8; 0],
    // _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
// typedef struct yajl_handle_t * yajl_handle;

/** yajl is an event driven parser.  this means as json elements are
 *  parsed, you are called back to do something with the data.  The
 *  functions in this table indicate the various events for which
 *  you will be called back.  Each callback accepts a "context"
 *  pointer, this is a void * that is passed into the yajl_parse
 *  function which the client code may use to pass around context.
 *
 *  All callbacks return an integer.  If non-zero, the parse will
 *  continue.  If zero, the parse will be canceled and
 *  yajl_status_client_canceled will be returned from the parse.
 *
 *  \attention {
 *    A note about the handling of numbers:
 *
 *    yajl will only convert numbers that can be represented in a
 *    double or a 64 bit (long long) int.  All other numbers will
 *    be passed to the client in string form using the yajl_number
 *    callback.  Furthermore, if yajl_number is not NULL, it will
 *    always be used to return numbers, that is yajl_integer and
 *    yajl_double will be ignored.  If yajl_number is NULL but one
 *    of yajl_integer or yajl_double are defined, parsing of a
 *    number larger than is representable in a double or 64 bit
 *    integer will result in a parse error.
 *  }
 */
#[repr(C)]
#[derive(Debug)]
pub struct yajl_callbacks {
    pub yajl_null: extern "C" fn(ctx: *mut c_void),
    // int (* yajl_boolean)(void * ctx, int boolVal);
    // int (* yajl_integer)(void * ctx, long long integerVal);
    // int (* yajl_double)(void * ctx, double doubleVal);
    // /** A callback which passes the string representation of the number
    //  *  back to the client.  Will be used for all numbers when present */
    // int (* yajl_number)(void * ctx, const char * numberVal,
    //                     size_t numberLen);

    // /** strings are returned as pointers into the JSON text when,
    //  * possible, as a result, they are _not_ null padded */
    // int (* yajl_string)(void * ctx, const unsigned char * stringVal,
    //                     size_t stringLen);

    // int (* yajl_start_map)(void * ctx);
    // int (* yajl_map_key)(void * ctx, const unsigned char * key,
    //                      size_t stringLen);
    // int (* yajl_end_map)(void * ctx);

    // int (* yajl_start_array)(void * ctx);
    // int (* yajl_end_array)(void * ctx);
}

/** allocate a parser handle
 *  \param callbacks  a yajl callbacks structure specifying the
 *                    functions to call when different JSON entities
 *                    are encountered in the input text.  May be NULL,
 *                    which is only useful for validation.
 *  \param afs        memory allocation functions, may be NULL for to use
 *                    C runtime library routines (malloc and friends)
 *  \param ctx        a context pointer that will be passed to callbacks.
 */
#[no_mangle]
pub extern "C" fn yajl_alloc(
    callbacks: *const yajl_callbacks,
    afs: *mut c_void,
    ctx: *mut c_void,
) -> *mut yajl_handle {
    let hand = yajl_handle { callbacks, ctx };
    let box_hand = Box::new(hand);
    Box::into_raw(box_hand)
}

/** configuration parameters for the parser, these may be passed to
 *  yajl_config() along with option specific argument(s).  In general,
 *  all configuration parameters default to *off*. */
#[repr(C)]
#[derive(Debug)]
pub enum yajl_option {
    /** Ignore javascript style comments present in
     *  JSON input.  Non-standard, but rather fun
     *  arguments: toggled off with integer zero, on otherwise.
     *
     *  example:
     *    yajl_config(h, yajl_allow_comments, 1); // turn comment support on
     */
    yajl_allow_comments = 0x01,
    /**
     * When set the parser will verify that all strings in JSON input are
     * valid UTF8 and will emit a parse error if this is not so.  When set,
     * this option makes parsing slightly more expensive (~7% depending
     * on processor and compiler in use)
     *
     * example:
     *   yajl_config(h, yajl_dont_validate_strings, 1); // disable utf8 checking
     */
    yajl_dont_validate_strings = 0x02,
    /**
     * By default, upon calls to yajl_complete_parse(), yajl will
     * ensure the entire input text was consumed and will raise an error
     * otherwise.  Enabling this flag will cause yajl to disable this
     * check.  This can be useful when parsing json out of a that contains more
     * than a single JSON document.
     */
    yajl_allow_trailing_garbage = 0x04,
    /**
     * Allow multiple values to be parsed by a single handle.  The
     * entire text must be valid JSON, and values can be seperated
     * by any kind of whitespace.  This flag will change the
     * behavior of the parser, and cause it continue parsing after
     * a value is parsed, rather than transitioning into a
     * complete state.  This option can be useful when parsing multiple
     * values from an input stream.
     */
    yajl_allow_multiple_values = 0x08,
    /**
     * When yajl_complete_parse() is called the parser will
     * check that the top level value was completely consumed.  I.E.,
     * if called whilst in the middle of parsing a value
     * yajl will enter an error state (premature EOF).  Setting this
     * flag suppresses that check and the corresponding error.
     */
    yajl_allow_partial_values = 0x10,
}

/** allow the modification of parser options subsequent to handle
 *  allocation (via yajl_alloc)
 *  \returns zero in case of errors, non-zero otherwise
 */
#[no_mangle]
pub extern "C" fn yajl_config(h: *mut yajl_handle, opt: yajl_option) -> c_int {
    todo!("yajl_config")
}

/** free a parser handle */
#[no_mangle]
pub extern "C" fn yajl_free(handle: *mut yajl_handle) {
    unsafe {
        drop(Box::from_raw(handle));
    }
}

/** Parse some json!
 *  \param hand - a handle to the json parser allocated with yajl_alloc
 *  \param jsonText - a pointer to the UTF8 json text to be parsed
 *  \param jsonTextLength - the length, in bytes, of input text
 */
#[no_mangle]
pub extern "C" fn yajl_parse(
    hand: *mut yajl_handle,
    jsonText: *const c_uchar,
    jsonTextLength: usize,
) -> yajl_status {
    yajl_status::yajl_status_ok
}

/** Parse any remaining buffered json.
 *  Since yajl is a stream-based parser, without an explicit end of
 *  input, yajl sometimes can't decide if content at the end of the
 *  stream is valid or not.  For example, if "1" has been fed in,
 *  yajl can't know whether another digit is next or some character
 *  that would terminate the integer token.
 *
 *  \param hand - a handle to the json parser allocated with yajl_alloc
 */
#[no_mangle]
pub extern "C" fn yajl_complete_parse(hand: *mut yajl_handle) -> yajl_status {
    yajl_status::yajl_status_ok
}

/** get an error string describing the state of the
 *  parse.
 *
 *  If verbose is non-zero, the message will include the JSON
 *  text where the error occured, along with an arrow pointing to
 *  the specific char.
 *
 *  \returns A dynamically allocated string will be returned which should
 *  be freed with yajl_free_error
 */
#[no_mangle]
pub extern "C" fn yajl_get_error(
    hand: *mut yajl_handle,
    verbose: c_int,
    jsonText: *const c_uchar,
    jsonTextLength: usize,
) -> *mut c_uchar {
    todo!()
}

/**
 * get the amount of data consumed from the last chunk passed to YAJL.
 *
 * In the case of a successful parse this can help you understand if
 * the entire buffer was consumed (which will allow you to handle
 * "junk at end of input").
 *
 * In the event an error is encountered during parsing, this function
 * affords the client a way to get the offset into the most recent
 * chunk where the error occured.  0 will be returned if no error
 * was encountered.
 */
#[no_mangle]
pub extern "C" fn yajl_get_bytes_consumed(hand: *mut yajl_handle) -> usize {
    todo!()
}

/** free an error returned from yajl_get_error */
#[no_mangle]
pub extern "C" fn yajl_free_error(hand: *mut yajl_handle, s: *mut c_uchar) {
    todo!()
}
