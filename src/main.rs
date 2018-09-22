#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate std;
#[cfg(test)]
extern crate array_init;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    loop {}
}
