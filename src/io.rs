use core::{ptr::write_volatile, convert::TryInto};



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
    pub unsafe fn print_bytes(&mut self, bytes: [u8; 16]) {
        for b in bytes {
            write_volatile(self.vga_buffer.offset(self.offset), b);
            write_volatile(self.vga_buffer.offset(self.offset+1), self.color_scheme);
            self.offset += 2;
        }
    }
}

impl display {
    pub unsafe fn print_hex(&mut self, mut n: u64){
        let tmp = self.color_scheme;
        self.color_scheme = 0x0E;
        let mut hexits = [0u8; 16];
        for c in (0..16).rev() {
            let r: u8 = (n%16).try_into().unwrap();
            n = n.div_floor(16);
            if r > 9 {
                hexits[c] = 5 - (15 - r) + 65; // 'A' + (5 - 'F' - r)
            }
            else{
                hexits[c] = r + 48;
            }
        }
        self.print("0x");
        self.print_bytes(hexits);
        self.color_scheme = tmp;
    }
}

unsafe impl Sync for display {

}

pub static mut DISP: display = display {vga_buffer: 0xb8000 as *mut u8, offset:0, color_scheme: 0x0A};
