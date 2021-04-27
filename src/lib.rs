mod opn2;
pub mod rom;
mod ym3438;

pub use opn2::*;
pub use ym3438::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Adsr {
    Attack,
    Decay,
    Sustain,
    Release,
}
