#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(raw_ref_op)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod src {
pub mod basic;
pub mod bind;
pub mod buffer;
pub mod colorscheme;
pub mod command_mode;
pub mod completion;
pub mod cscope;
pub mod cutln;
pub mod display;
pub mod eval;
pub mod exec;
pub mod file;
pub mod fileio;
pub mod globals;
pub mod highlight;
pub mod input;
pub mod isearch;
pub mod line;
pub mod lock;
pub mod main;
pub mod names;
pub mod nanox;
pub mod ncurses;
pub mod paste_slot;
pub mod pklock;
pub mod platform;
pub mod posix;
pub mod random;
pub mod region;
pub mod scraper;
pub mod search;
pub mod spawn;
pub mod tcap;
pub mod term_wrapper;
pub mod usage;
pub mod utf8;
pub mod version;
pub mod window;
pub mod word;
pub mod wrapper;
} // mod src
