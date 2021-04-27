use crate::ym3438::Ym3438;
use bitflags::bitflags;

bitflags! {
    pub struct Mode: u32 {
        /// No special behavior
        const NONE = 0x00;
        /// Enables YM2612 emulation (MD1, MD2 VA2)
        const YM2612 = 0x01;
        /// Enables status read on any port (TeraDrive, MD1 VA7, MD2, etc)
        const READMODE = 0x02;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Opl2 {
    pub chip: Ym3438,
    pub mode: Mode,
}

impl Default for Opl2 {
    fn default() -> Self {
        Opl2 {
            chip: Default::default(),
            mode: Mode::READMODE,
        }
    }
}

impl Opl2 {
    pub fn reset(&mut self) {
        self.chip = Ym3438::default();
    }

    pub fn set_chip_type(&mut self, chip_type: Mode) {
        self.mode = chip_type;
    }

    pub fn clock(&mut self, buffer: &mut [i16]) {
        let slot: u32 = self.chip.cycles;
        self.chip.lfo_inc = self.chip.mode_test_21[1];
        self.chip.pg_read = self.chip.pg_read.checked_shr(1).unwrap_or_default();
        self.chip.eg_read[1] = self.chip.eg_read[1].checked_shr(1).unwrap_or_default();
        self.chip.eg_cycle = self.chip.eg_cycle.wrapping_add(1);

        /* Lock envelope generator timer value */
        if self.chip.cycles == 1 && self.chip.eg_quotient == 2 {
            if self.chip.eg_cycle_stop > 0 {
                self.chip.eg_shift_lock = 0
            } else {
                self.chip.eg_shift_lock = self.chip.eg_shift.wrapping_add(1)
            }
            self.chip.eg_timer_low_lock = (self.chip.eg_timer & 0x03) as u8;
        }

        /* Cycle specific functions */
        match self.chip.cycles {
            0 => {
                self.chip.lfo_pm = self.chip.lfo_cnt.checked_shr(2).unwrap_or_default();
                if self.chip.lfo_cnt & 0x40 > 0 {
                    self.chip.lfo_am = self.chip.lfo_cnt & 0x3f
                } else {
                    self.chip.lfo_am = self.chip.lfo_cnt ^ 0x3f
                }
                self.chip.lfo_am = self.chip.lfo_am.checked_shl(1).unwrap_or(u8::MAX);
            }
            1 => {
                self.chip.eg_quotient = self.chip.eg_quotient.wrapping_add(1);
                self.chip.eg_quotient %= 3;
                self.chip.eg_cycle = 0;
                self.chip.eg_cycle_stop = 1;
                self.chip.eg_shift = 0;
                self.chip.eg_timer_inc |=
                    self.chip.eg_quotient.checked_shr(1).unwrap_or_default() as u8;
                self.chip.eg_timer = self
                    .chip
                    .eg_timer
                    .wrapping_add(self.chip.eg_timer_inc as u16);
                self.chip.eg_timer_inc =
                    self.chip.eg_timer.checked_shr(12).unwrap_or_default() as u8;
                self.chip.eg_timer &= 0xfff
            }
            2 => {
                self.chip.pg_read = self.chip.pg_phase[21] & 0x3ff;
                self.chip.eg_read[1] = self.chip.eg_out[0] as u32;
            }
            13 => {
                self.chip.eg_cycle = 0;
                self.chip.eg_cycle_stop = 1;
                self.chip.eg_shift = 0;
                self.chip.eg_timer = self
                    .chip
                    .eg_timer
                    .wrapping_add(self.chip.eg_timer_inc as u16);
                self.chip.eg_timer_inc =
                    (self.chip.eg_timer.checked_shr(12).unwrap_or_default()) as u8;
                self.chip.eg_timer &= 0xfff;
            }
            23 => self.chip.lfo_inc |= 1,
            _ => {}
        }

        self.chip.eg_timer &= !((self.chip.mode_test_21[5] as u16)
            .checked_shl(self.chip.eg_cycle as u32)
            .unwrap_or(u16::MAX));

        if (((self
            .chip
            .eg_timer
            .checked_shr(self.chip.eg_cycle as u32)
            .unwrap_or_default())
            | (self.chip.pin_test_in & self.chip.eg_custom_timer) as u16)
            & self.chip.eg_cycle_stop as u16)
            > 0
        {
            self.chip.eg_shift = self.chip.eg_cycle;
            self.chip.eg_cycle_stop = 0
        }

        self.chip.do_io();

        self.chip.do_timer_a();
        self.chip.do_timer_b();
        self.chip.key_on();

        self.chip.ch_output(self.mode & Mode::YM2612 != Mode::NONE);
        self.chip.ch_generate();

        self.chip.fm_prepare();
        self.chip.fm_generate();

        self.chip.phase_generate();
        self.chip.phase_calc_increment();

        self.chip.envelope_adsr();
        self.chip.envelope_generate();
        self.chip.envelope_ssg_eg();
        self.chip.envelope_prepare();

        /* Prepare fnum & block */
        if self.chip.mode_ch3 > 0 {
            /* Channel 3 special mode */
            match slot {
                1 => {
                    /* OP1 */
                    self.chip.pg_fnum = self.chip.fnum_3ch[1];
                    self.chip.pg_block = self.chip.block_3ch[1];
                    self.chip.pg_kcode = self.chip.kcode_3ch[1]
                }
                7 => {
                    /* OP3 */
                    self.chip.pg_fnum = self.chip.fnum_3ch[0];
                    self.chip.pg_block = self.chip.block_3ch[0];
                    self.chip.pg_kcode = self.chip.kcode_3ch[0]
                }
                13 => {
                    /* OP2 */
                    self.chip.pg_fnum = self.chip.fnum_3ch[2];
                    self.chip.pg_block = self.chip.block_3ch[2];
                    self.chip.pg_kcode = self.chip.kcode_3ch[2]
                }
                _ => {
                    /* OP4 */
                    self.chip.pg_fnum =
                        self.chip.fnum[(self.chip.channel.wrapping_add(1) % 6) as usize];
                    self.chip.pg_block =
                        self.chip.block[(self.chip.channel.wrapping_add(1) % 6) as usize];
                    self.chip.pg_kcode =
                        self.chip.kcode[(self.chip.channel.wrapping_add(1) % 6) as usize]
                }
            }
        } else {
            self.chip.pg_fnum = self.chip.fnum[(self.chip.channel.wrapping_add(1) % 6) as usize];
            self.chip.pg_block = self.chip.block[(self.chip.channel.wrapping_add(1) % 6) as usize];
            self.chip.pg_kcode = self.chip.kcode[(self.chip.channel.wrapping_add(1) % 6) as usize]
        }
        self.chip.update_lfo();
        self.chip.do_reg_write();
        self.chip.cycles = self.chip.cycles.wrapping_add(1) % 24;
        self.chip.channel = self.chip.cycles % 6;

        buffer[0] = self.chip.mol;
        buffer[1] = self.chip.mor;

        if self.chip.status_time > 0 {
            self.chip.status_time = self.chip.status_time.wrapping_sub(1)
        };
    }

    pub fn write(&mut self, mut port: u32, data: u8) {
        port &= 3;
        self.chip.write_data =
            ((port.checked_shl(7).unwrap_or(u32::MAX)) & 0x100 | data as u32) as u16;
        if port & 1 > 0 {
            /* Data */
            self.chip.write_d |= 1
        } else {
            /* Address */
            self.chip.write_a |= 1
        };
    }

    pub fn set_test_pin(&mut self, value: u32) {
        self.chip.pin_test_in = (value & 1) as u8;
    }

    pub fn read_test_pin(&self) -> bool {
        if self.chip.mode_test_2c[7] == 0 {
            return false;
        }
        self.chip.cycles == 23
    }

    pub fn read_irq_pin(&self) -> u32 {
        (self.chip.timer_a_overflow_flag | self.chip.timer_b_overflow_flag) as u32
    }

    pub fn read(&mut self, port: u32) -> u8 {
        if port & 3 == 0 || self.mode & Mode::READMODE != Mode::NONE {
            if self.chip.mode_test_21[6] > 0 {
                /* Read test data */
                let slot: u32 = self.chip.cycles.wrapping_add(18) % 24;
                let mut testdata: u16 = ((self.chip.pg_read & 0x1)
                    .checked_shl(15)
                    .unwrap_or(u32::MAX)
                    | (self.chip.eg_read[self.chip.mode_test_21[0] as usize] & 0x1)
                        .checked_shl(14)
                        .unwrap_or(u32::MAX)) as u16;
                if self.chip.mode_test_2c[4] > 0 {
                    testdata |= (self.chip.ch_read & 0x1ff) as u16
                } else {
                    testdata |= (self.chip.fm_out[slot as usize] & 0x3fff) as u16
                }
                if self.chip.mode_test_21[7] > 0 {
                    self.chip.status = (testdata & 0xff) as u8
                } else {
                    self.chip.status = testdata.checked_shr(8).unwrap_or_default() as u8
                }
            } else {
                self.chip.status = (self.chip.busy.checked_shl(7).unwrap_or(u8::MAX))
                    | (self
                        .chip
                        .timer_b_overflow_flag
                        .checked_shl(1)
                        .unwrap_or(u8::MAX))
                    | self.chip.timer_a_overflow_flag
            }
            if self.mode & Mode::YM2612 != Mode::NONE {
                self.chip.status_time = 300000
            } else {
                self.chip.status_time = 40000000
            }
        }
        if self.chip.status_time != 0 {
            return self.chip.status;
        }
        0
    }
}
