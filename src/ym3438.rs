pub const EG_NUM_ATTACK: u8 = 0;
pub const EG_NUM_DECAY: u8 = 1;
pub const EG_NUM_SUSTAIN: u8 = 2;
pub const EG_NUM_RELEASE: u8 = 3;

// Logsin table
pub const LOGSIN_ROM: [u16; 256] = [
    0x859, 0x6c3, 0x607, 0x58b, 0x52e, 0x4e4, 0x4a6, 0x471, 0x443, 0x41a, 0x3f5, 0x3d3, 0x3b5,
    0x398, 0x37e, 0x365, 0x34e, 0x339, 0x324, 0x311, 0x2ff, 0x2ed, 0x2dc, 0x2cd, 0x2bd, 0x2af,
    0x2a0, 0x293, 0x286, 0x279, 0x26d, 0x261, 0x256, 0x24b, 0x240, 0x236, 0x22c, 0x222, 0x218,
    0x20f, 0x206, 0x1fd, 0x1f5, 0x1ec, 0x1e4, 0x1dc, 0x1d4, 0x1cd, 0x1c5, 0x1be, 0x1b7, 0x1b0,
    0x1a9, 0x1a2, 0x19b, 0x195, 0x18f, 0x188, 0x182, 0x17c, 0x177, 0x171, 0x16b, 0x166, 0x160,
    0x15b, 0x155, 0x150, 0x14b, 0x146, 0x141, 0x13c, 0x137, 0x133, 0x12e, 0x129, 0x125, 0x121,
    0x11c, 0x118, 0x114, 0x10f, 0x10b, 0x107, 0x103, 0xff, 0xfb, 0xf8, 0xf4, 0xf0, 0xec, 0xe9,
    0xe5, 0xe2, 0xde, 0xdb, 0xd7, 0xd4, 0xd1, 0xcd, 0xca, 0xc7, 0xc4, 0xc1, 0xbe, 0xbb, 0xb8, 0xb5,
    0xb2, 0xaf, 0xac, 0xa9, 0xa7, 0xa4, 0xa1, 0x9f, 0x9c, 0x99, 0x97, 0x94, 0x92, 0x8f, 0x8d, 0x8a,
    0x88, 0x86, 0x83, 0x81, 0x7f, 0x7d, 0x7a, 0x78, 0x76, 0x74, 0x72, 0x70, 0x6e, 0x6c, 0x6a, 0x68,
    0x66, 0x64, 0x62, 0x60, 0x5e, 0x5c, 0x5b, 0x59, 0x57, 0x55, 0x53, 0x52, 0x50, 0x4e, 0x4d, 0x4b,
    0x4a, 0x48, 0x46, 0x45, 0x43, 0x42, 0x40, 0x3f, 0x3e, 0x3c, 0x3b, 0x39, 0x38, 0x37, 0x35, 0x34,
    0x33, 0x31, 0x30, 0x2f, 0x2e, 0x2d, 0x2b, 0x2a, 0x29, 0x28, 0x27, 0x26, 0x25, 0x24, 0x23, 0x22,
    0x21, 0x20, 0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x17, 0x16, 0x15, 0x14, 0x14,
    0x13, 0x12, 0x11, 0x11, 0x10, 0xf, 0xf, 0xe, 0xd, 0xd, 0xc, 0xc, 0xb, 0xa, 0xa, 0x9, 0x9, 0x8,
    0x8, 0x7, 0x7, 0x7, 0x6, 0x6, 0x5, 0x5, 0x5, 0x4, 0x4, 0x4, 0x3, 0x3, 0x3, 0x2, 0x2, 0x2, 0x2,
    0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0, 0, 0, 0, 0, 0, 0, 0,
];

// Exp table
pub const EXP_ROM: [u16; 256] = [
    0, 0x3, 0x6, 0x8, 0xb, 0xe, 0x11, 0x14, 0x16, 0x19, 0x1c, 0x1f, 0x22, 0x25, 0x28, 0x2a, 0x2d,
    0x30, 0x33, 0x36, 0x39, 0x3c, 0x3f, 0x42, 0x45, 0x48, 0x4b, 0x4e, 0x51, 0x54, 0x57, 0x5a, 0x5d,
    0x60, 0x63, 0x66, 0x69, 0x6c, 0x6f, 0x72, 0x75, 0x78, 0x7b, 0x7e, 0x82, 0x85, 0x88, 0x8b, 0x8e,
    0x91, 0x94, 0x98, 0x9b, 0x9e, 0xa1, 0xa4, 0xa8, 0xab, 0xae, 0xb1, 0xb5, 0xb8, 0xbb, 0xbe, 0xc2,
    0xc5, 0xc8, 0xcc, 0xcf, 0xd2, 0xd6, 0xd9, 0xdc, 0xe0, 0xe3, 0xe7, 0xea, 0xed, 0xf1, 0xf4, 0xf8,
    0xfb, 0xff, 0x102, 0x106, 0x109, 0x10c, 0x110, 0x114, 0x117, 0x11b, 0x11e, 0x122, 0x125, 0x129,
    0x12c, 0x130, 0x134, 0x137, 0x13b, 0x13e, 0x142, 0x146, 0x149, 0x14d, 0x151, 0x154, 0x158,
    0x15c, 0x160, 0x163, 0x167, 0x16b, 0x16f, 0x172, 0x176, 0x17a, 0x17e, 0x181, 0x185, 0x189,
    0x18d, 0x191, 0x195, 0x199, 0x19c, 0x1a0, 0x1a4, 0x1a8, 0x1ac, 0x1b0, 0x1b4, 0x1b8, 0x1bc,
    0x1c0, 0x1c4, 0x1c8, 0x1cc, 0x1d0, 0x1d4, 0x1d8, 0x1dc, 0x1e0, 0x1e4, 0x1e8, 0x1ec, 0x1f0,
    0x1f5, 0x1f9, 0x1fd, 0x201, 0x205, 0x209, 0x20e, 0x212, 0x216, 0x21a, 0x21e, 0x223, 0x227,
    0x22b, 0x230, 0x234, 0x238, 0x23c, 0x241, 0x245, 0x249, 0x24e, 0x252, 0x257, 0x25b, 0x25f,
    0x264, 0x268, 0x26d, 0x271, 0x276, 0x27a, 0x27f, 0x283, 0x288, 0x28c, 0x291, 0x295, 0x29a,
    0x29e, 0x2a3, 0x2a8, 0x2ac, 0x2b1, 0x2b5, 0x2ba, 0x2bf, 0x2c4, 0x2c8, 0x2cd, 0x2d2, 0x2d6,
    0x2db, 0x2e0, 0x2e5, 0x2e9, 0x2ee, 0x2f3, 0x2f8, 0x2fd, 0x302, 0x306, 0x30b, 0x310, 0x315,
    0x31a, 0x31f, 0x324, 0x329, 0x32e, 0x333, 0x338, 0x33d, 0x342, 0x347, 0x34c, 0x351, 0x356,
    0x35b, 0x360, 0x365, 0x36a, 0x370, 0x375, 0x37a, 0x37f, 0x384, 0x38a, 0x38f, 0x394, 0x399,
    0x39f, 0x3a4, 0x3a9, 0x3ae, 0x3b4, 0x3b9, 0x3bf, 0x3c4, 0x3c9, 0x3cf, 0x3d4, 0x3da, 0x3df,
    0x3e4, 0x3ea, 0x3ef, 0x3f5, 0x3fa,
];

