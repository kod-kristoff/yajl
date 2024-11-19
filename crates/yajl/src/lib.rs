#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(feature = "nightly", feature(c_variadic))]
#![cfg_attr(feature = "nightly", feature(extern_types))]

extern crate libc;
pub(crate) mod buffer;
pub(crate) mod lexer;
pub mod parser;
pub mod status;
pub mod yajl_alloc;
pub mod yajl_encode;
pub mod yajl_gen;
pub mod yajl_tree;
pub mod yajl_version;

pub use parser::{Parser, ParserOption};
pub use status::Status;

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
mod util_libc;
