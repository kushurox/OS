#![no_std]
#![no_main]

#![feature(panic_info_message)]

extern crate x86_64;

use core::{panic::PanicInfo};
use io::display;

mod io;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut stderr = display::new();
    stderr.color_scheme = 0x0C;
    unsafe {
        stderr.clrscr();

    }
    loop {}
}


#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut disp = display::new();

    unsafe {
        disp.clrscr();
        disp.print("AAAAAAAA\n");
        disp.print("This is a very long string which causes the program to crash unfortunately!, fret not let's fix this!");
        disp.print("Paging setup\nGDT Setup\nKernel Running!\nReally Cool!");
    }

    loop {}
}
