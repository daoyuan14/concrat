#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(register_tool)]
#![feature(rustc_private)]
#![feature(thread_local)]
#![feature(untagged_unions)]
#![register_tool(c2rust)]


extern crate libc;
pub mod main;
