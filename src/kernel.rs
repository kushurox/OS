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
        disp.print("kruthart is so cool he made \ncome into reality");
        // write(temp.offset(160), 0x61);
        
        // write(temp.offset(161), 0x1F);

    }

    loop {}
}