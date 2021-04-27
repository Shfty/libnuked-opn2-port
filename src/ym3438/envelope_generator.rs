use crate::{
    rom::{EG_AM_SHIFT, EG_STEP_HI},
    Adsr, Lfo, PhaseGenerator, Registers,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnvelopeGenerator {
    pub cycle: u8,
    pub cycle_stop: u8,
    pub shift: u8,
    pub shift_lock: u8,
    pub timer_low_lock: u8,
    pub timer: u16,
    pub timer_inc: u8,
    pub quotient: u16,
    pub custom_timer: u8,
    pub rate: u8,
    pub ksv: u8,
    pub inc: u8,
    pub ratemax: u8,
    pub sl: [u8; 2],
    pub lfo_am: u8,
    pub tl: [u8; 2],
    pub state: [u8; 24],
    pub level: [u16; 24],
    pub out: [u16; 24],
    pub kon: [u8; 24],
    pub kon_csm: [u8; 24],
    pub kon_latch: [u8; 24],
    pub csm_mode: [u8; 24],
    pub ssg_enable: [u8; 24],
    pub ssg_pgrst_latch: [u8; 24],
    pub ssg_repeat_latch: [u8; 24],
    pub ssg_hold_up_latch: [u8; 24],
    pub ssg_dir: [u8; 24],
    pub ssg_inv: [u8; 24],
    pub read: [u32; 2],
    pub read_inc: u8,
}

impl Default for EnvelopeGenerator {
    fn default() -> Self {
        let mut eg: EnvelopeGenerator = unsafe { std::mem::zeroed() };
        eg.out = [0x3ff; 24];
        eg.level = [0x3ff; 24];
        eg.state = [Adsr::Release as u8; 24];
        eg
    }
}

impl EnvelopeGenerator {
    pub fn cycle_1(&mut self) {
        // Lock envelope generator timer value
        if self.quotient == 2 {
            if self.cycle_stop != 0 {
                self.shift_lock = 0;
            } else {
                self.shift_lock = (self.shift as i32 + 1) as u8
            }
            self.timer_low_lock = (self.timer as i32 & 0x3) as u8
        }

        self.quotient = self.quotient.wrapping_add(1);
        self.quotient = (self.quotient as i32 % 3) as u16;
        self.cycle = 0;
        self.cycle_stop = 1;
        self.shift = 0;
        self.timer_inc = (self.timer_inc as i32 | self.quotient as i32 >> 1) as u8;
        self.timer = (self.timer as i32 + self.timer_inc as i32) as u16;
        self.timer_inc = (self.timer as i32 >> 12) as u8;
        self.timer = (self.timer as i32 & 0xfff) as u16
    }

    pub fn cycle_2(&mut self) {
        self.read[1] = self.out[0] as u32;
    }

    pub fn cycle_13(&mut self) {
        self.cycle = 0;
        self.cycle_stop = 1;
        self.shift = 0;
        self.timer = (self.timer as i32 + self.timer_inc as i32) as u16;
        self.timer_inc = (self.timer as i32 >> 12) as u8;
        self.timer = (self.timer as i32 & 0xfff) as u16
    }

    pub fn prepare(
        &mut self,
        cycles: u32,
        channel: u32,
        registers: &Registers,
        phase_generator: &PhaseGenerator,
        lfo: &Lfo,
    ) {
        let mut inc = 0;
        let slot = cycles;
        let mut rate_sel: u8;

        // Prepare increment
        let mut rate = (((self.rate as i32) << 1) + self.ksv as i32) as u8;
        if rate as i32 > 0x3f {
            rate = 0x3f;
        }
        let sum = (((rate as i32 >> 2) + self.shift_lock as i32) & 0xf) as u8;
        if self.rate as i32 != 0 && self.quotient as i32 == 2 {
            if (rate as i32) < 48 {
                match sum as i32 {
                    12 => inc = 1,
                    13 => inc = (rate as i32 >> 1 & 0x1) as u8,
                    14 => inc = (rate as i32 & 0x1) as u8,
                    _ => {}
                }
            } else {
                inc = EG_STEP_HI[(rate as i32 & 0x3) as usize][self.timer_low_lock as usize]
                    .wrapping_add((rate as i32 >> 2) as u32)
                    .wrapping_sub(11) as u8;
                if inc as i32 > 4 {
                    inc = 4;
                }
            }
        }
        self.inc = inc;
        self.ratemax = (rate as i32 >> 1 == 0x1f) as i32 as u8;

        // Prepare rate & ksv
        rate_sel = self.state[slot as usize];
        if self.kon[slot as usize] as i32 != 0 && self.ssg_repeat_latch[slot as usize] as i32 != 0
            || self.kon[slot as usize] == 0 && self.kon_latch[slot as usize] as i32 != 0
        {
            rate_sel = Adsr::Attack as i32 as u8
        }
        match rate_sel as i32 {
            0 => self.rate = registers.ar[slot as usize],
            1 => self.rate = registers.dr[slot as usize],
            2 => self.rate = registers.sr[slot as usize],
            3 => self.rate = ((registers.rr[slot as usize] as i32) << 1 | 0x1) as u8,
            _ => {}
        }
        self.ksv =
            (phase_generator.kcode as i32 >> (registers.ks[slot as usize] as i32 ^ 0x3)) as u8;
        if registers.am[slot as usize] != 0 {
            self.lfo_am = (lfo.am as i32
                >> EG_AM_SHIFT[registers.ams[channel as usize] as usize] as i32)
                as u8
        } else {
            self.lfo_am = 0;
        }

        // Delay TL & SL value
        self.tl[1] = self.tl[0];
        self.tl[0] = registers.tl[slot as usize];
        self.sl[1] = self.sl[0];
        self.sl[0] = registers.sl[slot as usize];
    }

    pub fn generate(&mut self, cycles: u32, channel: u32, registers: &Registers) {
        let slot = cycles.wrapping_add(23).wrapping_rem(24);
        let mut level = self.level[slot as usize];
        if self.ssg_inv[slot as usize] != 0 {
            // Inverse
            level = (512 - level as i32) as u16
        }
        if registers.mode_test_21[5] != 0 {
            level = 0;
        }
        level = (level as i32 & 0x3ff) as u16;

        // Apply AM LFO
        level = (level as i32 + self.lfo_am as i32) as u16;

        // Apply TL
        if !(registers.mode_csm as i32 != 0 && channel == (2 + 1) as u32) {
            level = (level as i32 + ((self.tl[0] as i32) << 3)) as u16
        }
        if level as i32 > 0x3ff {
            level = 0x3ff;
        }
        self.out[slot as usize] = level;
    }

    pub fn adsr(&mut self, cycles: u32, phase_generator: &mut PhaseGenerator) {
        let slot = cycles.wrapping_add(22).wrapping_rem(24);
        let nkon = self.kon_latch[slot as usize];
        let okon = self.kon[slot as usize];
        let mut nextstate = self.state[slot as usize];
        let mut inc = 0;
        self.read[0] = self.read_inc as u32;
        self.read_inc = (self.inc as i32 > 0) as i32 as u8;

        // Reset phase generator
        phase_generator.reset[slot as usize] = (nkon as i32 != 0 && okon == 0
            || self.ssg_pgrst_latch[slot as usize] as i32 != 0)
            as i32 as u8;

        // KeyOn/Off
        let kon_event = (nkon as i32 != 0 && okon == 0
            || okon as i32 != 0
                && self.ssg_repeat_latch[slot as usize] as i32 != 0)
            as i32 as u8;
        let koff_event = (okon as i32 != 0 && nkon == 0) as i32 as u8;
        let mut level = self.level[slot as usize] as i16;
        let mut ssg_level = level;

        if self.ssg_inv[slot as usize] != 0 {
            // Inverse
            ssg_level = (512 - level as i32) as i16;
            ssg_level = (ssg_level as i32 & 0x3ff) as i16
        }

        if koff_event != 0 {
            level = ssg_level
        }

        let eg_off = if self.ssg_enable[slot as usize] != 0 {
            (level as i32 >> 9) as u8
        } else {
            (level as i32 & 0x3f0 == 0x3f0) as i32 as u8
        };

        let mut nextlevel = level;
        if kon_event != 0 {
            nextstate = Adsr::Attack as i32 as u8;
            // Instant attack
            if self.ratemax != 0 {
                nextlevel = 0;
            } else if self.state[slot as usize] as i32 == Adsr::Attack as i32
                && level as i32 != 0
                && self.inc as i32 != 0
                && nkon as i32 != 0
            {
                inc = ((!(level as i32)) << self.inc as i32 >> 5) as i16
            }
        } else {
            match self.state[slot as usize] as i32 {
                0 => {
                    if level as i32 == 0 {
                        nextstate = Adsr::Decay as i32 as u8
                    } else if self.inc as i32 != 0
                        && self.ratemax == 0
                        && nkon as i32 != 0
                    {
                        inc = ((!(level as i32)) << self.inc as i32 >> 5) as i16
                    }
                }
                1 => {
                    if level as i32 >> 5 == self.sl[1] as i32 {
                        nextstate = Adsr::Sustain as i32 as u8
                    } else if eg_off == 0 && self.inc as i32 != 0 {
                        inc = (1 << (self.inc as i32 - 1)) as i16;
                        if self.ssg_enable[slot as usize] != 0 {
                            inc = ((inc as i32) << 2) as i16
                        }
                    }
                }
                2 | 3 => {
                    if eg_off == 0 && self.inc as i32 != 0 {
                        inc = (1 << (self.inc as i32 - 1)) as i16;
                        if self.ssg_enable[slot as usize] != 0 {
                            inc = ((inc as i32) << 2) as i16
                        }
                    }
                }
                _ => {}
            }
            if nkon == 0 {
                nextstate = Adsr::Release as i32 as u8
            }
        }

        if self.kon_csm[slot as usize] != 0 {
            nextlevel = (nextlevel as i32 | (self.tl[1] as i32) << 3) as i16
        }

        // Envelope off
        if kon_event == 0
            && self.ssg_hold_up_latch[slot as usize] == 0
            && self.state[slot as usize] as i32 != Adsr::Attack as i32
            && eg_off as i32 != 0
        {
            nextstate = Adsr::Release as i32 as u8;
            nextlevel = 0x3ff;
        }

        nextlevel = (nextlevel as i32 + inc as i32) as i16;
        self.kon[slot as usize] =
            self.kon_latch[slot as usize];
        self.level[slot as usize] = (nextlevel as u16 as i32 & 0x3ff) as u16;
        self.state[slot as usize] = nextstate;
    }

    pub fn ssg_eg(&mut self, cycles: u32, registers: &Registers) {
        let slot = cycles;
        let mut direction = 0;
        self.ssg_pgrst_latch[slot as usize] = 0;
        self.ssg_repeat_latch[slot as usize] = 0;
        self.ssg_hold_up_latch[slot as usize] = 0;
        self.ssg_inv[slot as usize] = 0;
        if registers.ssg_eg[slot as usize] as i32 & 0x8 != 0 {
            direction = self.ssg_dir[slot as usize];
            if self.level[slot as usize] as i32 & 0x200 != 0 {
                // Reset
                if registers.ssg_eg[slot as usize] as i32 & 0x3 == 0 {
                    self.ssg_pgrst_latch[slot as usize] = 1;
                }
                // Repeat
                if registers.ssg_eg[slot as usize] as i32 & 0x1 == 0 {
                    self.ssg_repeat_latch[slot as usize] = 1;
                }
                // Inverse
                if registers.ssg_eg[slot as usize] as i32 & 0x3 == 0x2 {
                    direction = (direction as i32 ^ 1) as u8;
                }
                if registers.ssg_eg[slot as usize] as i32 & 0x3 == 0x3 {
                    direction = 1;
                }
            }

            // Hold up
            if self.kon_latch[slot as usize] as i32 != 0
                && (registers.ssg_eg[slot as usize] as i32 & 0x7 == 0x5
                    || registers.ssg_eg[slot as usize] as i32 & 0x7 == 0x3)
            {
                self.ssg_hold_up_latch[slot as usize] = 1;
            }
            direction = (direction as i32 & self.kon[slot as usize] as i32) as u8;
            self.ssg_inv[slot as usize] = ((self.ssg_dir[slot as usize] as i32
                ^ registers.ssg_eg[slot as usize] as i32 >> 2 & 0x1)
                & self.kon[slot as usize] as i32) as u8
        }
        self.ssg_dir[slot as usize] = direction;
        self.ssg_enable[slot as usize] = (registers.ssg_eg[slot as usize] as i32 >> 3 & 0x1) as u8;
    }
}
