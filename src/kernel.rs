#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(int_roundings)]

extern crate x86_64;

use core::{panic::PanicInfo, arch::asm};
use io::display;

mod io;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut stderr = display::new();
    stderr.color_scheme = 0x0C;
    // let msg = _info.message().unwrap().as_str().unwrap(); need a second stage loader to make this work
    unsafe {
        stderr.clrscr();
        stderr.print("Panic :(")
    }
    loop {}
}


#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut disp = display::new();
    let rip: u64;
    unsafe {
        disp.clrscr();
        disp.print("Paging setup\nGDT Setup\nKernel Running!\nRIP:");
        asm!("lea {rip}, [rip]", rip=out(reg) rip);
        disp.print_hex(rip);
    }

    loop {}
}
