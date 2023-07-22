use crate::{write_reg, read_reg, io::DISP};
use core::arch::{asm, x86_64::__cpuid};

pub fn check_apic(){
    unsafe{
        let result = __cpuid(1);
        if (result.edx & (1<<9) == 0) {
            DISP.print("APIC not found"); // cannot use panic handler yet cause of no second stage loader
            loop {};
        }
    }
}