use core::{ptr::{write, write_volatile, write_unaligned}};


#[allow(non_camel_case_types)]
pub struct display{
    pub vga_buffer: *mut u8,
    pub offset: isize
}

impl display{
    pub fn new() -> Self{
        return display {vga_buffer: 0xb8000 as *mut u8, offset:0};
    }
    pub unsafe fn print(&mut self, text: &str){
        for b in text.bytes() {
            if b == 10 {  // \n ascii value is 10
                self.offset = self.offset + 160-(self.offset%160);
                continue;
            }
            write_unaligned(self.vga_buffer.offset(self.offset), b);
            write_unaligned(self.vga_buffer.offset(self.offset+1), 0x1F);
            self.offset += 2; 
        }
    }
    pub unsafe fn clrscr(&mut self){
        self.offset = 0;
        for _ in 0..80*25{
            write_volatile(self.vga_buffer.offset(self.offset), 0);
            write_volatile(self.vga_buffer.offset(self.offset + 1), 0x1F);
            self.offset += 2;
        }
        self.offset = 0;
    }
}

// pub unsafe fn print(text: &str){
//     let vga_buffer = 0xb8000 as *mut u8;
//     let mut offset = 0;
//     for b in text.bytes(){
//         write(vga_buffer.offset(offset), b);
//         write(vga_buffer.offset(offset + 1), 0x1F);
//         offset += 2;
//     }
// }

// pub unsafe fn clrscr(){
//     let vga_buffer = 0xb8000 as *mut u8;
//     let mut offset = 0;
//     for _ in 0..80*25{
//         write(vga_buffer.offset(offset), 0);
//         write(vga_buffer.offset(offset + 1), 0x1F);
//         offset += 2;
//     }
// }