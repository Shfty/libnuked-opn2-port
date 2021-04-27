use crate::{
    rom::{PG_DETUNE, PG_LFO_SH1, PG_LFO_SH2},
    Lfo, Registers,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhaseGenerator {
    pub fnum: u16,
    pub block: u8,
    pub kcode: u8,
    pub inc: [u32; 24],
    pub phase: [u32; 24],
    pub reset: [u8; 24],
    pub read: u32,
}

impl PhaseGenerator {
    pub fn cycle_2(&mut self) {
        self.read = self.phase[21] & 0x3ff;
    }

    pub fn generate(&mut self, cycles: u32, registers: &Registers) {
        // Mask increment
        let slot: u32 = cycles
            .wrapping_add(20)
            .wrapping_rem(24);

        if self.reset[slot as usize] != 0 {
            self.inc[slot as usize] = 0
        }

        // Phase step
        let slot = cycles
            .wrapping_add(19)
            .wrapping_rem(24);

        if self.reset[slot as usize] as i32 != 0
            || registers.mode_test_21[3] as i32 != 0
        {
            self.phase[slot as usize] = 0
        }

        self.phase[slot as usize] =
            (self.phase[slot as usize] as u32).wrapping_add(self.inc[slot as usize]) as u32 as u32;

        self.phase[slot as usize] &= 0xfffff;
    }

    pub fn phase_calc_increment(
        &mut self,
        cycles: u32,
        channel: u32,
        registers: &Registers,
        lfo: &Lfo,
    ) {
        let chan = channel;
        let slot = cycles;
        let mut fnum = self.fnum as u32;
        let fnum_h = fnum >> 4;
        let lfo = lfo.pm;
        let mut lfo_l = (lfo as i32 & 0xf) as u8;
        let pms = registers.pms[chan as usize];
        let dt = registers.dt[slot as usize];
        let dt_l = (dt as i32 & 0x3) as u8;
        let mut detune = 0;
        let mut kcode = self.kcode;
        fnum <<= 1;

        // Apply LFO
        if lfo_l as i32 & 0x8 != 0 {
            lfo_l = (lfo_l as i32 ^ 0xf) as u8
        }
        let mut fm = (fnum_h >> PG_LFO_SH1[pms as usize][lfo_l as usize])
            .wrapping_add(fnum_h >> PG_LFO_SH2[pms as usize][lfo_l as usize]);
        if pms as i32 > 5 {
            fm <<= pms as i32 - 5
        }
        fm >>= 2;
        if lfo as i32 & 0x10 != 0 {
            fnum = (fnum as u32).wrapping_sub(fm) as u32 as u32
        } else {
            fnum = (fnum as u32).wrapping_add(fm) as u32 as u32
        }
        fnum &= 0xfff;
        let mut basefreq = fnum << self.block as i32 >> 2;

        // Apply detune
        if dt_l != 0 {
            if kcode as i32 > 0x1c {
                kcode = 0x1c;
            }
            let block = (kcode as i32 >> 2) as u8;
            let note = (kcode as i32 & 0x3) as u8;
            let sum =
                (block as i32 + 9 + ((dt_l as i32 == 3) as i32 | dt_l as i32 & 0x2))
                    as u8;
            let sum_h = (sum as i32 >> 1) as u8;
            let sum_l = (sum as i32 & 0x1) as u8;
            detune = (PG_DETUNE[((sum_l as i32) << 2 | note as i32) as usize]
                >> (9 - sum_h as i32)) as u8
        }
        if dt as i32 & 0x4 != 0 {
            basefreq = (basefreq as u32).wrapping_sub(detune as u32) as u32 as u32
        } else {
            basefreq = (basefreq as u32).wrapping_add(detune as u32) as u32 as u32
        }
        basefreq &= 0x1ffff;
        self.inc[slot as usize] =
            basefreq.wrapping_mul(registers.multi[slot as usize] as u32) >> 1;
        self.inc[slot as usize] &= 0xfffff;
    }
}
