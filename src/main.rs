// in src/main.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(siso::test_runner)]

use core::panic::PanicInfo;
use siso::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    siso::init();
    println!("Hello Kernal!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    siso::test_panic_handler(info)
}
