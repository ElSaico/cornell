#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate cornell;

use core::panic::PanicInfo;
use cornell::halt_qemu;

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    serial_println!("failed");
    serial_println!("{}", _info);
    unsafe { halt_qemu() }
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("ok");
    unsafe { halt_qemu() }
    loop {}
}
