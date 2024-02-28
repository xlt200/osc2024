#![no_std]
#![no_main]

mod driver;
mod memory;
mod shell;

use core::{arch::global_asm, panic::PanicInfo};

use library::println;
use shell::Shell;

global_asm!(include_str!("entry.S"));

#[no_mangle]
pub extern "C" fn kernel_start() {
    unsafe {
        driver::init().unwrap();
    }
    let mut shell = Shell::new();
    shell.run();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
