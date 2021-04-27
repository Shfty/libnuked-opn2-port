use crate::Ym3438;

use bitflags::bitflags;

bitflags! {
    pub struct Mode: u32 {
        const NONE = 0;
        const YM2612 = 1;
        const READMODE = 2;
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::READMODE
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Opn2 {
    pub chip: Ym3438,
    pub mode: Mode,
}

impl Opn2 {
    pub fn reset(&mut self) {
        self.chip = Ym3438::default();
    }

    pub fn set_chip_type(&mut self, chip_type: u32) {
        self.mode = Mode::from_bits(chip_type).unwrap()
    }

    pub fn clock(&mut self, buffer: &mut [i16]) {
        let slot = self.chip.cycles;
        self.chip.lfo.inc = self.chip.registers.mode_test_21[1];
        self.chip.phase_generator.read >>= 1;
        self.chip.envelope_generator.read[1] >>= 1;
        self.chip.envelope_generator.cycle = self.chip.envelope_generator.cycle.wrapping_add(1);

        // Cycle specific functions
        match self.chip.cycles {
            0 => self.chip.lfo.cycle_0(),
            1 => self.chip.envelope_generator.cycle_1(),
            2 => {
                self.chip.phase_generator.cycle_2();
                self.chip.envelope_generator.cycle_2();
            }
            13 => self.chip.envelope_generator.cycle_13(),
            23 => self.chip.lfo.cycle_23(),
            _ => {}
        }

        self.chip.envelope_generator.timer = (self.chip.envelope_generator.timer as i32
            & !((self.chip.registers.mode_test_21[5] as i32)
                << self.chip.envelope_generator.cycle as i32))
            as u16;

        #[allow(clippy::suspicious_operation_groupings)]
        if (self.chip.envelope_generator.timer as i32 >> self.chip.envelope_generator.cycle as i32
            | self.chip.io.pin_test_in as i32 & self.chip.envelope_generator.custom_timer as i32)
            & self.chip.envelope_generator.cycle_stop as i32
            != 0
        {
            self.chip.envelope_generator.shift = self.chip.envelope_generator.cycle;
            self.chip.envelope_generator.cycle_stop = 0;
        }

        self.chip.io.clock();
        
        self.chip
            .timer_a
            .clock(self.chip.cycles, &mut self.chip.registers);

        self.chip
            .timer_b
            .clock(self.chip.cycles, &self.chip.registers);

        self.chip.key_on();

        self.chip.ch_output(self.mode & Mode::YM2612 != Mode::NONE);

        self.chip.ch.generate(
            self.chip.cycles,
            self.chip.channel,
            &self.chip.registers,
            &self.chip.fm,
        );

        self.chip
            .fm
            .prepare(self.chip.cycles, self.chip.channel, &self.chip.registers);

        self.chip.fm.generate(
            self.chip.cycles,
            &self.chip.registers,
            &self.chip.phase_generator,
            &self.chip.envelope_generator,
        );

        self.chip
            .phase_generator
            .generate(self.chip.cycles, &self.chip.registers);

        self.chip.phase_generator.phase_calc_increment(
            self.chip.cycles,
            self.chip.channel,
            &self.chip.registers,
            &self.chip.lfo,
        );

        self.chip
            .envelope_generator
            .adsr(self.chip.cycles, &mut self.chip.phase_generator);

        self.chip.envelope_generator.generate(
            self.chip.cycles,
            self.chip.channel,
            &self.chip.registers,
        );

        self.chip
            .envelope_generator
            .ssg_eg(self.chip.cycles, &self.chip.registers);

        self.chip.envelope_generator.prepare(
            self.chip.cycles,
            self.chip.channel,
            &self.chip.registers,
            &self.chip.phase_generator,
            &self.chip.lfo,
        );

        // Prepare fnum & block
        if self.chip.registers.mode_ch3 != 0 {
            // Channel 3 special mode
            match slot {
                1 => {
                    // OP1
                    self.chip.phase_generator.fnum = self.chip.registers.fnum_3ch[1];
                    self.chip.phase_generator.block = self.chip.registers.block_3ch[1];
                    self.chip.phase_generator.kcode = self.chip.registers.kcode_3ch[1]
                }
                7 => {
                    // OP3
                    self.chip.phase_generator.fnum = self.chip.registers.fnum_3ch[0];
                    self.chip.phase_generator.block = self.chip.registers.block_3ch[0];
                    self.chip.phase_generator.kcode = self.chip.registers.kcode_3ch[0]
                }
                13 => {
                    // OP2
                    self.chip.phase_generator.fnum = self.chip.registers.fnum_3ch[2];
                    self.chip.phase_generator.block = self.chip.registers.block_3ch[2];
                    self.chip.phase_generator.kcode = self.chip.registers.kcode_3ch[2]
                }
                _ => {
                    // OP4
                    self.chip.phase_generator.fnum = self.chip.registers.fnum
                        [self.chip.channel.wrapping_add(1).wrapping_rem(6) as usize];
                    self.chip.phase_generator.block = self.chip.registers.block
                        [self.chip.channel.wrapping_add(1).wrapping_rem(6) as usize];
                    self.chip.phase_generator.kcode = self.chip.registers.kcode
                        [self.chip.channel.wrapping_add(1).wrapping_rem(6) as usize]
                }
            }
        } else {
            self.chip.phase_generator.fnum = self.chip.registers.fnum
                [self.chip.channel.wrapping_add(1).wrapping_rem(6) as usize];
            self.chip.phase_generator.block = self.chip.registers.block
                [self.chip.channel.wrapping_add(1).wrapping_rem(6) as usize];
            self.chip.phase_generator.kcode = self.chip.registers.kcode
                [self.chip.channel.wrapping_add(1).wrapping_rem(6) as usize]
        }

        self.chip.lfo.update();
        self.chip.do_reg_write();
        self.chip.cycles = self.chip.cycles.wrapping_add(1).wrapping_rem(24);
        self.chip.channel = self.chip.cycles.wrapping_rem(6);
        buffer[0] = self.chip.mol;
        buffer[1] = self.chip.mor;

        if self.chip.registers.status_time != 0 {
            self.chip.registers.status_time = self.chip.registers.status_time.wrapping_sub(1)
        };
    }

    pub fn write(&mut self, mut port: u32, data: u8) {
        port &= 3;
        self.chip.io.write_data = (port << 7 & 0x100 | data as u32) as u16;
        if port & 1 != 0 {
            // Data
            self.chip.io.write_d = (self.chip.io.write_d as i32 | 1) as u8
        } else {
            // Address
            self.chip.io.write_a = (self.chip.io.write_a as i32 | 1) as u8
        };
    }

    pub fn set_test_pin(&mut self, value: u32) {
        self.chip.io.pin_test_in = (value & 1) as u8;
    }

    pub fn read_test_pin(&self) -> u32 {
        if self.chip.registers.mode_test_2c[7] == 0 {
            return 0;
        }

        (self.chip.cycles == 23) as u32
    }

    pub fn read_irq_pin(&self) -> u32 {
        (self.chip.timer_a.overflow_flag as i32 | self.chip.timer_b.overflow_flag as i32) as u32
    }

    pub fn read(&mut self, port: u32) -> u8 {
        if port & 3 == 0 || self.mode & Mode::READMODE != Mode::NONE {
            if self.chip.registers.mode_test_21[6] != 0 {
                // Read test data
                let slot = self.chip.cycles.wrapping_add(18).wrapping_rem(24);

                let mut testdata = ((self.chip.phase_generator.read & 0x1) << 15
                    | (self.chip.envelope_generator.read
                        [self.chip.registers.mode_test_21[0] as usize]
                        & 0x1)
                        << 14) as u16;

                if self.chip.registers.mode_test_2c[4] != 0 {
                    testdata = (testdata as i32 | self.chip.ch.read as i32 & 0x1ff) as u16
                } else {
                    testdata =
                        (testdata as i32 | self.chip.fm.out[slot as usize] as i32 & 0x3fff) as u16
                }

                if self.chip.registers.mode_test_21[7] != 0 {
                    self.chip.registers.status = (testdata as i32 & 0xff) as u8
                } else {
                    self.chip.registers.status = (testdata as i32 >> 8) as u8
                }
            } else {
                self.chip.registers.status = ((self.chip.io.busy as i32) << 7
                    | (self.chip.timer_b.overflow_flag as i32) << 1
                    | self.chip.timer_a.overflow_flag as i32)
                    as u8
            }

            if self.mode & Mode::YM2612 != Mode::NONE {
                self.chip.registers.status_time = 300000
            } else {
                self.chip.registers.status_time = 40000000
            }
        }

        if self.chip.registers.status_time != 0 {
            return self.chip.registers.status;
        }

        0
    }
}