// Note table
pub const FN_NOTE: [u32; 16] = [0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 3, 3, 3, 3, 3];

// Envelope generator
pub const EG_STEP_HI: [[u32; 4]; 4] = [[0, 0, 0, 0], [1, 0, 0, 0], [1, 0, 1, 0], [1, 1, 1, 0]];
pub const EG_AM_SHIFT: [u8; 4] = [7, 3, 1, 0];

// Phase generator
pub const PG_DETUNE: [u32; 8] = [16, 17, 19, 20, 22, 24, 27, 29];
pub const PG_LFO_SH1: [[u32; 8]; 8] = [
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 7, 7, 1, 1],
    [7, 7, 7, 7, 1, 1, 1, 1],
    [7, 7, 7, 1, 1, 1, 1, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
];
pub const PG_LFO_SH2: [[u32; 8]; 8] = [
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 2, 2, 2, 2],
    [7, 7, 7, 2, 2, 2, 7, 7],
    [7, 7, 2, 2, 7, 7, 2, 2],
    [7, 7, 2, 7, 7, 7, 2, 7],
    [7, 7, 7, 2, 7, 7, 2, 1],
    [7, 7, 7, 2, 7, 7, 2, 1],
    [7, 7, 7, 2, 7, 7, 2, 1],
];

// Address decoder
pub const OP_OFFSET: [u32; 12] = [
    0, 0x1, 0x2, 0x100, 0x101, 0x102, 0x4, 0x5, 0x6, 0x104, 0x105, 0x106,
];
pub const CH_OFFSET: [u32; 6] = [0, 0x1, 0x2, 0x100, 0x101, 0x102];

// LFO
pub const LFO_CYCLES: [u32; 8] = [108, 77, 71, 67, 62, 44, 8, 5];

