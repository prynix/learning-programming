#![allow(unused_variables, dead_code)]

#![feature(get_type_id)]

extern crate libc;

#[cfg(if_types)]
mod primitive_types;

mod modules;

fn main() {
    // primitive_types::primitive_types_main();
    modules::modules_main();
}
