// pest's test macros spit out too many of these errors!
#![allow(non_fmt_panic)]

pub mod parser;

#[macro_use]
extern crate pest_derive;
