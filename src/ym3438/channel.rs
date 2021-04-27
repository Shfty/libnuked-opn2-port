use crate::{rom::FM_ALGORITHM, Fm, Registers};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Channel {
    pub acc: [i16; 6],
    pub out: [i16; 6],
    pub lock: i16,
    pub lock_l: u8,
    pub lock_r: u8,
    pub read: i16,
}

impl Channel {
    pub fn generate(&mut self, cycles: u32, channel: u32, registers: &Registers, fm: &Fm) {
        let slot = cycles
            .wrapping_add(18)
            .wrapping_rem(24);
        let channel = channel;
        let op = slot.wrapping_div(6);
        let test_dac = registers.mode_test_2c[5] as u32;
        let mut acc = self.acc[channel as usize];
        let mut add = test_dac as i16;
        let mut sum;
        if op == 0 && test_dac == 0 {
            acc = 0;
        }
        if FM_ALGORITHM[op as usize][5][registers.connect[channel as usize] as usize] != 0
            && test_dac == 0
        {
            add = (add as i32 + (fm.out[slot as usize] as i32 >> 5)) as i16
        }
        sum = (acc as i32 + add as i32) as i16;

        // Clamp
        if sum as i32 > 255 {
            sum = 255;
        } else if (sum as i32) < -256 {
            sum = -256;
        }
        if op == 0 || test_dac != 0 {
            self.out[channel as usize] = self.acc[channel as usize]
        }
        self.acc[channel as usize] = sum;
    }
}
