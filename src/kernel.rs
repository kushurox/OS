#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::write};
use io::display;

// extern crate alloc;
mod io;
// mod mem;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut disp = display::new();
    let temp = 0xb8000 as *mut u8;

    unsafe {
        disp.clrscr();
        disp.print("Paging setup\nGDT Setup\nKernel Running!\nReally Cool!");

    }

    loop {}
}
