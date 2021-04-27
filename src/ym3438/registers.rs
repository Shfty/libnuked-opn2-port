#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Registers {
    pub mode_test_21: [u8; 8],
    pub mode_test_2c: [u8; 8],
    pub mode_ch3: u8,
    pub mode_kon_channel: u8,
    pub mode_kon_operator: [u8; 4],
    pub mode_kon: [u8; 24],
    pub mode_csm: u8,
    pub mode_kon_csm: u8,
    pub dacen: u8,
    pub dacdata: i16,
    pub ks: [u8; 24],
    pub ar: [u8; 24],
    pub sr: [u8; 24],
    pub dt: [u8; 24],
    pub multi: [u8; 24],
    pub sl: [u8; 24],
    pub rr: [u8; 24],
    pub dr: [u8; 24],
    pub am: [u8; 24],
    pub tl: [u8; 24],
    pub ssg_eg: [u8; 24],
    pub fnum: [u16; 6],
    pub block: [u8; 6],
    pub kcode: [u8; 6],
    pub fnum_3ch: [u16; 6],
    pub block_3ch: [u8; 6],
    pub kcode_3ch: [u8; 6],
    pub reg_a4: u8,
    pub reg_ac: u8,
    pub connect: [u8; 6],
    pub fb: [u8; 6],
    pub pan_l: [u8; 6],
    pub pan_r: [u8; 6],
    pub ams: [u8; 6],
    pub pms: [u8; 6],
    pub status: u8,
    pub status_time: u32,
}

impl Default for Registers {
    fn default() -> Self {
        let mut registers: Registers = unsafe { std::mem::zeroed() };
        registers.multi = [1; 24];
        registers.pan_l = [1; 6];
        registers.pan_r = [1; 6];
        registers
    }
}
