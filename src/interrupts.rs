use crate::{write_reg, read_reg, temp_panic};
use core::arch::{asm, x86_64::__cpuid};


pub unsafe fn get_msr_val(ecx: u32) -> u64{
    let mut hi: u64 = 0;  // EDX
    let mut lo: u64 = 0;  // EAX
    asm!("xor rax, rax");
    asm!("xor rdx, rdx");
    asm!("mov ecx, {0:e}", in(reg) ecx);
    asm!("rdmsr");
    asm!("mov {}, rdx", out(reg) hi);
    asm!("mov {}, rax", out(reg) lo);
    // DISP.print("DEBUG:\n");
    // DISP.print_hex(hi);
    // DISP.print("\n");
    // DISP.print_hex(lo);
    return (hi << 32) | lo;
}

pub fn check_apic(){
    unsafe{
        let result = __cpuid(1);
        if result.edx & (1<<9) == 0 {
            temp_panic("Apic not found");
            loop {};
        }
    }
}

pub fn get_apic_base(){

}