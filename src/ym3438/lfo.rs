use crate::rom::LFO_CYCLES;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lfo {
    pub en: u8,
    pub freq: u8,
    pub pm: u8,
    pub am: u8,
    pub cnt: u8,
    pub inc: u8,
    pub quotient: u8,
}

impl Lfo {
    pub fn cycle_0(&mut self) {
        self.pm = (self.cnt as i32 >> 2) as u8;
        if self.cnt as i32 & 0x40 != 0 {
            self.am = (self.cnt as i32 & 0x3f) as u8
        } else {
            self.am = (self.cnt as i32 ^ 0x3f) as u8
        }
        self.am = ((self.am as i32) << 1) as u8
    }

    pub fn cycle_23(&mut self) {
        self.inc = (self.inc as i32 | 1) as u8;
    }

    pub fn update(&mut self) {
        if self.quotient as u32 & LFO_CYCLES[self.freq as usize] == LFO_CYCLES[self.freq as usize] {
            self.quotient = 0;
            self.cnt = self.cnt.wrapping_add(1)
        } else {
            self.quotient = (self.quotient as i32 + self.inc as i32) as u8
        }
        self.cnt = (self.cnt as i32 & self.en as i32) as u8;
    }
}
