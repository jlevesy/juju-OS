#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from juju-OS {}", "!");
    println!("Such funny things incoming");

    loop {}
}
