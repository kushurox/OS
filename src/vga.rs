use core::ptr::write_volatile;

use crate::temp_panic;

pub struct Vga {
    pub vga_address: *mut u16,
    pub color: u16,
    pub offset: usize
}


impl Vga{
    pub fn new() -> Self{
        Vga { vga_address: 0xb8000 as *mut u16, color: 0x0A, offset: 0 }
    }

    pub fn print_string(&mut self, st: &'static str) {
        if st.len() > (80*25) {
            temp_panic("Too long!!!");
        }
        for b in st.bytes(){
            if b == 10{
                self.offset = self.offset + (80 - self.offset%80);
                continue;
            }
            unsafe{
                let char: u16 = (self.color<<8) | (b as u16);
                write_volatile(self.vga_address.add(self.offset), char);
            }
            self.offset += 1;
        }
    }

    pub fn clrscr(&self){
        unsafe{
            for offset in 0..(80*25){
                write_volatile(self.vga_address.offset(offset), 0x0)
            }
        }
    }

    pub fn putch(&mut self, ch: u16){
        unsafe {
            if ch == 10 {
                self.offset = self.offset + (80 - self.offset%80);
                return;
            }
            write_volatile(self.vga_address.add(self.offset), (self.color << 8) | ch);
            self.offset += 1;
        }
    }


    pub fn print_hex(&mut self, mut d: u64) {
        let mut chars = [0u16; 16];
        for i in 0..16{
            let temp = (d & 0xF) as u16;
            if temp > 9 {
                chars[15-i] = temp - 10 + 0x41;
            }
            else {
                chars[15-i] = temp + 0x30;
            }
            d = d >> 4;
        }
        self.print_string("0x");
        for ch in chars {
            self.putch(ch);
        }

    }
}