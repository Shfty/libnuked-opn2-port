mod channel;
mod envelope_generator;
mod fm;
mod io;
mod lfo;
mod phase_generator;
mod registers;
mod timer_a;
mod timer_b;

pub use channel::*;
pub use envelope_generator::*;
pub use fm::*;
pub use io::*;
pub use lfo::*;
pub use phase_generator::*;
pub use registers::*;
pub use timer_a::*;
pub use timer_b::*;

use crate::rom::{CH_OFFSET, FN_NOTE, OP_OFFSET};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ym3438 {
    pub cycles: u32,
    pub channel: u32,
    pub mol: i16,
    pub mor: i16,

    pub io: Io,
    pub lfo: Lfo,
    pub phase_generator: PhaseGenerator,
    pub envelope_generator: EnvelopeGenerator,
    pub fm: Fm,
    pub ch: Channel,
    pub timer_a: TimerA,
    pub timer_b: TimerB,
    pub registers: Registers,
}

impl Ym3438 {
    pub fn do_reg_write(&mut self) {
        let mut i: u32;
        let mut slot = self.cycles.wrapping_rem(12);
        let mut address: u32;
        let channel = self.channel;

        // Update registers
        if self.io.write_fm_data != 0 {
            // Slot
            if OP_OFFSET[slot as usize] == (self.io.address as i32 & 0x107) as u32 {
                if self.io.address as i32 & 0x8 != 0 {
                    // OP2, OP4
                    slot = (slot as u32).wrapping_add(12) as u32 as u32
                }
                address = (self.io.address as i32 & 0xf0) as u32;
                match address {
                    48 => {
                        // DT, MULTI
                        self.registers.multi[slot as usize] = (self.io.data as i32 & 0xf) as u8;
                        if self.registers.multi[slot as usize] == 0 {
                            self.registers.multi[slot as usize] = 1;
                        } else {
                            self.registers.multi[slot as usize] =
                                ((self.registers.multi[slot as usize] as i32) << 1) as u8
                        }
                        self.registers.dt[slot as usize] = (self.io.data as i32 >> 4 & 0x7) as u8;
                    }
                    64 => {
                        // TL
                        self.registers.tl[slot as usize] = (self.io.data as i32 & 0x7f) as u8
                    }
                    80 => {
                        // KS, AR
                        self.registers.ar[slot as usize] = (self.io.data as i32 & 0x1f) as u8;
                        self.registers.ks[slot as usize] = (self.io.data as i32 >> 6 & 0x3) as u8;
                    }
                    96 => {
                        // AM, DR
                        self.registers.dr[slot as usize] = (self.io.data as i32 & 0x1f) as u8;
                        self.registers.am[slot as usize] = (self.io.data as i32 >> 7 & 0x1) as u8;
                    }
                    112 => {
                        // SR
                        self.registers.sr[slot as usize] = (self.io.data as i32 & 0x1f) as u8;
                    }
                    128 => {
                        // SL, RR
                        self.registers.rr[slot as usize] = (self.io.data as i32 & 0xf) as u8;
                        self.registers.sl[slot as usize] = (self.io.data as i32 >> 4 & 0xf) as u8;
                        self.registers.sl[slot as usize] = (self.registers.sl[slot as usize] as i32
                            | (self.registers.sl[slot as usize] as i32 + 1) & 0x10)
                            as u8
                    }
                    144 => {
                        // SSG-EG
                        self.registers.ssg_eg[slot as usize] = (self.io.data as i32 & 0xf) as u8
                    }
                    _ => {}
                }
            }

            // Channel
            if CH_OFFSET[channel as usize] == (self.io.address as i32 & 0x103) as u32 {
                address = (self.io.address as i32 & 0xfc) as u32;
                match address {
                    160 => {
                        self.registers.fnum[channel as usize] = (self.io.data as i32 & 0xff
                            | (self.registers.reg_a4 as i32 & 0x7) << 8)
                            as u16;
                        self.registers.block[channel as usize] =
                            (self.registers.reg_a4 as i32 >> 3 & 0x7) as u8;
                        self.registers.kcode[channel as usize] =
                            (((self.registers.block[channel as usize] as i32) << 2) as u32
                                | FN_NOTE
                                    [(self.registers.fnum[channel as usize] as i32 >> 7) as usize])
                                as u8
                    }
                    164 => self.registers.reg_a4 = (self.io.data as i32 & 0xff) as u8,
                    168 => {
                        self.registers.fnum_3ch[channel as usize] = (self.io.data as i32 & 0xff
                            | (self.registers.reg_ac as i32 & 0x7) << 8)
                            as u16;
                        self.registers.block_3ch[channel as usize] =
                            (self.registers.reg_ac as i32 >> 3 & 0x7) as u8;
                        self.registers.kcode_3ch[channel as usize] =
                            (((self.registers.block_3ch[channel as usize] as i32) << 2) as u32
                                | FN_NOTE[(self.registers.fnum_3ch[channel as usize] as i32 >> 7)
                                    as usize]) as u8
                    }
                    172 => self.registers.reg_ac = (self.io.data as i32 & 0xff) as u8,
                    176 => {
                        self.registers.connect[channel as usize] =
                            (self.io.data as i32 & 0x7) as u8;
                        self.registers.fb[channel as usize] = (self.io.data as i32 >> 3 & 0x7) as u8
                    }
                    180 => {
                        self.registers.pms[channel as usize] = (self.io.data as i32 & 0x7) as u8;
                        self.registers.ams[channel as usize] =
                            (self.io.data as i32 >> 4 & 0x3) as u8;
                        self.registers.pan_l[channel as usize] =
                            (self.io.data as i32 >> 7 & 0x1) as u8;
                        self.registers.pan_r[channel as usize] =
                            (self.io.data as i32 >> 6 & 0x1) as u8
                    }
                    _ => {}
                }
            }
        }

        if self.io.write_a_en as i32 != 0 || self.io.write_d_en as i32 != 0 {
            // Data
            if self.io.write_a_en != 0 {
                self.io.write_fm_data = 0;
            }

            if self.io.write_fm_address as i32 != 0 && self.io.write_d_en as i32 != 0 {
                self.io.write_fm_data = 1;
            }

            // Address
            if self.io.write_a_en != 0 {
                if self.io.write_data as i32 & 0xf0 != 0 {
                    // FM Write
                    self.io.address = self.io.write_data;
                    self.io.write_fm_address = 1;
                } else {
                    // SSG write
                    self.io.write_fm_address = 0;
                }
            }

            // FM Mode
            // Data
            if self.io.write_d_en as i32 != 0 && self.io.write_data as i32 & 0x100 == 0 {
                match self.io.write_fm_mode_a as i32 {
                    33 => {
                        // LSI test 1
                        i = 0;
                        while i < 8 {
                            self.registers.mode_test_21[i as usize] =
                                (self.io.write_data as i32 >> i & 0x1) as u8;
                            i = i.wrapping_add(1)
                        }
                    }
                    34 => {
                        // LFO control
                        if self.io.write_data as i32 >> 3 & 0x1 != 0 {
                            self.lfo.en = 0x7f;
                        } else {
                            self.lfo.en = 0;
                        }
                        self.lfo.freq = (self.io.write_data as i32 & 0x7) as u8
                    }
                    36 => {
                        // Timer A
                        self.timer_a.reg = (self.timer_a.reg as i32 & 0x3) as u16;
                        self.timer_a.reg = (self.timer_a.reg as i32
                            | (self.io.write_data as i32 & 0xff) << 2)
                            as u16
                    }
                    37 => {
                        self.timer_a.reg = (self.timer_a.reg as i32 & 0x3fc) as u16;
                        self.timer_a.reg =
                            (self.timer_a.reg as i32 | self.io.write_data as i32 & 0x3) as u16
                    }
                    38 => {
                        // Timer B
                        self.timer_b.reg = (self.io.write_data as i32 & 0xff) as u16
                    }
                    39 => {
                        // CSM, Timer control
                        self.registers.mode_ch3 = ((self.io.write_data as i32 & 0xc0) >> 6) as u8;
                        self.registers.mode_csm =
                            (self.registers.mode_ch3 as i32 == 2) as i32 as u8;
                        self.timer_a.load = (self.io.write_data as i32 & 0x1) as u8;
                        self.timer_a.enable = (self.io.write_data as i32 >> 2 & 0x1) as u8;
                        self.timer_a.reset = (self.io.write_data as i32 >> 4 & 0x1) as u8;
                        self.timer_b.load = (self.io.write_data as i32 >> 1 & 0x1) as u8;
                        self.timer_b.enable = (self.io.write_data as i32 >> 3 & 0x1) as u8;
                        self.timer_b.reset = (self.io.write_data as i32 >> 5 & 0x1) as u8
                    }
                    40 => {
                        // Key on/off
                        i = 0;
                        while i < 4 {
                            self.registers.mode_kon_operator[i as usize] =
                                (self.io.write_data as i32 >> (4u32).wrapping_add(i) & 0x1) as u8;
                            i = i.wrapping_add(1)
                        }
                        if self.io.write_data as i32 & 0x3 == 0x3 {
                            // Invalid address
                            self.registers.mode_kon_channel = 0xff;
                        } else {
                            self.registers.mode_kon_channel = ((self.io.write_data as i32 & 0x3)
                                + (self.io.write_data as i32 >> 2 & 1) * 3)
                                as u8
                        }
                    }
                    42 => {
                        // DAC data
                        self.registers.dacdata = (self.registers.dacdata as i32 & 0x1) as i16;
                        self.registers.dacdata = (self.registers.dacdata as i32
                            | (self.io.write_data as i32 ^ 0x80) << 1)
                            as i16
                    }
                    43 => {
                        // DAC enable
                        self.registers.dacen = (self.io.write_data as i32 >> 7) as u8
                    }
                    44 => {
                        // LSI test 2
                        i = 0;
                        while i < 8 {
                            self.registers.mode_test_2c[i as usize] =
                                (self.io.write_data as i32 >> i & 0x1) as u8;
                            i = i.wrapping_add(1)
                        }
                        self.registers.dacdata = (self.registers.dacdata as i32 & 0x1fe) as i16;
                        self.registers.dacdata = (self.registers.dacdata as i32
                            | self.registers.mode_test_2c[3] as i32)
                            as i16;
                        self.envelope_generator.custom_timer = (self.registers.mode_test_2c[7] == 0
                            && self.registers.mode_test_2c[6] as i32 != 0)
                            as i32
                            as u8
                    }
                    _ => {}
                }
            }
            // Address
            if self.io.write_a_en != 0 {
                self.io.write_fm_mode_a = (self.io.write_data as i32 & 0x1ff) as u16
            }
        }
        if self.io.write_fm_data != 0 {
            self.io.data = (self.io.write_data as i32 & 0xff) as u8
        };
    }

