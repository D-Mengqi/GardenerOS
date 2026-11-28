#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    for i in 1..=3 {
        println!("{} , Hello,this is Hou hanbo's OS!", i);
    }
    unsafe {
        asm!("sret");
    }
    0
}