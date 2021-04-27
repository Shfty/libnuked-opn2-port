use crate::Registers;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerA {
    pub cnt: u16,
    pub reg: u16,
    pub load_lock: u8,
    pub load: u8,
    pub enable: u8,
    pub reset: u8,
    pub load_latch: u8,
    pub overflow_flag: u8,
    pub overflow: u8,
}

impl TimerA {
    pub fn clock(&mut self, cycles: u32, registers: &mut Registers) {
        let mut time: u16;
        let mut load = self.overflow;
        if cycles == 2 {
            // Lock load value
            load = (load as i32 | (self.load_lock == 0 && self.load as i32 != 0) as i32) as u8;
            self.load_lock = self.load;
            if registers.mode_csm != 0 {
                // CSM KeyOn
                registers.mode_kon_csm = load
            } else {
                registers.mode_kon_csm = 0;
            }
        }
        // Load counter
        if self.load_latch != 0 {
            time = self.reg
        } else {
            time = self.cnt
        }
        self.load_latch = load;

        // Increase counter
        if cycles == 1 && self.load_lock as i32 != 0
            || registers.mode_test_21[2] as i32 != 0
        {
            time = time.wrapping_add(1)
        }

        // Set overflow flag
        if self.reset != 0 {
            self.reset = 0;
            self.overflow_flag = 0;
        } else {
            self.overflow_flag =
                (self.overflow_flag as i32 | self.overflow as i32 & self.enable as i32) as u8
        }
        self.overflow = (time as i32 >> 10) as u8;
        self.cnt = (time as i32 & 0x3ff) as u16;
    }
}
