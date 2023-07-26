#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(int_roundings)]


use core::panic::PanicInfo;
use io::DISP;
use paging::Mapper;

use crate::interrupts::{check_apic, get_msr_val};


mod io;
mod regs;
mod interrupts;
mod paging;


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
    let pm = Mapper::default();
    check_apic();
    unsafe {
        DISP.clrscr();
        let apic_base = get_msr_val(0x1B);
        print!("Apic present!\n", "MSR: ");
        DISP.print_hex(apic_base);
        print!("\n");
        pm.info_table();
    }

    loop {}
}

