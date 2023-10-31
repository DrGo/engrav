#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(linkage)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod abcparse;
pub mod buffer;
pub mod deco;
pub mod draw;
pub mod format;
pub mod front;
pub mod glyph;
pub mod music;
pub mod parse;
pub mod subs;
pub mod svg;
pub mod syms;
} // mod src
