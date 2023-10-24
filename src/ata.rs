pub struct ChsType {
    c: u16,
    drive_head: u8,
    sec_count: u8,
    sec_no: u8
}

impl Into<u64> for ChsType {
    fn into(self) -> u64 {
        let mut data: u64 = 0;
        data = data & (self.sec_no as u64);
        todo!();
        data
    }
}

extern "C" {
    // dest_addr: destination
    // chs: CHS coords | 3 bytes unused | 2 bytes: cylinder | 1 byte: drive_head | 1 byte: sector count | 1 byte: sector number

    // returns error code if failed
    pub fn read_chs(dest_addr: *const u64, chs: u64) -> u8;
}
