use core::{ptr::{write_volatile}, convert::TryInto};



#[allow(non_camel_case_types)]
pub struct display{
    pub vga_buffer: *mut u8,
    pub offset: isize,
    pub color_scheme: u8
}

impl display{
    pub fn new() -> Self{
        return display {vga_buffer: 0xb8000 as *mut u8, offset:0, color_scheme: 0x0A};
    }
    pub unsafe fn print(&mut self, text: &str){
        for b in text.bytes() {
            if b == 10 {  // \n ascii value is 10
                self.offset = self.offset + 160-(self.offset%160);
                continue;
            }
            write_volatile(self.vga_buffer.offset(self.offset), b);
            write_volatile(self.vga_buffer.offset(self.offset+1), self.color_scheme);
            self.offset += 2; 
        }
    }
    pub unsafe fn clrscr(&mut self){
        self.offset = 0;
        for _ in 0..80*25{
            write_volatile(self.vga_buffer.offset(self.offset), 0);
            write_volatile(self.vga_buffer.offset(self.offset + 1), self.color_scheme);
            self.offset += 2;
        }
        self.offset = 0;
    }
}

impl display {
    pub fn printHex(n: u64){
        let mut hexits = [0u8; 16];
        let mut c = 0;
        while n!=0 {
            let r: u8 = (n%16).try_into().unwrap();
            if r>9 {
                hexits[c] = ('F' as u8) - (15-r);
            }
        }

    }
}