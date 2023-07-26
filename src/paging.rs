use core::{ptr::read_volatile, convert::TryInto};

use crate::{print, io::DISP};

const PML4_BASE: u64 = 0x1000;              // from paging.asm, we know pml4 base is 0x1000

pub struct Mapper {
    pub pml4_addr: *mut u64,
}

impl Mapper{
    pub fn default() -> Self{
        // uses default addresses for tables (from paging.asm)
        Mapper {
            pml4_addr: PML4_BASE as *mut u64
        }
    }

    pub fn info_table(&self) {
        // display current pointer addresses
        todo!()
    }


    fn map_address(&mut self, phy: *const u64, virt: *const u64){
        todo!()
    }

    pub fn resolve(&self, virt: u64) -> u64 {
        let i4 = virt >> 39;
        let i3 = (virt >> 30) & 0b111111111;
        let i2 = (virt >> 21) & 0b111111111;
        let i1 = (virt >> 12) & 0b111111111;
        let page_offset = virt & 0xFFF;

        unsafe {
            let l3_address = read_volatile(self.pml4_addr.wrapping_add(i4.try_into().unwrap()));
        }
        4
    }
}

// virt addr: | unused bits | 9 bits | 9 bits | 9 bits | 9 bits | 12 bits |
// unused bits: 16 bits, used bits: 48 bits

// each l4 entry can map upto 1 * 512 * 512 * 512 * 4 kb = 512 GB or 0x20000000 kilobytes
// each l3 entry can map upto 1 * 512 * 512 * 4kb = 1 GB or 0x100000 kilobytes
// each l2 entry can map upto 1 * 512 * 4 = 2048 kilobytes or 2 MB
// each l1 entry can map upto 4 kilobytes
