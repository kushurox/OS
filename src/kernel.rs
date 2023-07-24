#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(int_roundings)]


use core::{panic::PanicInfo, ptr::read_volatile};
use io::DISP;

use crate::interrupts::{check_apic, get_msr_val};


mod io;
mod regs;
mod interrupts;

pub fn read_memory(addr: *const u64) -> u64 {
    unsafe {
        let val = read_volatile(addr);
        return val;
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // let msg = _info.message().unwrap().as_str().unwrap(); need a second stage loader to make this work
    unsafe {
        DISP.color_scheme = 0x0C;
        DISP.clrscr();
        DISP.print("Panic :(")
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    check_apic();
    unsafe {
        DISP.clrscr();
        let msr_result = get_msr_val(0x1B);
        
    }

    loop {}
}

