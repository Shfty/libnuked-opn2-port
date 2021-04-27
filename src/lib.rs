pub mod opl2;
pub mod ym3438;
pub mod ym3438_raw;

use crate::opl2::Opl2;
use ::opl2::OPL2Chip;
use ym3438_raw::Ym3438Raw;

impl OPL2Chip for Opl2 {
    /// Reset emulated chip
    fn reset(&mut self) {
        Opl2::reset(self)
    }

    /// Advances emulated chip state by 1 internal clock (6 master clocks). Writes signed 9-bit MOL, MOR pin states to buffer.
    fn clock(&mut self, buffer: &mut [i16]) {
        Opl2::clock(self, buffer)
    }

    /// Write 8-bit data to port.
    fn write(&mut self, port: u32, data: u8) {
        Opl2::write(self, port, data);
    }

    /// Set TEST pin value.
    fn set_test_pin(&mut self, value: u32) {
        Opl2::set_test_pin(self, value);
    }

    /// Read TEST pin value.
    fn read_test_pin(&self) -> u32 {
        Opl2::read_test_pin(self) as u32
    }

    /// Read IRQ pin value.
    fn read_irq_pin(&self) -> u32 {
        Opl2::read_irq_pin(self)
    }

    /// Read chip status.
    fn read(&mut self, port: u32) -> u8 {
        Opl2::read(self, port)
    }
}

pub struct Opl2Raw {
    chip: Ym3438Raw,
}

impl Default for Opl2Raw {
    fn default() -> Self {
        let chip: Ym3438Raw = unsafe { std::mem::zeroed() };
        Opl2Raw { chip }
    }
}

impl OPL2Chip for Opl2Raw {
    /// Reset emulated chip
    fn reset(&mut self) {
        self.chip.opn2_reset()
    }

    /// Advances emulated chip state by 1 internal clock (6 master clocks). Writes signed 9-bit MOL, MOR pin states to buffer.
    fn clock(&mut self, buffer: &mut [i16]) {
        self.chip.opn2_clock(buffer)
    }

    /// Write 8-bit data to port.
    fn write(&mut self, port: u32, data: u8) {
        self.chip.opn2_write(port, data)
    }

    /// Set TEST pin value.
    fn set_test_pin(&mut self, value: u32) {
        self.chip.opn2_set_test_pin(value)
    }

    /// Read TEST pin value.
    fn read_test_pin(&self) -> u32 {
        self.chip.opn2_read_test_pin()
    }

    /// Read IRQ pin value.
    fn read_irq_pin(&self) -> u32 {
        self.chip.opn2_read_irq_pin()
    }

    /// Read chip status.
    fn read(&mut self, port: u32) -> u8 {
        self.chip.opn2_read(port)
    }
}
