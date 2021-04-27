use crate::{
    rom::{EXP_ROM, FM_ALGORITHM, LOGSIN_ROM},
    EnvelopeGenerator, PhaseGenerator, Registers,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fm {
    pub op1: [[i16; 2]; 6],
    pub op2: [i16; 6],
    pub out: [i16; 24],
    pub modulation: [u16; 24],
}

impl Fm {
    pub fn prepare(&mut self, cycles: u32, channel: u32, registers: &Registers) {
        let mut slot = cycles
            .wrapping_add(6)
            .wrapping_rem(24);
        let channel = channel;
        let op = slot.wrapping_div(6);
        let connect = registers.connect[channel as usize];
        let prevslot = cycles
            .wrapping_add(18)
            .wrapping_rem(24);

        // Calculate modulation
        let mut mod2 = 0;
        let mut mod1 = mod2;
        if FM_ALGORITHM[op as usize][0][connect as usize] != 0 {
            mod2 = (mod2 as i32 | self.op1[channel as usize][0] as i32) as i16
        }
        if FM_ALGORITHM[op as usize][1][connect as usize] != 0 {
            mod1 = (mod1 as i32 | self.op1[channel as usize][1] as i32) as i16
        }
        if FM_ALGORITHM[op as usize][2][connect as usize] != 0 {
            mod1 = (mod1 as i32 | self.op2[channel as usize] as i32) as i16
        }
        if FM_ALGORITHM[op as usize][3][connect as usize] != 0 {
            mod2 = (mod2 as i32 | self.out[prevslot as usize] as i32) as i16
        }
        if FM_ALGORITHM[op as usize][4][connect as usize] != 0 {
            mod1 = (mod1 as i32 | self.out[prevslot as usize] as i32) as i16
        }
        let mut mod_0 = (mod1 as i32 + mod2 as i32) as i16;
        if op == 0 {
            // Feedback
            mod_0 = (mod_0 as i32 >> (10 - registers.fb[channel as usize] as i32)) as i16;
            if registers.fb[channel as usize] == 0 {
                mod_0 = 0;
            }
        } else {
            mod_0 = (mod_0 as i32 >> 1) as i16
        }
        self.modulation[slot as usize] = mod_0 as u16;
        slot = cycles
            .wrapping_add(18)
            .wrapping_rem(24);

        // OP1
        if slot.wrapping_div(6) == 0 {
            self.op1[channel as usize][1] = self.op1[channel as usize][0];
            self.op1[channel as usize][0] = self.out[slot as usize]
        }

        // OP2
        if slot.wrapping_div(6) == 2 {
            self.op2[channel as usize] = self.out[slot as usize]
        };
    }

    pub fn generate(
        &mut self,
        cycles: u32,
        registers: &Registers,
        phase_generator: &PhaseGenerator,
        envelope_generator: &EnvelopeGenerator,
    ) {
        let slot = cycles
            .wrapping_add(19)
            .wrapping_rem(24);

        // Calculate phase
        let phase = ((self.modulation[slot as usize] as u32)
            .wrapping_add(phase_generator.phase[slot as usize] >> 10)
            & 0x3ff) as u16;
        let quarter = if phase as i32 & 0x100 != 0 {
            ((phase as i32 ^ 0xff) & 0xff) as u16
        } else {
            (phase as i32 & 0xff) as u16
        };
        let mut level = LOGSIN_ROM[quarter as usize];

        // Apply envelope
        level = (level as i32 + ((envelope_generator.out[slot as usize] as i32) << 2)) as u16;

        // Transform
        if level as i32 > 0x1fff {
            level = 0x1fff;
        }
        let mut output =
            ((EXP_ROM[(level as i32 & 0xff ^ 0xff) as usize] as i32 | 0x400) << 2
                >> (level as i32 >> 8)) as i16;
        if phase as i32 & 0x200 != 0 {
            output =
                ((!(output as i32) ^ (registers.mode_test_21[4] as i32) << 13) + 1) as i16
        } else {
            output = (output as i32 ^ (registers.mode_test_21[4] as i32) << 13) as i16
        }
        output = ((output as i32) << 2) as i16;
        output = (output as i32 >> 2) as i16;
        self.out[slot as usize] = output;
    }
}
