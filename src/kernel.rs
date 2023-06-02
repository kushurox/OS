#![no_std]
#![no_main]

#![feature(panic_info_message)]

extern crate x86_64;

use core::{panic::PanicInfo};
use io::display;

mod io;
// mod mem;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut stderr = display::new();
    stderr.color_scheme = 0x0C;
    unsafe {
        stderr.clrscr();
        // stderr.print("You Paniced!");

    }
    loop {}
}


#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut disp = display::new();

    // panic!("Ahh!!");
    unsafe {
        disp.clrscr();
        disp.print("AAAAAAAA\n");
        disp.print("AAAAAasjdiajsidajsidjaisjdiajsdiajsidjAAA\n");

        disp.print("Paging setup\nGDT Setup\nKernel Running!\nReally Cool!");
    }

    loop {}
}
