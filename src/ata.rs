extern "C" {
    // dest_addr: destination
    // chs: CHS coords |3 unused bytes|2 bytes cylinder|1 byte sector count|1 byte sector number|

    // returns error code if failed
    pub fn read_chs(dest_addr: *const u64, chs: u64) -> u8;
}