    pub fn ch_output(&mut self, ym2612: bool) {
        let cycles = self.cycles;
        let slot = self.cycles;
        let mut channel = self.channel;
        let test_dac = self.registers.mode_test_2c[5] as u32;
        self.ch.read = self.ch.lock;
        if slot < 12 {
            // Ch 4,5,6
            channel = channel.wrapping_add(1)
        }
        if cycles & 3 == 0 {
            if test_dac == 0 {
                // Lock value
                self.ch.lock = self.ch.out[channel as usize];
            }
            self.ch.lock_l = self.registers.pan_l[channel as usize];
            self.ch.lock_r = self.registers.pan_r[channel as usize];
        }

        // Ch 6
        let mut out;
        if cycles >> 2 == 1 && self.registers.dacen as i32 != 0 || test_dac != 0 {
            out = self.registers.dacdata;
            out = ((out as i32) << 7) as i16;
            out = (out as i32 >> 7) as i16;
        } else {
            out = self.ch.lock;
        }
        self.mol = 0;
        self.mor = 0;
        if ym2612 {
            let out_en = (cycles & 3 == 3 || test_dac != 0) as i32 as u32;
            // YM2612 DAC emulation(not verified)
            let mut sign = (out as i32 >> 8) as i16;
            if out as i32 >= 0 {
                out += 1;
                sign += 1
            }
            if self.ch.lock_l as i32 != 0 && out_en != 0 {
                self.mol = out;
            } else {
                self.mol = sign;
            }
            if self.ch.lock_r as i32 != 0 && out_en != 0 {
                self.mor = out;
            } else {
                self.mor = sign;
            }
            // Amplify signal
            self.mol = (self.mol as i32 * 3) as i16;
            self.mor = (self.mor as i32 * 3) as i16;
        } else {
            let out_en = (cycles & 3 != 0 || test_dac != 0) as i32 as u32;
            if self.ch.lock_l as i32 != 0 && out_en != 0 {
                self.mol = out;
            }
            if self.ch.lock_r as i32 != 0 && out_en != 0 {
                self.mor = out;
            }
        };
    }

    pub fn key_on(&mut self) {
        let slot = self.cycles;
        let chan = self.channel;

        // Key On
        self.envelope_generator.kon_latch[slot as usize] = self.registers.mode_kon[slot as usize];
        self.envelope_generator.kon_csm[slot as usize] = 0;

        if self.channel == 2 && self.registers.mode_kon_csm as i32 != 0 {
            // CSM Key On
            self.envelope_generator.kon_latch[slot as usize] = 1;
            self.envelope_generator.kon_csm[slot as usize] = 1;
        }

        if self.cycles == self.registers.mode_kon_channel as u32 {
            // OP1
            self.registers.mode_kon[chan as usize] = self.registers.mode_kon_operator[0];
            // OP2
            self.registers.mode_kon[chan.wrapping_add(12) as usize] =
                self.registers.mode_kon_operator[1];
            // OP3
            self.registers.mode_kon[chan.wrapping_add(6) as usize] =
                self.registers.mode_kon_operator[2];
            // OP4
            self.registers.mode_kon[chan.wrapping_add(18) as usize] =
                self.registers.mode_kon_operator[3]
        };
    }
}
