const PML4_BASE: u64 = 0x1000;
const PML3_BASE: u64 = PML4_BASE + 0x1000;
const PML2_BASE: u64 = PML4_BASE + 0x2000;
const PML1_BASE: i64 = PML4_BASE + 0x3000;



pub struct Entry{
    data: u64
}

impl Entry {
    pub fn new(d: u64) -> Self{
        
    }
}

pub struct PageTable {
    pub addr: u64,
    pub entry_count: u16
}

impl PageTable {
    pub fn new(buf_loc: u64) -> Self{
        PageTable {
            addr: buf_loc,
            entry_count: 0
        }
    }

    pub fn add_entry(e: Entry){

    }
}