// FM algorithm
pub const FM_ALGORITHM: [[[u32; 8]; 6]; 4] = [
    [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1],
    ],
    [
        [0, 1, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 1, 1],
    ],
    [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 1, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 1, 1, 1],
    ],
    [
        [0, 0, 1, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [1, 1, 0, 1, 1, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ],
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ym3438 {
    pub cycles: u32,
    pub channel: u32,
    pub mol: i16,
    pub mor: i16,

    // IO
    pub write_data: u16,
    pub write_a: u8,
    pub write_d: u8,
    pub write_a_en: u8,
    pub write_d_en: u8,
    pub write_busy: u8,
    pub write_busy_cnt: u8,
    pub write_fm_address: u8,
    pub write_fm_data: u8,
    pub write_fm_mode_a: u16,
    pub address: u16,
    pub data: u8,
    pub pin_test_in: u8,
    pub pin_irq: u8,
    pub busy: u8,

    // LFO
    pub lfo_en: u8,
    pub lfo_freq: u8,
    pub lfo_pm: u8,
    pub lfo_am: u8,
    pub lfo_cnt: u8,
    pub lfo_inc: u8,
    pub lfo_quotient: u8,

    // Phase generator
    pub pg_fnum: u16,
    pub pg_block: u8,
    pub pg_kcode: u8,
    pub pg_inc: [u32; 24],
    pub pg_phase: [u32; 24],
    pub pg_reset: [u8; 24],
    pub pg_read: u32,

    // Envelope generator
    pub eg_cycle: u8,
    pub eg_cycle_stop: u8,
    pub eg_shift: u8,
    pub eg_shift_lock: u8,
    pub eg_timer_low_lock: u8,
    pub eg_timer: u16,
    pub eg_timer_inc: u8,
    pub eg_quotient: u16,
    pub eg_custom_timer: u8,
    pub eg_rate: u8,
    pub eg_ksv: u8,
    pub eg_inc: u8,
    pub eg_ratemax: u8,
    pub eg_sl: [u8; 2],
    pub eg_lfo_am: u8,
    pub eg_tl: [u8; 2],
    pub eg_state: [u8; 24],
    pub eg_level: [u16; 24],
    pub eg_out: [u16; 24],
    pub eg_kon: [u8; 24],
    pub eg_kon_csm: [u8; 24],
    pub eg_kon_latch: [u8; 24],
    pub eg_csm_mode: [u8; 24],
    pub eg_ssg_enable: [u8; 24],
    pub eg_ssg_pgrst_latch: [u8; 24],
    pub eg_ssg_repeat_latch: [u8; 24],
    pub eg_ssg_hold_up_latch: [u8; 24],
    pub eg_ssg_dir: [u8; 24],
    pub eg_ssg_inv: [u8; 24],
    pub eg_read: [u32; 2],
    pub eg_read_inc: u8,

    // FM
    pub fm_op1: [[i16; 2]; 6],
    pub fm_op2: [i16; 6],
    pub fm_out: [i16; 24],
    pub fm_mod: [u16; 24],

    // Channel
    pub ch_acc: [i16; 6],
    pub ch_out: [i16; 6],
    pub ch_lock: i16,
    pub ch_lock_l: u8,
    pub ch_lock_r: u8,
    pub ch_read: i16,

    // Timer
    pub timer_a_cnt: u16,
    pub timer_a_reg: u16,
    pub timer_a_load_lock: u8,
    pub timer_a_load: u8,
    pub timer_a_enable: u8,
    pub timer_a_reset: u8,
    pub timer_a_load_latch: u8,
    pub timer_a_overflow_flag: u8,
    pub timer_a_overflow: u8,

    pub timer_b_cnt: u16,
    pub timer_b_subcnt: u8,
    pub timer_b_reg: u16,
    pub timer_b_load_lock: u8,
    pub timer_b_load: u8,
    pub timer_b_enable: u8,
    pub timer_b_reset: u8,
    pub timer_b_load_latch: u8,
    pub timer_b_overflow_flag: u8,
    pub timer_b_overflow: u8,

    // Register set
    pub mode_test_21: [u8; 8],
    pub mode_test_2c: [u8; 8],
    pub mode_ch3: u8,
    pub mode_kon_channel: u8,
    pub mode_kon_operator: [u8; 4],
    pub mode_kon: [u8; 24],
    pub mode_csm: u8,
    pub mode_kon_csm: u8,
    pub dacen: u8,
    pub dacdata: i16,

    pub ks: [u8; 24],
    pub ar: [u8; 24],
    pub sr: [u8; 24],
    pub dt: [u8; 24],
    pub multi: [u8; 24],
    pub sl: [u8; 24],
    pub rr: [u8; 24],
    pub dr: [u8; 24],
    pub am: [u8; 24],
    pub tl: [u8; 24],
    pub ssg_eg: [u8; 24],

    pub fnum: [u16; 6],
    pub block: [u8; 6],
    pub kcode: [u8; 6],
    pub fnum_3ch: [u16; 6],
    pub block_3ch: [u8; 6],
    pub kcode_3ch: [u8; 6],
    pub reg_a4: u8,
    pub reg_ac: u8,
    pub connect: [u8; 6],
    pub fb: [u8; 6],
    pub pan_l: [u8; 6],
    pub pan_r: [u8; 6],
    pub ams: [u8; 6],
    pub pms: [u8; 6],
    pub status: u8,
    pub status_time: u32,
}

impl Default for Ym3438 {
    fn default() -> Self {
        let mut chip = unsafe { std::mem::zeroed::<Self>() };

        for i in 0usize..24 {
            chip.eg_out[i] = 0x3ff;
            chip.eg_level[i] = 0x3ff;
            chip.eg_state[i] = EG_NUM_RELEASE;
            chip.multi[i] = 1;
        }

        for i in 0usize..6 {
            chip.pan_l[i] = 1;
            chip.pan_r[i] = 1;
        }

        chip
    }
}

impl Ym3438 {
    pub fn do_io(&mut self) {
        // Write signal check
        self.write_a_en = (self.write_a & 0x03 == 0x01) as u8;
        self.write_d_en = (self.write_d & 0x03 == 0x01) as u8;
        self.write_a <<= 1;
        self.write_d <<= 1;
        // Busy counter
        self.busy = self.write_busy;
        self.write_busy_cnt = self.write_busy_cnt.wrapping_add(self.write_busy);
        self.write_busy =
            ((self.write_busy > 0 && self.write_busy_cnt >> 5 == 0) || self.write_d_en > 0) as u8;
        self.write_busy_cnt &= 0x1f;
    }

    pub fn do_reg_write(&mut self) {
        let mut slot = (self.cycles % 12) as usize;
        let mut address: u32;
        let channel = self.channel as usize;

        // Update registers
        if self.write_fm_data > 0 {
            // Slot
            if OP_OFFSET[slot] == (self.address & 0x107) as u32 {
                if self.address & 0x8 > 0 {
                    // OP2, OP4
                    slot = slot.wrapping_add(12);
                }
                address = self.address as u32 & 0xf0;
                match address {
                    0x30 => {
                        // DT, MULTI
                        self.multi[slot] = self.data & 0x0f;
                        if self.multi[slot] == 0 {
                            self.multi[slot] = 1
                        } else {
                            self.multi[slot] <<= 1
                        }
                        self.dt[slot] = (self.data >> 4) & 0x07;
                    }
                    0x40 => {
                        // TL
                        self.tl[slot] = self.data & 0x07f;
                    }
                    0x50 => {
                        // KS, AR
                        self.ar[slot] = self.data & 0x1f;
                        self.ks[slot] = (self.data >> 6) & 0x03;
                    }
                    0x60 => {
                        // AM, DR
                        self.dr[slot] = self.data & 0x1f;
                        self.am[slot] = (self.data >> 7) & 0x01;
                    }
                    0x70 => {
                        // SR
                        self.sr[slot] = self.data & 0x1f;
                    }
                    0x80 => {
                        // SL, RR
                        self.rr[slot] = self.data & 0x0f;
                        self.sl[slot] = (self.data >> 4) & 0x0f;
                        self.sl[slot] |= (self.sl[slot].wrapping_add(1)) & 0x10;
                    }
                    0x90 => {
                        // SSG-EG
                        self.ssg_eg[slot] = self.data & 0x0f
                    }
                    _ => {}
                }
            }

            // Channel
            if CH_OFFSET[channel] == (self.address & 0x103) as u32 {
                address = (self.address & 0xfc) as u32;
                match address {
                    0xa0 => {
                        self.fnum[channel] =
                            self.data as u16 | (((self.reg_a4 & 0x07) as u16) << 8);
                        self.block[channel] = (self.reg_a4 >> 3) & 0x7;
                        self.kcode[channel] = ((self.block[channel]) << 2)
                            | (FN_NOTE[(self.fnum[channel] >> 7) as usize]) as u8
                    }
                    0xa4 => self.reg_a4 = self.data,
                    0xa8 => {
                        self.fnum_3ch[channel] =
                            self.data as u16 | ((self.reg_ac as u16 & 0x07) << 8);
                        self.block_3ch[channel] = (self.reg_ac >> 3) & 0x7;
                        self.kcode_3ch[channel] = ((self.block_3ch[channel] << 2) as u32
                            | FN_NOTE[(self.fnum_3ch[channel] >> 7) as usize])
                            as u8
                    }
                    0xac => self.reg_ac = self.data,
                    0xb0 => {
                        self.connect[channel] = self.data & 0x7;
                        self.fb[channel] = (self.data >> 3) & 0x7;
                    }
                    0xb4 => {
                        self.pms[channel] = self.data & 0x7;
                        self.ams[channel] = (self.data >> 4) & 0x3;
                        self.pan_l[channel] = (self.data >> 7) & 0x1;
                        self.pan_r[channel] = (self.data >> 6) & 0x1;
                    }
                    _ => {}
                }
            }
        }
        if self.write_a_en > 0 || self.write_d_en > 0 {
            // Data
            if self.write_a_en > 0 {
                self.write_fm_data = 0
            }
            if self.write_fm_address > 0 && self.write_d_en > 0 {
                self.write_fm_data = 1
            }

            // Address
            if self.write_a_en > 0 {
                if self.write_data & 0xf0 > 0 {
                    // FM Write
                    self.address = self.write_data;
                    self.write_fm_address = 1
                } else {
                    // SSG write
                    self.write_fm_address = 0
                }
            }

            // FM Mode
            // Data
            if self.write_d_en > 0 && self.write_data & 0x100 == 0 {
                match self.write_fm_mode_a {
                    0x21 => {
                        // LSI test 1
                        for i in 0usize..8 {
                            self.mode_test_21[i] = (self.write_data >> i & 0x1) as u8;
                        }
                    }
                    0x22 => {
                        // LFO control
                        if (self.write_data >> 3) & 0x1 > 0 {
                            self.lfo_en = 0x7f
                        } else {
                            self.lfo_en = 0
                        }
                        self.lfo_freq = (self.write_data & 0x7) as u8;
                    }
                    0x24 => {
                        // Timer A
                        self.timer_a_reg &= 0x03;
                        self.timer_a_reg |= (self.write_data & 0xff) << 2;
                    }
                    0x25 => {
                        self.timer_a_reg &= 0x3fc;
                        self.timer_a_reg |= self.write_data & 0x03;
                    }
                    0x26 => {
                        // Timer B
                        self.timer_b_reg = self.write_data & 0xff;
                    }
                    0x27 => {
                        // CSM, Timer control
                        self.mode_ch3 = ((self.write_data & 0xc0) >> 6) as u8;
                        self.mode_csm = (self.mode_ch3 == 2) as u8;
                        self.timer_a_load = (self.write_data & 0x01) as u8;
                        self.timer_a_enable = ((self.write_data >> 2) & 0x01) as u8;
                        self.timer_a_reset = ((self.write_data >> 4) & 0x01) as u8;
                        self.timer_b_load = ((self.write_data >> 1) & 0x01) as u8;
                        self.timer_b_enable = ((self.write_data >> 3) & 0x01) as u8;
                        self.timer_b_reset = ((self.write_data >> 5) & 0x01) as u8;
                    }
                    0x28 => {
                        // Key on/off
                        for i in 0usize..4 {
                            self.mode_kon_operator[i] =
                                ((self.write_data >> (4usize.wrapping_add(i))) & 0x1) as u8;
                        }
                        if self.write_data & 0x03 == 0x03 {
                            // Invalid address
                            self.mode_kon_channel = 0xff
                        } else {
                            self.mode_kon_channel = ((self.write_data & 0x03)
                                .wrapping_add(((self.write_data >> 2) & 1) * 3))
                                as u8;
                        }
                    }
                    0x2a => {
                        // DAC data
                        self.dacdata &= 0x1;
                        self.dacdata |= ((self.write_data ^ 0x80) << 1) as i16
                    }
                    0x2b => {
                        // DAC enable
                        self.dacen = (self.write_data >> 7) as u8
                    }
                    0x2c => {
                        // LSI test 2
                        for i in 0usize..8 {
                            self.mode_test_2c[i] = ((self.write_data >> i) & 0x1) as u8;
                        }
                        self.dacdata &= 0x1fe;
                        self.dacdata |= self.mode_test_2c[3] as i16;
                        self.eg_custom_timer =
                            (self.mode_test_2c[7] == 0 && self.mode_test_2c[6] > 0) as u8;
                    }
                    _ => {}
                }
            }
            // Address
            if self.write_a_en > 0 {
                self.write_fm_mode_a = (self.write_data & 0x1ff) as u16
            }
        }
        if self.write_fm_data > 0 {
            self.data = (self.write_data & 0xff) as u8;
        };
    }

    pub fn phase_calc_increment(&mut self) {
        let chan: u32 = self.channel;
        let slot = self.cycles as usize;
        let mut fnum: u32 = self.pg_fnum as u32;
        let fnum_h: u32 = fnum >> 4;
        let mut fm: u32;
        let mut basefreq: u32;
        let lfo: u8 = self.lfo_pm;
        let mut lfo_l: u8 = lfo & 0x0f;
        let pms: u8 = self.pms[chan as usize];
        let dt: u8 = self.dt[slot];
        let dt_l: u8 = dt & 0x03;
        let mut detune: u8 = 0;
        let block: u8;
        let note: u8;
        let sum: u8;
        let sum_h: u8;
        let sum_l: u8;
        let mut kcode: u8 = self.pg_kcode;

        fnum <<= 1;

        // Apply LFO
        if lfo_l & 0x08 > 0 {
            lfo_l ^= 0x0f
        }
        fm = (fnum_h >> PG_LFO_SH1[pms as usize][lfo_l as usize])
            .wrapping_add(fnum_h >> PG_LFO_SH2[pms as usize][lfo_l as usize]);
        if pms > 5 {
            fm <<= pms - 5
        }
        fm >>= 2;
        if lfo & 0x10 > 0 {
            fnum = (fnum).wrapping_sub(fm);
        } else {
            fnum = (fnum).wrapping_add(fm);
        }
        fnum &= 0xfff;

        basefreq = (fnum << self.pg_block) >> 2;

        // Apply detune
        if dt_l > 0 {
            if kcode > 0x1c {
                kcode = 0x1c;
            }
            block = kcode >> 2;
            note = kcode & 0x03;
            sum = block
                .wrapping_add(9)
                .wrapping_add((dt_l == 3) as u8 | (dt_l & 0x02));
            sum_h = sum >> 1;
            sum_l = sum & 0x01;
            detune = (PG_DETUNE[((sum_l << 2) | note) as usize] >> (9 - sum_h)) as u8
        }
        if dt & 0x04 > 0 {
            basefreq = basefreq.wrapping_sub(detune as u32)
        } else {
            basefreq = basefreq.wrapping_add(detune as u32)
        }
        basefreq &= 0x1ffff;
        self.pg_inc[slot] = (basefreq * self.multi[slot] as u32) >> 1;
        self.pg_inc[slot] &= 0xfffff;
    }

    pub fn phase_generate(&mut self) {
        // Mask increment
        let mut slot = (self.cycles.wrapping_add(20) % 24) as usize;
        if self.pg_reset[slot] > 0 {
            self.pg_inc[slot] = 0
        }

        // Phase step
        slot = self.cycles.wrapping_add(19).wrapping_rem(24) as usize;
        if self.pg_reset[slot] > 0 || self.mode_test_21[3] > 0 {
            self.pg_phase[slot] = 0
        }
        self.pg_phase[slot] = self.pg_phase[slot].wrapping_add(self.pg_inc[slot]);
        self.pg_phase[slot] &= 0xfffff;
    }

    pub fn envelope_ssg_eg(&mut self) {
        let slot = self.cycles as usize;
        let mut direction: u8 = 0;
        self.eg_ssg_pgrst_latch[slot] = 0;
        self.eg_ssg_repeat_latch[slot] = 0;
        self.eg_ssg_hold_up_latch[slot] = 0;
        self.eg_ssg_inv[slot] = 0;
        if self.ssg_eg[slot] & 0x8 > 0 {
            direction = self.eg_ssg_dir[slot];
            if self.eg_level[slot] & 0x200 > 0 {
                // Reset
                if self.ssg_eg[slot] & 0x03 == 0x00 {
                    self.eg_ssg_pgrst_latch[slot] = 1;
                }

                // Repeat
                if self.ssg_eg[slot] & 0x01 == 0x00 {
                    self.eg_ssg_repeat_latch[slot] = 1;
                }

                // Inverse
                if self.ssg_eg[slot] & 0x03 == 0x02 {
                    direction ^= 1;
                }
                if self.ssg_eg[slot] & 0x03 == 0x03 {
                    direction = 1;
                }
            }

            // Hold up
            if self.eg_kon_latch[slot] > 0
                && (self.ssg_eg[slot] & 0x07 == 0x05 || self.ssg_eg[slot] & 0x07 == 0x03)
            {
                self.eg_ssg_hold_up_latch[slot] = 1
            }
            direction &= self.eg_kon[slot];
            self.eg_ssg_inv[slot] =
                (self.eg_ssg_dir[slot] ^ ((self.ssg_eg[slot] >> 2) & 0x01)) & self.eg_kon[slot]
        }
        self.eg_ssg_dir[slot] = direction;
        self.eg_ssg_enable[slot] = (self.ssg_eg[slot] >> 3) & 0x01;
    }

    pub fn envelope_adsr(&mut self) {
        let slot = self.cycles.wrapping_add(22).wrapping_rem(24) as usize;
        let nkon: u8 = self.eg_kon_latch[slot];
        let okon: u8 = self.eg_kon[slot];
        let kon_event: u8;
        let koff_event: u8;
        let eg_off: u8;
        let mut level: i16;
        let mut nextlevel: i16;
        let mut ssg_level: i16;
        let mut nextstate: u8 = self.eg_state[slot];
        let mut inc: i16 = 0;
        self.eg_read[0] = self.eg_read_inc as u32;
        self.eg_read_inc = (self.eg_inc > 0) as u8;

        // Reset phase generator
        self.pg_reset[slot] = (nkon > 0 && okon == 0 || self.eg_ssg_pgrst_latch[slot] > 0) as u8;

        // KeyOn/Off
        kon_event = (nkon > 0 && okon == 0 || okon > 0 && self.eg_ssg_repeat_latch[slot] > 0) as u8;
        koff_event = (okon > 0 && nkon == 0) as u8;
        level = self.eg_level[slot] as i16;

        ssg_level = level;

        if self.eg_ssg_inv[slot] > 0 {
            // Inverse
            ssg_level = 512i16.wrapping_sub(level);
            ssg_level &= 0x3ff
        }
        if koff_event > 0 {
            level = ssg_level;
        }
        if self.eg_ssg_enable[slot] > 0 {
            eg_off = (level >> 9) as u8;
        } else {
            eg_off = (level & 0x3f0 == 0x3f0) as u8;
        }
        nextlevel = level;
        if kon_event > 0 {
            nextstate = EG_NUM_ATTACK;
            // Instant attack
            if self.eg_ratemax > 0 {
                nextlevel = 0;
            } else if self.eg_state[slot] == EG_NUM_ATTACK
                && level > 0
                && self.eg_inc > 0
                && nkon > 0
            {
                inc = (!level << self.eg_inc) >> 5;
            }
        } else {
            match self.eg_state[slot] {
                EG_NUM_ATTACK => {
                    if level == 0 {
                        nextstate = EG_NUM_DECAY;
                    } else if self.eg_inc > 0 && self.eg_ratemax == 0 && nkon > 0 {
                        inc = (!level << self.eg_inc) >> 5;
                    }
                }
                EG_NUM_DECAY => {
                    if level >> 5 == self.eg_sl[1] as i16 {
                        nextstate = EG_NUM_SUSTAIN
                    } else if eg_off == 0 && self.eg_inc > 0 {
                        inc = 1 << (self.eg_inc - 1);
                        if self.eg_ssg_enable[slot] > 0 {
                            inc <<= 2
                        }
                    }
                }
                EG_NUM_SUSTAIN | EG_NUM_RELEASE => {
                    if eg_off == 0 && self.eg_inc > 0 {
                        inc = 1 << (self.eg_inc - 1);
                        if self.eg_ssg_enable[slot] > 0 {
                            inc <<= 2
                        }
                    }
                }
                _ => {}
            }
            if nkon == 0 {
                nextstate = EG_NUM_RELEASE
            }
        }
        if self.eg_kon_csm[slot] > 0 {
            nextlevel |= (self.eg_tl[1] << 3) as i16
        }

        // Envelope off
        if kon_event == 0
            && self.eg_ssg_hold_up_latch[slot] == 0
            && self.eg_state[slot] != EG_NUM_ATTACK
            && eg_off > 0
        {
            nextstate = EG_NUM_RELEASE;
            nextlevel = 0x3ff
        }

        nextlevel = nextlevel.wrapping_add(inc);

        self.eg_kon[slot] = self.eg_kon_latch[slot];
        self.eg_level[slot] = (nextlevel & 0x3ff) as u16;
        self.eg_state[slot] = nextstate;
    }

    pub fn envelope_prepare(&mut self) {
        let mut rate: u8;
        let sum: u8;
        let mut inc: u8 = 0;
        let slot = self.cycles as usize;
        let mut rate_sel: u8;

        // Prepare increment
        rate = (self.eg_rate.checked_shl(1).unwrap_or(u8::MAX)).wrapping_add(self.eg_ksv);

        if rate > 0x3f {
            rate = 0x3f
        }

        sum = ((rate >> 2).wrapping_add(self.eg_shift_lock)) & 0xf;
        if self.eg_rate > 0 && self.eg_quotient == 2 {
            if (rate) < 48 {
                match sum {
                    12 => inc = 1,
                    13 => inc = (rate >> 1) & 0x1,
                    14 => inc = rate & 0x1,
                    _ => {}
                }
            } else {
                inc = EG_STEP_HI[(rate & 0x3) as usize][self.eg_timer_low_lock as usize]
                    .wrapping_add((rate >> 2) as u32)
                    .wrapping_sub(11) as u8;
                if inc > 4 {
                    inc = 4
                }
            }
        }
        self.eg_inc = inc;
        self.eg_ratemax = (rate >> 1 == 0x1f) as u8;

        // Prepare rate & ksv
        rate_sel = self.eg_state[slot];
        if self.eg_kon[slot] > 0 && self.eg_ssg_repeat_latch[slot] > 0
            || self.eg_kon[slot] == 0 && self.eg_kon_latch[slot] > 0
        {
            rate_sel = EG_NUM_ATTACK
        }
        match rate_sel {
            0 => self.eg_rate = self.ar[slot],
            1 => self.eg_rate = self.dr[slot],
            2 => self.eg_rate = self.sr[slot],
            3 => self.eg_rate = (self.rr[slot] << 1) | 0x01,
            _ => {}
        }
        self.eg_ksv = self.pg_kcode >> (self.ks[slot] ^ 0x03);
        if self.am[slot] > 0 {
            self.eg_lfo_am = self.lfo_am >> EG_AM_SHIFT[self.ams[self.channel as usize] as usize]
        } else {
            self.eg_lfo_am = 0
        }

        // Delay TL & SL value
        self.eg_tl[1] = self.eg_tl[0];
        self.eg_tl[0] = self.tl[slot];
        self.eg_sl[1] = self.eg_sl[0];
        self.eg_sl[0] = self.sl[slot];
    }

    pub fn envelope_generate(&mut self) {
        let slot = ((self.cycles.wrapping_add(23)) % 24) as usize;
        let mut level: u16;

        level = self.eg_level[slot];

        if self.eg_ssg_inv[slot] > 0 {
            // Inverse
            level = 512u16.wrapping_sub(level);
        }
        if self.mode_test_21[5] > 0 {
            level = 0;
        }
        level &= 0x3ff;

        // Apply AM LFO
        level = level.wrapping_add(self.eg_lfo_am as u16);

        // Apply TL
        if !(self.mode_csm > 0 && self.channel == 2 + 1) {
            level = level.wrapping_add((self.eg_tl[0] << 3) as u16);
        }
        if level > 0x3ff {
            level = 0x3ff;
        }
        self.eg_out[slot] = level;
    }

    pub fn update_lfo(&mut self) {
        if self.lfo_quotient as u32 & LFO_CYCLES[self.lfo_freq as usize]
            == LFO_CYCLES[self.lfo_freq as usize]
        {
            self.lfo_quotient = 0;
            self.lfo_cnt = self.lfo_cnt.wrapping_add(1)
        } else {
            self.lfo_quotient = self.lfo_quotient.wrapping_add(self.lfo_inc)
        }
        self.lfo_cnt &= self.lfo_en;
    }

    pub fn fm_prepare(&mut self) {
        let mut slot = (self.cycles.wrapping_add(6) % 24) as usize;
        let channel = self.channel as usize;
        let mut mod_0: i16;
        let mut mod1: i16;
        let mut mod2: i16;
        let op = slot / 6;
        let connect: u8 = self.connect[channel];
        let prevslot = (self.cycles.wrapping_add(18) % 24) as usize;

        // Calculate modulation
        mod2 = 0;
        mod1 = mod2;

        if FM_ALGORITHM[op][0][connect as usize] > 0 {
            mod2 |= self.fm_op1[channel][0]
        }
        if FM_ALGORITHM[op][1][connect as usize] > 0 {
            mod1 |= self.fm_op1[channel][1]
        }
        if FM_ALGORITHM[op][2][connect as usize] > 0 {
            mod1 |= self.fm_op2[channel]
        }
        if FM_ALGORITHM[op][3][connect as usize] > 0 {
            mod2 |= self.fm_out[prevslot]
        }
        if FM_ALGORITHM[op][4][connect as usize] > 0 {
            mod1 |= self.fm_out[prevslot]
        }
        mod_0 = mod1.wrapping_add(mod2);
        if op == 0 {
            // Feedback
            mod_0 >>= 10 - self.fb[channel];
            if self.fb[channel] == 0 {
                mod_0 = 0
            }
        } else {
            mod_0 >>= 1
        }
        self.fm_mod[slot] = mod_0 as u16;

        slot = ((self.cycles.wrapping_add(18)) % 24) as usize;

        // OP1
        if slot.wrapping_div(6) == 0 {
            self.fm_op1[channel][1] = self.fm_op1[channel][0];
            self.fm_op1[channel][0] = self.fm_out[slot]
        }

        // OP2
        if slot.wrapping_div(6) == 2 {
            self.fm_op2[channel] = self.fm_out[slot]
        };
    }

    pub fn ch_generate(&mut self) {
        let slot = ((self.cycles.wrapping_add(18)) % 24) as usize;
        let channel = self.channel as usize;
        let op = slot / 6;
        let test_dac: u32 = self.mode_test_2c[5] as u32;
        let mut acc: i16 = self.ch_acc[channel];
        let mut add: i16 = test_dac as i16;
        let mut sum: i16;
        if op == 0 && test_dac == 0 {
            acc = 0
        }
        if FM_ALGORITHM[op][5][self.connect[channel] as usize] > 0 && test_dac == 0 {
            add = add.wrapping_add(self.fm_out[slot].checked_shr(5).unwrap_or_default())
        }
        sum = acc.wrapping_add(add);

        // Clamp
        if sum > 255 {
            sum = 255
        } else if sum < -256 {
            sum = -256
        }

        if op == 0 || test_dac > 0 {
            self.ch_out[channel] = self.ch_acc[channel]
        }
        self.ch_acc[channel] = sum;
    }

    pub fn ch_output(&mut self, ym2612: bool) {
        let cycles: u32 = self.cycles;
        let slot = self.cycles as usize;
        let mut channel = self.channel as usize;
        let test_dac: u32 = self.mode_test_2c[5] as u32;
        let mut out: i16;
        let mut sign: i16;
        let out_en: u32;
        self.ch_read = self.ch_lock;
        if slot < 12 {
            // Ch 4,5,6
            channel = channel.wrapping_add(1)
        }
        if cycles & 3 == 0 {
            if test_dac == 0 {
                // Lock value
                self.ch_lock = self.ch_out[channel]
            }
            self.ch_lock_l = self.pan_l[channel];
            self.ch_lock_r = self.pan_r[channel]
        }
        // Ch 6
        if (cycles >> 2) == 1 && self.dacen > 0 || test_dac > 0 {
            out = self.dacdata;
            out <<= 7;
            out >>= 7;
        } else {
            out = self.ch_lock
        }
        self.mol = 0;
        self.mor = 0;
        if ym2612 {
            out_en = (cycles & 3 == 3 || test_dac > 0) as u32;

            // YM2612 DAC emulation(not verified)
            sign = out >> 8;
            if out >= 0 {
                out = out.wrapping_add(1);
                sign = sign.wrapping_add(1);
            }
            if self.ch_lock_l > 0 && out_en > 0 {
                self.mol = out
            } else {
                self.mol = sign
            }
            if self.ch_lock_r > 0 && out_en > 0 {
                self.mor = out
            } else {
                self.mor = sign
            }

            // Amplify signal
            self.mol *= 3;
            self.mor *= 3
        } else {
            out_en = (cycles & 3 > 0 || test_dac > 0) as u32;
            if self.ch_lock_l > 0 && out_en > 0 {
                self.mol = out
            }
            if self.ch_lock_r > 0 && out_en > 0 {
                self.mor = out
            }
        };
    }

    pub fn fm_generate(&mut self) {
        let slot = (self.cycles.wrapping_add(19) % 24) as usize;

        // Calculate phase
        let phase: u16 =
            (((self.fm_mod[slot] as u32).wrapping_add(self.pg_phase[slot] >> 10)) & 0x3ff) as u16;
        let quarter: u16;
        let mut level: u16;
        let mut output: i16;
        if phase & 0x100 > 0 {
            quarter = ((phase ^ 0xff) & 0xff) as u16;
        } else {
            quarter = (phase & 0xff) as u16;
        }
        level = LOGSIN_ROM[quarter as usize];

        // Apply envelope
        level = level.wrapping_add(self.eg_out[slot].checked_shl(2).unwrap_or(u16::MAX));

        // Transform
        if level > 0x1fff {
            level = 0x1fff;
        }

        output = ((EXP_ROM[((level & 0xff) ^ 0xff) as usize] as i16 | 0x400) << 2)
            .checked_shr((level >> 8) as u32)
            .unwrap_or_default();

        if phase & 0x200 > 0 {
            output = ((!output) ^ (self.mode_test_21[4] as i16) << 13).wrapping_add(1);
        } else {
            output ^= (self.mode_test_21[4] as i16) << 13;
        }
        output <<= 2;
        output >>= 2;
        self.fm_out[slot] = output;
    }

    pub fn do_timer_a(&mut self) {
        let mut time: u16;
        let mut load: u8;
        load = self.timer_a_overflow;
        if self.cycles == 2 {
            // Lock load value
            load |= (self.timer_a_load_lock == 0 && self.timer_a_load > 0) as u8;
            self.timer_a_load_lock = self.timer_a_load;
            if self.mode_csm > 0 {
                // CSM KeyOn
                self.mode_kon_csm = load
            } else {
                self.mode_kon_csm = 0
            }
        }

        // Load counter
        if self.timer_a_load_latch > 0 {
            time = self.timer_a_reg
        } else {
            time = self.timer_a_cnt
        }
        self.timer_a_load_latch = load;

        // Increase counter
        if (self.cycles == 1 && self.timer_a_load_lock > 0) || self.mode_test_21[2] > 0 {
            time = time.wrapping_add(1);
        }

        // Set overflow flag
        if self.timer_a_reset > 0 {
            self.timer_a_reset = 0;
            self.timer_a_overflow_flag = 0
        } else {
            self.timer_a_overflow_flag |= self.timer_a_overflow & self.timer_a_enable
        }
        self.timer_a_overflow = (time >> 10) as u8;
        self.timer_a_cnt = (time & 0x3ff) as u16;
    }

    pub fn do_timer_b(&mut self) {
        let mut time: u16;
        let mut load: u8;
        load = self.timer_b_overflow;
        if self.cycles == 2 {
            // Lock load value
            load |= (self.timer_b_load_lock == 0 && self.timer_b_load > 0) as u8;
            self.timer_b_load_lock = self.timer_b_load
        }

        // Load counter
        if self.timer_b_load_latch > 0 {
            time = self.timer_b_reg
        } else {
            time = self.timer_b_cnt
        }
        self.timer_b_load_latch = load;

        // Increase counter
        if self.cycles == 1 {
            self.timer_b_subcnt = self.timer_b_subcnt.wrapping_add(1)
        }
        if (self.timer_b_subcnt == 0x10 && self.timer_b_load_lock > 0) || self.mode_test_21[2] > 0 {
            time = time.wrapping_add(1)
        }
        self.timer_b_subcnt &= 0x0f;

        // Set overflow flag
        if self.timer_b_reset > 0 {
            self.timer_b_reset = 0;
            self.timer_b_overflow_flag = 0
        } else {
            self.timer_b_overflow_flag |= self.timer_b_overflow & self.timer_b_enable
        }
        self.timer_b_overflow = (time >> 8) as u8;
        self.timer_b_cnt = (time & 0xff) as u16;
    }

    pub fn key_on(&mut self) {
        let slot = self.cycles as usize;
        let chan = self.channel as usize;
        // Key On
        self.eg_kon_latch[slot] = self.mode_kon[slot];
        self.eg_kon_csm[slot] = 0;
        if self.channel == 2 && self.mode_kon_csm > 0 {
            // CSM Key On
            self.eg_kon_latch[slot] = 1;
            self.eg_kon_csm[slot] = 1
        }
        if self.cycles == self.mode_kon_channel as u32 {
            // OP1
            self.mode_kon[chan] = self.mode_kon_operator[0];

            // OP2
            self.mode_kon[chan.wrapping_add(12)] = self.mode_kon_operator[1];

            // OP3
            self.mode_kon[chan.wrapping_add(6)] = self.mode_kon_operator[2];

            // OP4
            self.mode_kon[chan.wrapping_add(18)] = self.mode_kon_operator[3]
        };
    }
}
