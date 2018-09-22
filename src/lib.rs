#![no_std]

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
extern crate uart_16550;
extern crate x86_64;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate std;
#[cfg(test)]
extern crate array_init;

pub mod vga_buffer;
pub mod serial;

pub unsafe fn halt_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
