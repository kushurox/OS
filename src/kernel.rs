#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(int_roundings)]


use core::{panic::PanicInfo, ptr::{null, read_volatile}};
use io::DISP;
use paging::{Mapper, VirtualAddress, AddressType::{VirtualAddress as VA_t, PhysicalAddress as PA_t}};

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
        // DISP.clrscr();
        DISP.print("\nPanic :(")
    }
    loop {}
}

pub fn temp_panic(msg: &'static str) -> !{ // second stage loader needed for actual panic, hence using this
    unsafe{print!("Panic occured:", msg);};
    loop {};
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let pm = Mapper::default();
    check_apic();
    unsafe {
        DISP.clrscr();
        // DISP.print_hex(read_volatile(0x1000 as *const u64));print!("\n");
        if let PA_t(addr) = pm.resolve(VA_t(VirtualAddress::new(0x7c00))){
            DISP.print_hex(addr as u64);
        }

    }

    loop {}
}

