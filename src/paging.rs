use core::ptr::read_volatile;

use crate::{print, io::DISP};

const PML4_BASE: u64 = 0x1000;              // from paging.asm, we know pml4 base is 0x1000
const PML3_BASE: u64 = PML4_BASE + 0x1000;
const PML2_BASE: u64 = PML4_BASE + 0x2000;
const PML1_BASE: u64 = PML4_BASE + 0x3000;


pub struct Mapper {
    pub pml4_addr: *mut u64,
    pub pml3_addr: *mut u64,
    pub pml2_addr: *mut u64,
    pub pml1_addr: *mut u64,
}

impl Mapper{
    pub fn init() -> Self{
        // uses default addresses for tables (from paging.asm)
        Mapper {
            pml4_addr: PML4_BASE as *mut u64,
            pml3_addr: PML3_BASE as *mut u64,
            pml2_addr: PML2_BASE as *mut u64,
            pml1_addr: PML1_BASE as *mut u64,
        }
    }

    pub fn info_table(&self) {
        // display current pointer addresses
        unsafe{
            print!("PML4:");
            DISP.print_hex(self.pml4_addr as u64);
            print!("\nPML3:");
            DISP.print_hex(self.pml3_addr as u64);
            print!("\nPML2:");
            DISP.print_hex(self.pml2_addr as u64);
            print!("\nPML1:");
            DISP.print_hex(self.pml1_addr as u64);
            print!("\nPML4 Entry count:");
            let mut temp = self.pml4_addr;
            let mut c = 0;
            while read_volatile(temp) != 0x0 {
                c += 1;
                temp = temp.wrapping_add(1);
            }
            DISP.print_hex(c);

            temp = self.pml3_addr;
            c = 0;
            print!("\nPML3 Entry count:");

            while read_volatile(temp) != 0x0 {
                c += 1;
                temp = temp.wrapping_add(1);
            }
            DISP.print_hex(c);

            temp = self.pml2_addr;
            c = 0;
            print!("\nPML2 Entry count:");
            while read_volatile(temp) != 0x0 {
                c += 1;
                temp = temp.wrapping_add(1);
            }
            DISP.print_hex(c);

            temp = self.pml1_addr;
            c = 0;
            print!("\nPML1 Entry count:");
            while read_volatile(temp) != 0x0 {
                c += 1;
                temp = temp.wrapping_add(1);
            }
            DISP.print_hex(c);
        }

    }
}