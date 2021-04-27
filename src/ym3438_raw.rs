/*
 * Copyright (C) 2017-2018 Alexey Khokholov (Nuke.YKT)
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 2
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 *
 *
 *  Nuked OPN2(Yamaha YM3438) emulator.
 *  Thanks:
 *      Silicon Pr0n:
 *          Yamaha YM3438 decap and die shot(digshadow).
 *      OPLx decapsulated(Matthew Gambrell, Olli Niemitalo):
 *          OPL2 ROMs.
 *
 * version: 1.0.9
 */

/* Enables status read on any port (TeraDrive, MD1 VA7, MD2, etc) */
pub const ym3438_mode_readmode: u32 = 2;

/* Enables YM2612 emulation (MD1, MD2 VA2) */
pub const ym3438_mode_ym2612: u32 = 1;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ym3438Raw {
    pub cycles: u32,
    pub channel: u32,
    pub mol: i16,
    pub mor: i16,
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
    pub lfo_en: u8,
    pub lfo_freq: u8,
    pub lfo_pm: u8,
    pub lfo_am: u8,
    pub lfo_cnt: u8,
    pub lfo_inc: u8,
    pub lfo_quotient: u8,
    pub pg_fnum: u16,
    pub pg_block: u8,
    pub pg_kcode: u8,
    pub pg_inc: [u32; 24],
    pub pg_phase: [u32; 24],
    pub pg_reset: [u8; 24],
    pub pg_read: u32,
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
    pub fm_op1: [[i16; 2]; 6],
    pub fm_op2: [i16; 6],
    pub fm_out: [i16; 24],
    pub fm_mod: [u16; 24],
    pub ch_acc: [i16; 6],
    pub ch_out: [i16; 6],
    pub ch_lock: i16,
    pub ch_lock_l: u8,
    pub ch_lock_r: u8,
    pub ch_read: i16,
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

pub const eg_num_release: u32 = 3;
pub const eg_num_sustain: u32 = 2;
pub const eg_num_decay: u32 = 1;
pub const eg_num_attack: u32 = 0;

/*
 * Copyright (C) 2017-2018 Alexey Khokholov (Nuke.YKT)
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 2
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 *
 *
 *  Nuked OPN2(Yamaha YM3438) emulator.
 *  Thanks:
 *      Silicon Pr0n:
 *          Yamaha YM3438 decap and die shot(digshadow).
 *      OPLx decapsulated(Matthew Gambrell, Olli Niemitalo):
 *          OPL2 ROMs.
 *
 * version: 1.0.10
 */

/* logsin table */
const logsinrom: [u16; 256] = [
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

/* exp table */
const EXP_ROM: [u16; 256] = [
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

/* Note table */
const FN_NOTE: [u32; 16] = [0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 3, 3, 3, 3, 3];

/* Envelope generator */
const EG_STEP_HI: [[u32; 4]; 4] = [[0, 0, 0, 0], [1, 0, 0, 0], [1, 0, 1, 0], [1, 1, 1, 0]];
const EG_AM_SHIFT: [u8; 4] = [7, 3, 1, 0];

/* Phase generator */
const PG_DETUNE: [u32; 8] = [16, 17, 19, 20, 22, 24, 27, 29];
const PG_LFO_SH1: [[u32; 8]; 8] = [
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 7, 7, 1, 1],
    [7, 7, 7, 7, 1, 1, 1, 1],
    [7, 7, 7, 1, 1, 1, 1, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
    [7, 7, 1, 1, 0, 0, 0, 0],
];
const PG_LFO_SH2: [[u32; 8]; 8] = [
    [7, 7, 7, 7, 7, 7, 7, 7],
    [7, 7, 7, 7, 2, 2, 2, 2],
    [7, 7, 7, 2, 2, 2, 7, 7],
    [7, 7, 2, 2, 7, 7, 2, 2],
    [7, 7, 2, 7, 7, 7, 2, 7],
    [7, 7, 7, 2, 7, 7, 2, 1],
    [7, 7, 7, 2, 7, 7, 2, 1],
    [7, 7, 7, 2, 7, 7, 2, 1],
];

/* Address decoder */
const OP_OFFSET: [u32; 12] = [
    0, 0x1, 0x2, 0x100, 0x101, 0x102, 0x4, 0x5, 0x6, 0x104, 0x105, 0x106,
];
const CH_OFFSET: [u32; 6] = [0, 0x1, 0x2, 0x100, 0x101, 0x102];

/* LFO */
const LFO_CYCLES: [u32; 8] = [108, 77, 71, 67, 62, 44, 8, 5];

/* FM algorithm */
const FM_ALGORITHM: [[[u32; 8]; 6]; 4] = [
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

const chip_type: u32 = ym3438_mode_readmode;

impl Ym3438Raw {
    fn opn2_do_io(&mut self) {
        /* Write signal check */
        self.write_a_en = (self.write_a as i32 & 0x3 as i32 == 0x1 as i32) as i32 as u8;
        self.write_d_en = (self.write_d as i32 & 0x3 as i32 == 0x1 as i32) as i32 as u8;
        self.write_a = ((self.write_a as i32) << 1 as i32) as u8;
        self.write_d = ((self.write_d as i32) << 1 as i32) as u8;
        /* Busy counter */
        self.busy = self.write_busy;
        self.write_busy_cnt = (self.write_busy_cnt as i32 + self.write_busy as i32) as u8;
        self.write_busy = (self.write_busy as i32 != 0
            && self.write_busy_cnt as i32 >> 5 as i32 == 0
            || self.write_d_en as i32 != 0) as i32 as u8;
        self.write_busy_cnt = (self.write_busy_cnt as i32 & 0x1f as i32) as u8;
    }

    fn opn2_do_reg_write(&mut self) {
        let mut i: u32 = 0;
        let mut slot = self.cycles.wrapping_rem(12 as i32 as u32);
        let mut address: u32 = 0;
        let mut channel = self.channel;
        /* Update registers */
        if self.write_fm_data != 0 {
            /* Slot */
            if OP_OFFSET[slot as usize] == (self.address as i32 & 0x107 as i32) as u32 {
                if self.address as i32 & 0x8 as i32 != 0 {
                    /* OP2, OP4 */
                    slot = (slot as u32).wrapping_add(12 as i32 as u32) as u32 as u32
                }
                address = (self.address as i32 & 0xf0 as i32) as u32;
                match address {
                    48 => {
                        /* DT, MULTI */
                        self.multi[slot as usize] = (self.data as i32 & 0xf as i32) as u8;
                        if self.multi[slot as usize] == 0 {
                            self.multi[slot as usize] = 1 as i32 as u8
                        } else {
                            self.multi[slot as usize] =
                                ((self.multi[slot as usize] as i32) << 1 as i32) as u8
                        }
                        self.dt[slot as usize] = (self.data as i32 >> 4 as i32 & 0x7 as i32) as u8
                    }
                    64 => {
                        /* TL */
                        self.tl[slot as usize] = (self.data as i32 & 0x7f as i32) as u8
                    }
                    80 => {
                        /* KS, AR */
                        self.ar[slot as usize] = (self.data as i32 & 0x1f as i32) as u8;
                        self.ks[slot as usize] = (self.data as i32 >> 6 as i32 & 0x3 as i32) as u8
                    }
                    96 => {
                        /* AM, DR */
                        self.dr[slot as usize] = (self.data as i32 & 0x1f as i32) as u8;
                        self.am[slot as usize] = (self.data as i32 >> 7 as i32 & 0x1 as i32) as u8
                    }
                    112 => {
                        /* SR */
                        self.sr[slot as usize] = (self.data as i32 & 0x1f as i32) as u8
                    }
                    128 => {
                        /* SL, RR */
                        self.rr[slot as usize] = (self.data as i32 & 0xf as i32) as u8;
                        self.sl[slot as usize] = (self.data as i32 >> 4 as i32 & 0xf as i32) as u8;
                        self.sl[slot as usize] = (self.sl[slot as usize] as i32
                            | self.sl[slot as usize] as i32 + 1 as i32 & 0x10 as i32)
                            as u8
                    }
                    144 => {
                        /* SSG-EG */
                        self.ssg_eg[slot as usize] = (self.data as i32 & 0xf as i32) as u8
                    }
                    _ => {}
                }
            }
            /* Channel */
            if CH_OFFSET[channel as usize] == (self.address as i32 & 0x103 as i32) as u32 {
                address = (self.address as i32 & 0xfc as i32) as u32;
                match address {
                    160 => {
                        self.fnum[channel as usize] = (self.data as i32 & 0xff as i32
                            | (self.reg_a4 as i32 & 0x7 as i32) << 8 as i32)
                            as u16;
                        self.block[channel as usize] =
                            (self.reg_a4 as i32 >> 3 as i32 & 0x7 as i32) as u8;
                        self.kcode[channel as usize] = (((self.block[channel as usize] as i32)
                            << 2 as i32)
                            as u32
                            | FN_NOTE[(self.fnum[channel as usize] as i32 >> 7 as i32) as usize])
                            as u8
                    }
                    164 => self.reg_a4 = (self.data as i32 & 0xff as i32) as u8,
                    168 => {
                        self.fnum_3ch[channel as usize] = (self.data as i32 & 0xff as i32
                            | (self.reg_ac as i32 & 0x7 as i32) << 8 as i32)
                            as u16;
                        self.block_3ch[channel as usize] =
                            (self.reg_ac as i32 >> 3 as i32 & 0x7 as i32) as u8;
                        self.kcode_3ch[channel as usize] =
                            (((self.block_3ch[channel as usize] as i32) << 2 as i32) as u32
                                | FN_NOTE
                                    [(self.fnum_3ch[channel as usize] as i32 >> 7 as i32) as usize])
                                as u8
                    }
                    172 => self.reg_ac = (self.data as i32 & 0xff as i32) as u8,
                    176 => {
                        self.connect[channel as usize] = (self.data as i32 & 0x7 as i32) as u8;
                        self.fb[channel as usize] =
                            (self.data as i32 >> 3 as i32 & 0x7 as i32) as u8
                    }
                    180 => {
                        self.pms[channel as usize] = (self.data as i32 & 0x7 as i32) as u8;
                        self.ams[channel as usize] =
                            (self.data as i32 >> 4 as i32 & 0x3 as i32) as u8;
                        self.pan_l[channel as usize] =
                            (self.data as i32 >> 7 as i32 & 0x1 as i32) as u8;
                        self.pan_r[channel as usize] =
                            (self.data as i32 >> 6 as i32 & 0x1 as i32) as u8
                    }
                    _ => {}
                }
            }
        }
        if self.write_a_en as i32 != 0 || self.write_d_en as i32 != 0 {
            /* Data */
            if self.write_a_en != 0 {
                self.write_fm_data = 0 as i32 as u8
            }
            if self.write_fm_address as i32 != 0 && self.write_d_en as i32 != 0 {
                self.write_fm_data = 1 as i32 as u8
            }
            /* Address */
            if self.write_a_en != 0 {
                if self.write_data as i32 & 0xf0 as i32 != 0 as i32 {
                    /* FM Write */
                    self.address = self.write_data;
                    self.write_fm_address = 1 as i32 as u8
                } else {
                    /* SSG write */
                    self.write_fm_address = 0 as i32 as u8
                }
            }
            /* FM Mode */
            /* Data */
            if self.write_d_en as i32 != 0 && self.write_data as i32 & 0x100 as i32 == 0 as i32 {
                match self.write_fm_mode_a as i32 {
                    33 => {
                        /* LSI test 1 */
                        i = 0 as i32 as u32;
                        while i < 8 as i32 as u32 {
                            self.mode_test_21[i as usize] =
                                (self.write_data as i32 >> i & 0x1 as i32) as u8;
                            i = i.wrapping_add(1)
                        }
                    }
                    34 => {
                        /* LFO control */
                        if self.write_data as i32 >> 3 as i32 & 0x1 as i32 != 0 {
                            self.lfo_en = 0x7f as i32 as u8
                        } else {
                            self.lfo_en = 0 as i32 as u8
                        }
                        self.lfo_freq = (self.write_data as i32 & 0x7 as i32) as u8
                    }
                    36 => {
                        /* Timer A */
                        self.timer_a_reg = (self.timer_a_reg as i32 & 0x3 as i32) as u16;
                        self.timer_a_reg = (self.timer_a_reg as i32
                            | (self.write_data as i32 & 0xff as i32) << 2 as i32)
                            as u16
                    }
                    37 => {
                        self.timer_a_reg = (self.timer_a_reg as i32 & 0x3fc as i32) as u16;
                        self.timer_a_reg =
                            (self.timer_a_reg as i32 | self.write_data as i32 & 0x3 as i32) as u16
                    }
                    38 => {
                        /* Timer B */
                        self.timer_b_reg = (self.write_data as i32 & 0xff as i32) as u16
                    }
                    39 => {
                        /* CSM, Timer control */
                        self.mode_ch3 = ((self.write_data as i32 & 0xc0 as i32) >> 6 as i32) as u8;
                        self.mode_csm = (self.mode_ch3 as i32 == 2 as i32) as i32 as u8;
                        self.timer_a_load = (self.write_data as i32 & 0x1 as i32) as u8;
                        self.timer_a_enable =
                            (self.write_data as i32 >> 2 as i32 & 0x1 as i32) as u8;
                        self.timer_a_reset =
                            (self.write_data as i32 >> 4 as i32 & 0x1 as i32) as u8;
                        self.timer_b_load = (self.write_data as i32 >> 1 as i32 & 0x1 as i32) as u8;
                        self.timer_b_enable =
                            (self.write_data as i32 >> 3 as i32 & 0x1 as i32) as u8;
                        self.timer_b_reset = (self.write_data as i32 >> 5 as i32 & 0x1 as i32) as u8
                    }
                    40 => {
                        /* Key on/off */
                        i = 0 as i32 as u32;
                        while i < 4 as i32 as u32 {
                            self.mode_kon_operator[i as usize] =
                                (self.write_data as i32 >> (4 as i32 as u32).wrapping_add(i)
                                    & 0x1 as i32) as u8;
                            i = i.wrapping_add(1)
                        }
                        if self.write_data as i32 & 0x3 as i32 == 0x3 as i32 {
                            /* Invalid address */
                            self.mode_kon_channel = 0xff as i32 as u8
                        } else {
                            self.mode_kon_channel = ((self.write_data as i32 & 0x3 as i32)
                                + (self.write_data as i32 >> 2 as i32 & 1 as i32) * 3 as i32)
                                as u8
                        }
                    }
                    42 => {
                        /* DAC data */
                        self.dacdata = (self.dacdata as i32 & 0x1 as i32) as i16;
                        self.dacdata = (self.dacdata as i32
                            | (self.write_data as i32 ^ 0x80 as i32) << 1 as i32)
                            as i16
                    }
                    43 => {
                        /* DAC enable */
                        self.dacen = (self.write_data as i32 >> 7 as i32) as u8
                    }
                    44 => {
                        /* LSI test 2 */
                        i = 0 as i32 as u32;
                        while i < 8 as i32 as u32 {
                            self.mode_test_2c[i as usize] =
                                (self.write_data as i32 >> i & 0x1 as i32) as u8;
                            i = i.wrapping_add(1)
                        }
                        self.dacdata = (self.dacdata as i32 & 0x1fe as i32) as i16;
                        self.dacdata = (self.dacdata as i32
                            | self.mode_test_2c[3 as i32 as usize] as i32)
                            as i16;
                        self.eg_custom_timer = (self.mode_test_2c[7 as i32 as usize] == 0
                            && self.mode_test_2c[6 as i32 as usize] as i32 != 0)
                            as i32 as u8
                    }
                    _ => {}
                }
            }
            /* Address */
            if self.write_a_en != 0 {
                self.write_fm_mode_a = (self.write_data as i32 & 0x1ff as i32) as u16
            }
        }
        if self.write_fm_data != 0 {
            self.data = (self.write_data as i32 & 0xff as i32) as u8
        };
    }

    fn opn2_phase_calc_increment(&mut self) {
        let mut chan = self.channel;
        let mut slot = self.cycles;
        let mut fnum = self.pg_fnum as u32;
        let mut fnum_h = fnum >> 4 as i32;
        let mut fm: u32 = 0;
        let mut basefreq: u32 = 0;
        let mut lfo = self.lfo_pm;
        let mut lfo_l = (lfo as i32 & 0xf as i32) as u8;
        let mut pms = self.pms[chan as usize];
        let mut dt = self.dt[slot as usize];
        let mut dt_l = (dt as i32 & 0x3 as i32) as u8;
        let mut detune = 0 as i32 as u8;
        let mut block: u8 = 0;
        let mut note: u8 = 0;
        let mut sum: u8 = 0;
        let mut sum_h: u8 = 0;
        let mut sum_l: u8 = 0;
        let mut kcode = self.pg_kcode;
        fnum <<= 1 as i32;
        /* Apply LFO */
        if lfo_l as i32 & 0x8 as i32 != 0 {
            lfo_l = (lfo_l as i32 ^ 0xf as i32) as u8
        }
        fm = (fnum_h >> PG_LFO_SH1[pms as usize][lfo_l as usize])
            .wrapping_add(fnum_h >> PG_LFO_SH2[pms as usize][lfo_l as usize]);
        if pms as i32 > 5 as i32 {
            fm <<= pms as i32 - 5 as i32
        }
        fm >>= 2 as i32;
        if lfo as i32 & 0x10 as i32 != 0 {
            fnum = (fnum as u32).wrapping_sub(fm) as u32 as u32
        } else {
            fnum = (fnum as u32).wrapping_add(fm) as u32 as u32
        }
        fnum &= 0xfff as i32 as u32;
        basefreq = fnum << self.pg_block as i32 >> 2 as i32;
        /* Apply detune */
        if dt_l != 0 {
            if kcode as i32 > 0x1c as i32 {
                kcode = 0x1c as i32 as u8
            }
            block = (kcode as i32 >> 2 as i32) as u8;
            note = (kcode as i32 & 0x3 as i32) as u8;
            sum = (block as i32
                + 9 as i32
                + ((dt_l as i32 == 3 as i32) as i32 | dt_l as i32 & 0x2 as i32))
                as u8;
            sum_h = (sum as i32 >> 1 as i32) as u8;
            sum_l = (sum as i32 & 0x1 as i32) as u8;
            detune = (PG_DETUNE[((sum_l as i32) << 2 as i32 | note as i32) as usize]
                >> 9 as i32 - sum_h as i32) as u8
        }
        if dt as i32 & 0x4 as i32 != 0 {
            basefreq = (basefreq as u32).wrapping_sub(detune as u32) as u32 as u32
        } else {
            basefreq = (basefreq as u32).wrapping_add(detune as u32) as u32 as u32
        }
        basefreq &= 0x1ffff as i32 as u32;
        self.pg_inc[slot as usize] =
            basefreq.wrapping_mul(self.multi[slot as usize] as u32) >> 1 as i32;
        self.pg_inc[slot as usize] &= 0xfffff as i32 as u32;
    }

    fn opn2_phase_generate(&mut self) {
        let mut slot: u32 = 0;
        /* Mask increment */
        slot = self
            .cycles
            .wrapping_add(20 as i32 as u32)
            .wrapping_rem(24 as i32 as u32);
        if self.pg_reset[slot as usize] != 0 {
            self.pg_inc[slot as usize] = 0 as i32 as u32
        }
        /* Phase step */
        slot = self
            .cycles
            .wrapping_add(19 as i32 as u32)
            .wrapping_rem(24 as i32 as u32);
        if self.pg_reset[slot as usize] as i32 != 0
            || self.mode_test_21[3 as i32 as usize] as i32 != 0
        {
            self.pg_phase[slot as usize] = 0 as i32 as u32
        }
        self.pg_phase[slot as usize] = (self.pg_phase[slot as usize] as u32)
            .wrapping_add(self.pg_inc[slot as usize]) as u32
            as u32;
        self.pg_phase[slot as usize] &= 0xfffff as i32 as u32;
    }

    fn opn2_envelope_ssg_eg(&mut self) {
        let mut slot = self.cycles;
        let mut direction = 0 as i32 as u8;
        self.eg_ssg_pgrst_latch[slot as usize] = 0 as i32 as u8;
        self.eg_ssg_repeat_latch[slot as usize] = 0 as i32 as u8;
        self.eg_ssg_hold_up_latch[slot as usize] = 0 as i32 as u8;
        self.eg_ssg_inv[slot as usize] = 0 as i32 as u8;
        if self.ssg_eg[slot as usize] as i32 & 0x8 as i32 != 0 {
            direction = self.eg_ssg_dir[slot as usize];
            if self.eg_level[slot as usize] as i32 & 0x200 as i32 != 0 {
                /* Reset */
                if self.ssg_eg[slot as usize] as i32 & 0x3 as i32 == 0 as i32 {
                    self.eg_ssg_pgrst_latch[slot as usize] = 1 as i32 as u8
                }
                /* Repeat */
                if self.ssg_eg[slot as usize] as i32 & 0x1 as i32 == 0 as i32 {
                    self.eg_ssg_repeat_latch[slot as usize] = 1 as i32 as u8
                }
                /* Inverse */
                if self.ssg_eg[slot as usize] as i32 & 0x3 as i32 == 0x2 as i32 {
                    direction = (direction as i32 ^ 1 as i32) as u8
                }
                if self.ssg_eg[slot as usize] as i32 & 0x3 as i32 == 0x3 as i32 {
                    direction = 1 as i32 as u8
                }
            }
            /* Hold up */
            if self.eg_kon_latch[slot as usize] as i32 != 0
                && (self.ssg_eg[slot as usize] as i32 & 0x7 as i32 == 0x5 as i32
                    || self.ssg_eg[slot as usize] as i32 & 0x7 as i32 == 0x3 as i32)
            {
                self.eg_ssg_hold_up_latch[slot as usize] = 1 as i32 as u8
            }
            direction = (direction as i32 & self.eg_kon[slot as usize] as i32) as u8;
            self.eg_ssg_inv[slot as usize] = ((self.eg_ssg_dir[slot as usize] as i32
                ^ self.ssg_eg[slot as usize] as i32 >> 2 as i32 & 0x1 as i32)
                & self.eg_kon[slot as usize] as i32)
                as u8
        }
        self.eg_ssg_dir[slot as usize] = direction;
        self.eg_ssg_enable[slot as usize] =
            (self.ssg_eg[slot as usize] as i32 >> 3 as i32 & 0x1 as i32) as u8;
    }

    fn opn2_envelope_adsr(&mut self) {
        let mut slot = self
            .cycles
            .wrapping_add(22 as i32 as u32)
            .wrapping_rem(24 as i32 as u32);
        let mut nkon = self.eg_kon_latch[slot as usize];
        let mut okon = self.eg_kon[slot as usize];
        let mut kon_event: u8 = 0;
        let mut koff_event: u8 = 0;
        let mut eg_off: u8 = 0;
        let mut level: i16 = 0;
        let mut nextlevel = 0 as i32 as i16;
        let mut ssg_level: i16 = 0;
        let mut nextstate = self.eg_state[slot as usize];
        let mut inc = 0 as i32 as i16;
        self.eg_read[0 as i32 as usize] = self.eg_read_inc as u32;
        self.eg_read_inc = (self.eg_inc as i32 > 0 as i32) as i32 as u8;
        /* Reset phase generator */
        self.pg_reset[slot as usize] = (nkon as i32 != 0 && okon == 0
            || self.eg_ssg_pgrst_latch[slot as usize] as i32 != 0)
            as i32 as u8;
        /* KeyOn/Off */
        kon_event = (nkon as i32 != 0 && okon == 0
            || okon as i32 != 0 && self.eg_ssg_repeat_latch[slot as usize] as i32 != 0)
            as i32 as u8;
        koff_event = (okon as i32 != 0 && nkon == 0) as i32 as u8;
        level = self.eg_level[slot as usize] as i16;
        ssg_level = level;
        if self.eg_ssg_inv[slot as usize] != 0 {
            /* Inverse */
            ssg_level = (512 as i32 - level as i32) as i16;
            ssg_level = (ssg_level as i32 & 0x3ff as i32) as i16
        }
        if koff_event != 0 {
            level = ssg_level
        }
        if self.eg_ssg_enable[slot as usize] != 0 {
            eg_off = (level as i32 >> 9 as i32) as u8
        } else {
            eg_off = (level as i32 & 0x3f0 as i32 == 0x3f0 as i32) as i32 as u8
        }
        nextlevel = level;
        if kon_event != 0 {
            nextstate = eg_num_attack as i32 as u8;
            /* Instant attack */
            if self.eg_ratemax != 0 {
                nextlevel = 0 as i32 as i16
            } else if self.eg_state[slot as usize] as i32 == eg_num_attack as i32
                && level as i32 != 0 as i32
                && self.eg_inc as i32 != 0
                && nkon as i32 != 0
            {
                inc = ((!(level as i32)) << self.eg_inc as i32 >> 5 as i32) as i16
            }
        } else {
            match self.eg_state[slot as usize] as i32 {
                0 => {
                    if level as i32 == 0 as i32 {
                        nextstate = eg_num_decay as i32 as u8
                    } else if self.eg_inc as i32 != 0 && self.eg_ratemax == 0 && nkon as i32 != 0 {
                        inc = ((!(level as i32)) << self.eg_inc as i32 >> 5 as i32) as i16
                    }
                }
                1 => {
                    if level as i32 >> 5 as i32 == self.eg_sl[1 as i32 as usize] as i32 {
                        nextstate = eg_num_sustain as i32 as u8
                    } else if eg_off == 0 && self.eg_inc as i32 != 0 {
                        inc = ((1 as i32) << self.eg_inc as i32 - 1 as i32) as i16;
                        if self.eg_ssg_enable[slot as usize] != 0 {
                            inc = ((inc as i32) << 2 as i32) as i16
                        }
                    }
                }
                2 | 3 => {
                    if eg_off == 0 && self.eg_inc as i32 != 0 {
                        inc = ((1 as i32) << self.eg_inc as i32 - 1 as i32) as i16;
                        if self.eg_ssg_enable[slot as usize] != 0 {
                            inc = ((inc as i32) << 2 as i32) as i16
                        }
                    }
                }
                _ => {}
            }
            if nkon == 0 {
                nextstate = eg_num_release as i32 as u8
            }
        }
        if self.eg_kon_csm[slot as usize] != 0 {
            nextlevel =
                (nextlevel as i32 | (self.eg_tl[1 as i32 as usize] as i32) << 3 as i32) as i16
        }
        /* Envelope off */
        if kon_event == 0
            && self.eg_ssg_hold_up_latch[slot as usize] == 0
            && self.eg_state[slot as usize] as i32 != eg_num_attack as i32
            && eg_off as i32 != 0
        {
            nextstate = eg_num_release as i32 as u8;
            nextlevel = 0x3ff as i32 as i16
        }
        nextlevel = (nextlevel as i32 + inc as i32) as i16;
        self.eg_kon[slot as usize] = self.eg_kon_latch[slot as usize];
        self.eg_level[slot as usize] = (nextlevel as u16 as i32 & 0x3ff as i32) as u16;
        self.eg_state[slot as usize] = nextstate;
    }

    fn opn2_envelope_prepare(&mut self) {
        let mut rate: u8 = 0;
        let mut sum: u8 = 0;
        let mut inc = 0 as i32 as u8;
        let mut slot = self.cycles;
        let mut rate_sel: u8 = 0;
        /* Prepare increment */
        rate = (((self.eg_rate as i32) << 1 as i32) + self.eg_ksv as i32) as u8;
        if rate as i32 > 0x3f as i32 {
            rate = 0x3f as i32 as u8
        }
        sum = ((rate as i32 >> 2 as i32) + self.eg_shift_lock as i32 & 0xf as i32) as u8;
        if self.eg_rate as i32 != 0 as i32 && self.eg_quotient as i32 == 2 as i32 {
            if (rate as i32) < 48 as i32 {
                match sum as i32 {
                    12 => inc = 1 as i32 as u8,
                    13 => inc = (rate as i32 >> 1 as i32 & 0x1 as i32) as u8,
                    14 => inc = (rate as i32 & 0x1 as i32) as u8,
                    _ => {}
                }
            } else {
                inc = EG_STEP_HI[(rate as i32 & 0x3 as i32) as usize]
                    [self.eg_timer_low_lock as usize]
                    .wrapping_add((rate as i32 >> 2 as i32) as u32)
                    .wrapping_sub(11 as i32 as u32) as u8;
                if inc as i32 > 4 as i32 {
                    inc = 4 as i32 as u8
                }
            }
        }
        self.eg_inc = inc;
        self.eg_ratemax = (rate as i32 >> 1 as i32 == 0x1f as i32) as i32 as u8;
        /* Prepare rate & ksv */
        rate_sel = self.eg_state[slot as usize];
        if self.eg_kon[slot as usize] as i32 != 0
            && self.eg_ssg_repeat_latch[slot as usize] as i32 != 0
            || self.eg_kon[slot as usize] == 0 && self.eg_kon_latch[slot as usize] as i32 != 0
        {
            rate_sel = eg_num_attack as i32 as u8
        }
        match rate_sel as i32 {
            0 => self.eg_rate = self.ar[slot as usize],
            1 => self.eg_rate = self.dr[slot as usize],
            2 => self.eg_rate = self.sr[slot as usize],
            3 => self.eg_rate = ((self.rr[slot as usize] as i32) << 1 as i32 | 0x1 as i32) as u8,
            _ => {}
        }
        self.eg_ksv = (self.pg_kcode as i32 >> (self.ks[slot as usize] as i32 ^ 0x3 as i32)) as u8;
        if self.am[slot as usize] != 0 {
            self.eg_lfo_am = (self.lfo_am as i32
                >> EG_AM_SHIFT[self.ams[self.channel as usize] as usize] as i32)
                as u8
        } else {
            self.eg_lfo_am = 0 as i32 as u8
        }
        /* Delay TL & SL value */
        self.eg_tl[1 as i32 as usize] = self.eg_tl[0 as i32 as usize];
        self.eg_tl[0 as i32 as usize] = self.tl[slot as usize];
        self.eg_sl[1 as i32 as usize] = self.eg_sl[0 as i32 as usize];
        self.eg_sl[0 as i32 as usize] = self.sl[slot as usize];
    }

    fn opn2_envelope_generate(&mut self) {
        let mut slot = self
            .cycles
            .wrapping_add(23 as i32 as u32)
            .wrapping_rem(24 as i32 as u32);
        let mut level: u16 = 0;
        level = self.eg_level[slot as usize];
        if self.eg_ssg_inv[slot as usize] != 0 {
            /* Inverse */
            level = (512 as i32 - level as i32) as u16
        }
        if self.mode_test_21[5 as i32 as usize] != 0 {
            level = 0 as i32 as u16
        }
        level = (level as i32 & 0x3ff as i32) as u16;
        /* Apply AM LFO */
        level = (level as i32 + self.eg_lfo_am as i32) as u16;
        /* Apply TL */
        if !(self.mode_csm as i32 != 0 && self.channel == (2 as i32 + 1 as i32) as u32) {
            level = (level as i32 + ((self.eg_tl[0 as i32 as usize] as i32) << 3 as i32)) as u16
        }
        if level as i32 > 0x3ff as i32 {
            level = 0x3ff as i32 as u16
        }
        self.eg_out[slot as usize] = level;
    }

    fn opn2_update_lfo(&mut self) {
        if self.lfo_quotient as u32 & LFO_CYCLES[self.lfo_freq as usize]
            == LFO_CYCLES[self.lfo_freq as usize]
        {
            self.lfo_quotient = 0 as i32 as u8;
            self.lfo_cnt = self.lfo_cnt.wrapping_add(1)
        } else {
            self.lfo_quotient = (self.lfo_quotient as i32 + self.lfo_inc as i32) as u8
        }
        self.lfo_cnt = (self.lfo_cnt as i32 & self.lfo_en as i32) as u8;
    }

    fn opn2_fm_prepare(&mut self) {
        let mut slot = self
            .cycles
            .wrapping_add(6 as i32 as u32)
            .wrapping_rem(24 as i32 as u32);
        let mut channel = self.channel;
        let mut mod_0: i16 = 0;
        let mut mod1: i16 = 0;
        let mut mod2: i16 = 0;
        let mut op = slot.wrapping_div(6 as i32 as u32);
        let mut connect = self.connect[channel as usize];
        let mut prevslot = self
            .cycles
            .wrapping_add(18 as i32 as u32)
            .wrapping_rem(24 as i32 as u32);
        /* Calculate modulation */
        mod2 = 0 as i32 as i16;
        mod1 = mod2;
        if FM_ALGORITHM[op as usize][0 as i32 as usize][connect as usize] != 0 {
            mod2 = (mod2 as i32 | self.fm_op1[channel as usize][0 as i32 as usize] as i32) as i16
        }
        if FM_ALGORITHM[op as usize][1 as i32 as usize][connect as usize] != 0 {
            mod1 = (mod1 as i32 | self.fm_op1[channel as usize][1 as i32 as usize] as i32) as i16
        }
        if FM_ALGORITHM[op as usize][2 as i32 as usize][connect as usize] != 0 {
            mod1 = (mod1 as i32 | self.fm_op2[channel as usize] as i32) as i16
        }
        if FM_ALGORITHM[op as usize][3 as i32 as usize][connect as usize] != 0 {
            mod2 = (mod2 as i32 | self.fm_out[prevslot as usize] as i32) as i16
        }
        if FM_ALGORITHM[op as usize][4 as i32 as usize][connect as usize] != 0 {
            mod1 = (mod1 as i32 | self.fm_out[prevslot as usize] as i32) as i16
        }
        mod_0 = (mod1 as i32 + mod2 as i32) as i16;
        if op == 0 as i32 as u32 {
            /* Feedback */
            mod_0 = (mod_0 as i32 >> 10 as i32 - self.fb[channel as usize] as i32) as i16;
            if self.fb[channel as usize] == 0 {
                mod_0 = 0 as i32 as i16
            }
        } else {
            mod_0 = (mod_0 as i32 >> 1 as i32) as i16
        }
        self.fm_mod[slot as usize] = mod_0 as u16;
        slot = self
            .cycles
            .wrapping_add(18 as i32 as u32)
            .wrapping_rem(24 as i32 as u32);
        /* OP1 */
        if slot.wrapping_div(6 as i32 as u32) == 0 as i32 as u32 {
            self.fm_op1[channel as usize][1 as i32 as usize] =
                self.fm_op1[channel as usize][0 as i32 as usize];
            self.fm_op1[channel as usize][0 as i32 as usize] = self.fm_out[slot as usize]
        }
        /* OP2 */
        if slot.wrapping_div(6 as i32 as u32) == 2 as i32 as u32 {
            self.fm_op2[channel as usize] = self.fm_out[slot as usize]
        };
    }

    fn opn2_ch_generate(&mut self) {
        let mut slot = self
            .cycles
            .wrapping_add(18 as i32 as u32)
            .wrapping_rem(24 as i32 as u32);
        let mut channel = self.channel;
        let mut op = slot.wrapping_div(6 as i32 as u32);
        let mut test_dac = self.mode_test_2c[5 as i32 as usize] as u32;
        let mut acc = self.ch_acc[channel as usize];
        let mut add = test_dac as i16;
        let mut sum = 0 as i32 as i16;
        if op == 0 as i32 as u32 && test_dac == 0 {
            acc = 0 as i32 as i16
        }
        if FM_ALGORITHM[op as usize][5 as i32 as usize][self.connect[channel as usize] as usize]
            != 0
            && test_dac == 0
        {
            add = (add as i32 + (self.fm_out[slot as usize] as i32 >> 5 as i32)) as i16
        }
        sum = (acc as i32 + add as i32) as i16;
        /* Clamp */
        if sum as i32 > 255 as i32 {
            sum = 255 as i32 as i16
        } else if (sum as i32) < -(256 as i32) {
            sum = -(256 as i32) as i16
        }
        if op == 0 as i32 as u32 || test_dac != 0 {
            self.ch_out[channel as usize] = self.ch_acc[channel as usize]
        }
        self.ch_acc[channel as usize] = sum;
    }

    fn opn2_ch_output(&mut self) {
        let mut cycles = self.cycles;
        let mut slot = self.cycles;
        let mut channel = self.channel;
        let mut test_dac = self.mode_test_2c[5 as i32 as usize] as u32;
        let mut out: i16 = 0;
        let mut sign: i16 = 0;
        let mut out_en: u32 = 0;
        self.ch_read = self.ch_lock;
        if slot < 12 as i32 as u32 {
            /* Ch 4,5,6 */
            channel = channel.wrapping_add(1)
        }
        if cycles & 3 as i32 as u32 == 0 as i32 as u32 {
            if test_dac == 0 {
                /* Lock value */
                self.ch_lock = self.ch_out[channel as usize]
            }
            self.ch_lock_l = self.pan_l[channel as usize];
            self.ch_lock_r = self.pan_r[channel as usize]
        }
        /* Ch 6 */
        if cycles >> 2 as i32 == 1 as i32 as u32 && self.dacen as i32 != 0 || test_dac != 0 {
            out = self.dacdata;
            out = ((out as i32) << 7 as i32) as i16;
            out = (out as i32 >> 7 as i32) as i16
        } else {
            out = self.ch_lock
        }
        self.mol = 0 as i32 as i16;
        self.mor = 0 as i32 as i16;
        if chip_type & ym3438_mode_ym2612 as i32 as u32 != 0 {
            out_en = (cycles & 3 as i32 as u32 == 3 as i32 as u32 || test_dac != 0) as i32 as u32;
            /* YM2612 DAC emulation(not verified) */
            sign = (out as i32 >> 8 as i32) as i16;
            if out as i32 >= 0 as i32 {
                out += 1;
                sign += 1
            }
            if self.ch_lock_l as i32 != 0 && out_en != 0 {
                self.mol = out
            } else {
                self.mol = sign
            }
            if self.ch_lock_r as i32 != 0 && out_en != 0 {
                self.mor = out
            } else {
                self.mor = sign
            }
            /* Amplify signal */
            self.mol = (self.mol as i32 * 3 as i32) as i16;
            self.mor = (self.mor as i32 * 3 as i32) as i16
        } else {
            out_en = (cycles & 3 as i32 as u32 != 0 as i32 as u32 || test_dac != 0) as i32 as u32;
            if self.ch_lock_l as i32 != 0 && out_en != 0 {
                self.mol = out
            }
            if self.ch_lock_r as i32 != 0 && out_en != 0 {
                self.mor = out
            }
        };
    }

    fn opn2_fm_generate(&mut self) {
        let mut slot = self
            .cycles
            .wrapping_add(19 as i32 as u32)
            .wrapping_rem(24 as i32 as u32);
        /* Calculate phase */
        let mut phase = ((self.fm_mod[slot as usize] as u32)
            .wrapping_add(self.pg_phase[slot as usize] >> 10 as i32)
            & 0x3ff as i32 as u32) as u16;
        let mut quarter: u16 = 0;
        let mut level: u16 = 0;
        let mut output: i16 = 0;
        if phase as i32 & 0x100 as i32 != 0 {
            quarter = ((phase as i32 ^ 0xff as i32) & 0xff as i32) as u16
        } else {
            quarter = (phase as i32 & 0xff as i32) as u16
        }
        level = logsinrom[quarter as usize];
        /* Apply envelope */
        level = (level as i32 + ((self.eg_out[slot as usize] as i32) << 2 as i32)) as u16;
        /* Transform */
        if level as i32 > 0x1fff as i32 {
            level = 0x1fff as i32 as u16
        }
        output = ((EXP_ROM[(level as i32 & 0xff as i32 ^ 0xff as i32) as usize] as i32
            | 0x400 as i32)
            << 2 as i32
            >> (level as i32 >> 8 as i32)) as i16;
        if phase as i32 & 0x200 as i32 != 0 {
            output = ((!(output as i32)
                ^ (self.mode_test_21[4 as i32 as usize] as i32) << 13 as i32)
                + 1 as i32) as i16
        } else {
            output =
                (output as i32 ^ (self.mode_test_21[4 as i32 as usize] as i32) << 13 as i32) as i16
        }
        output = ((output as i32) << 2 as i32) as i16;
        output = (output as i32 >> 2 as i32) as i16;
        self.fm_out[slot as usize] = output;
    }

    fn opn2_do_timer_a(&mut self) {
        let mut time: u16 = 0;
        let mut load: u8 = 0;
        load = self.timer_a_overflow;
        if self.cycles == 2 as i32 as u32 {
            /* Lock load value */
            load = (load as i32
                | (self.timer_a_load_lock == 0 && self.timer_a_load as i32 != 0) as i32)
                as u8;
            self.timer_a_load_lock = self.timer_a_load;
            if self.mode_csm != 0 {
                /* CSM KeyOn */
                self.mode_kon_csm = load
            } else {
                self.mode_kon_csm = 0 as i32 as u8
            }
        }
        /* Load counter */
        if self.timer_a_load_latch != 0 {
            time = self.timer_a_reg
        } else {
            time = self.timer_a_cnt
        }
        self.timer_a_load_latch = load;
        /* Increase counter */
        if self.cycles == 1 as i32 as u32 && self.timer_a_load_lock as i32 != 0
            || self.mode_test_21[2 as i32 as usize] as i32 != 0
        {
            time = time.wrapping_add(1)
        }
        /* Set overflow flag */
        if self.timer_a_reset != 0 {
            self.timer_a_reset = 0 as i32 as u8;
            self.timer_a_overflow_flag = 0 as i32 as u8
        } else {
            self.timer_a_overflow_flag = (self.timer_a_overflow_flag as i32
                | self.timer_a_overflow as i32 & self.timer_a_enable as i32)
                as u8
        }
        self.timer_a_overflow = (time as i32 >> 10 as i32) as u8;
        self.timer_a_cnt = (time as i32 & 0x3ff as i32) as u16;
    }

    fn opn2_do_timer_b(&mut self) {
        let mut time: u16 = 0;
        let mut load: u8 = 0;
        load = self.timer_b_overflow;
        if self.cycles == 2 as i32 as u32 {
            /* Lock load value */
            load = (load as i32
                | (self.timer_b_load_lock == 0 && self.timer_b_load as i32 != 0) as i32)
                as u8;
            self.timer_b_load_lock = self.timer_b_load
        }
        /* Load counter */
        if self.timer_b_load_latch != 0 {
            time = self.timer_b_reg
        } else {
            time = self.timer_b_cnt
        }
        self.timer_b_load_latch = load;
        /* Increase counter */
        if self.cycles == 1 as i32 as u32 {
            self.timer_b_subcnt = self.timer_b_subcnt.wrapping_add(1)
        }
        if self.timer_b_subcnt as i32 == 0x10 as i32 && self.timer_b_load_lock as i32 != 0
            || self.mode_test_21[2 as i32 as usize] as i32 != 0
        {
            time = time.wrapping_add(1)
        }
        self.timer_b_subcnt = (self.timer_b_subcnt as i32 & 0xf as i32) as u8;
        /* Set overflow flag */
        if self.timer_b_reset != 0 {
            self.timer_b_reset = 0 as i32 as u8;
            self.timer_b_overflow_flag = 0 as i32 as u8
        } else {
            self.timer_b_overflow_flag = (self.timer_b_overflow_flag as i32
                | self.timer_b_overflow as i32 & self.timer_b_enable as i32)
                as u8
        }
        self.timer_b_overflow = (time as i32 >> 8 as i32) as u8;
        self.timer_b_cnt = (time as i32 & 0xff as i32) as u16;
    }

    fn opn2_key_on(&mut self) {
        let mut slot = self.cycles;
        let mut chan = self.channel;
        /* Key On */
        self.eg_kon_latch[slot as usize] = self.mode_kon[slot as usize];
        self.eg_kon_csm[slot as usize] = 0 as i32 as u8;
        if self.channel == 2 as i32 as u32 && self.mode_kon_csm as i32 != 0 {
            /* CSM Key On */
            self.eg_kon_latch[slot as usize] = 1 as i32 as u8;
            self.eg_kon_csm[slot as usize] = 1 as i32 as u8
        }
        if self.cycles == self.mode_kon_channel as u32 {
            /* OP1 */
            self.mode_kon[chan as usize] = self.mode_kon_operator[0 as i32 as usize];
            /* OP2 */
            self.mode_kon[chan.wrapping_add(12 as i32 as u32) as usize] =
                self.mode_kon_operator[1 as i32 as usize];
            /* OP3 */
            self.mode_kon[chan.wrapping_add(6 as i32 as u32) as usize] =
                self.mode_kon_operator[2 as i32 as usize];
            /* OP4 */
            self.mode_kon[chan.wrapping_add(18 as i32 as u32) as usize] =
                self.mode_kon_operator[3 as i32 as usize]
        };
    }

    pub fn opn2_reset(&mut self) {
        let mut i: u32 = 0;
        *self = unsafe { std::mem::zeroed() };

        i = 0 as i32 as u32;
        while i < 24 as i32 as u32 {
            self.eg_out[i as usize] = 0x3ff as i32 as u16;
            self.eg_level[i as usize] = 0x3ff as i32 as u16;
            self.eg_state[i as usize] = eg_num_release as i32 as u8;
            self.multi[i as usize] = 1 as i32 as u8;
            i = i.wrapping_add(1)
        }
        i = 0 as i32 as u32;
        while i < 6 as i32 as u32 {
            self.pan_l[i as usize] = 1 as i32 as u8;
            self.pan_r[i as usize] = 1 as i32 as u8;
            i = i.wrapping_add(1)
        }
    }

    pub fn opn2_set_chip_type(mut type_0: u32) {
        todo!();
        //chip_type = type_0;
    }

    pub fn opn2_clock(&mut self, mut buffer: &mut [i16]) {
        let mut slot = self.cycles;
        self.lfo_inc = self.mode_test_21[1 as i32 as usize];
        self.pg_read >>= 1 as i32;
        self.eg_read[1 as i32 as usize] >>= 1 as i32;
        self.eg_cycle = self.eg_cycle.wrapping_add(1);
        /* Lock envelope generator timer value */
        if self.cycles == 1 as i32 as u32 && self.eg_quotient as i32 == 2 as i32 {
            if self.eg_cycle_stop != 0 {
                self.eg_shift_lock = 0 as i32 as u8
            } else {
                self.eg_shift_lock = (self.eg_shift as i32 + 1 as i32) as u8
            }
            self.eg_timer_low_lock = (self.eg_timer as i32 & 0x3 as i32) as u8
        }
        /* Cycle specific functions */
        match self.cycles {
            0 => {
                self.lfo_pm = (self.lfo_cnt as i32 >> 2 as i32) as u8;
                if self.lfo_cnt as i32 & 0x40 as i32 != 0 {
                    self.lfo_am = (self.lfo_cnt as i32 & 0x3f as i32) as u8
                } else {
                    self.lfo_am = (self.lfo_cnt as i32 ^ 0x3f as i32) as u8
                }
                self.lfo_am = ((self.lfo_am as i32) << 1 as i32) as u8
            }
            1 => {
                self.eg_quotient = self.eg_quotient.wrapping_add(1);
                self.eg_quotient = (self.eg_quotient as i32 % 3 as i32) as u16;
                self.eg_cycle = 0 as i32 as u8;
                self.eg_cycle_stop = 1 as i32 as u8;
                self.eg_shift = 0 as i32 as u8;
                self.eg_timer_inc =
                    (self.eg_timer_inc as i32 | self.eg_quotient as i32 >> 1 as i32) as u8;
                self.eg_timer = (self.eg_timer as i32 + self.eg_timer_inc as i32) as u16;
                self.eg_timer_inc = (self.eg_timer as i32 >> 12 as i32) as u8;
                self.eg_timer = (self.eg_timer as i32 & 0xfff as i32) as u16
            }
            2 => {
                self.pg_read = self.pg_phase[21 as i32 as usize] & 0x3ff as i32 as u32;
                self.eg_read[1 as i32 as usize] = self.eg_out[0 as i32 as usize] as u32
            }
            13 => {
                self.eg_cycle = 0 as i32 as u8;
                self.eg_cycle_stop = 1 as i32 as u8;
                self.eg_shift = 0 as i32 as u8;
                self.eg_timer = (self.eg_timer as i32 + self.eg_timer_inc as i32) as u16;
                self.eg_timer_inc = (self.eg_timer as i32 >> 12 as i32) as u8;
                self.eg_timer = (self.eg_timer as i32 & 0xfff as i32) as u16
            }
            23 => self.lfo_inc = (self.lfo_inc as i32 | 1 as i32) as u8,
            _ => {}
        }
        self.eg_timer = (self.eg_timer as i32
            & !((self.mode_test_21[5 as i32 as usize] as i32) << self.eg_cycle as i32))
            as u16;
        if (self.eg_timer as i32 >> self.eg_cycle as i32
            | self.pin_test_in as i32 & self.eg_custom_timer as i32)
            & self.eg_cycle_stop as i32
            != 0
        {
            self.eg_shift = self.eg_cycle;
            self.eg_cycle_stop = 0 as i32 as u8
        }
        self.opn2_do_io();
        self.opn2_do_timer_a();
        self.opn2_do_timer_b();
        self.opn2_key_on();
        self.opn2_ch_output();
        self.opn2_ch_generate();
        self.opn2_fm_prepare();
        self.opn2_fm_generate();
        self.opn2_phase_generate();
        self.opn2_phase_calc_increment();
        self.opn2_envelope_adsr();
        self.opn2_envelope_generate();
        self.opn2_envelope_ssg_eg();
        self.opn2_envelope_prepare();
        /* Prepare fnum & block */
        if self.mode_ch3 != 0 {
            /* Channel 3 special mode */
            match slot {
                1 => {
                    /* OP1 */
                    self.pg_fnum = self.fnum_3ch[1 as i32 as usize];
                    self.pg_block = self.block_3ch[1 as i32 as usize];
                    self.pg_kcode = self.kcode_3ch[1 as i32 as usize]
                }
                7 => {
                    /* OP3 */
                    self.pg_fnum = self.fnum_3ch[0 as i32 as usize];
                    self.pg_block = self.block_3ch[0 as i32 as usize];
                    self.pg_kcode = self.kcode_3ch[0 as i32 as usize]
                }
                13 => {
                    /* OP2 */
                    self.pg_fnum = self.fnum_3ch[2 as i32 as usize];
                    self.pg_block = self.block_3ch[2 as i32 as usize];
                    self.pg_kcode = self.kcode_3ch[2 as i32 as usize]
                }
                19 | _ => {
                    /* OP4 */
                    self.pg_fnum = self.fnum[self
                        .channel
                        .wrapping_add(1 as i32 as u32)
                        .wrapping_rem(6 as i32 as u32)
                        as usize];
                    self.pg_block = self.block[self
                        .channel
                        .wrapping_add(1 as i32 as u32)
                        .wrapping_rem(6 as i32 as u32)
                        as usize];
                    self.pg_kcode = self.kcode[self
                        .channel
                        .wrapping_add(1 as i32 as u32)
                        .wrapping_rem(6 as i32 as u32)
                        as usize]
                }
            }
        } else {
            self.pg_fnum = self.fnum[self
                .channel
                .wrapping_add(1 as i32 as u32)
                .wrapping_rem(6 as i32 as u32) as usize];
            self.pg_block = self.block[self
                .channel
                .wrapping_add(1 as i32 as u32)
                .wrapping_rem(6 as i32 as u32) as usize];
            self.pg_kcode = self.kcode[self
                .channel
                .wrapping_add(1 as i32 as u32)
                .wrapping_rem(6 as i32 as u32) as usize]
        }
        self.opn2_update_lfo();
        self.opn2_do_reg_write();
        self.cycles = self
            .cycles
            .wrapping_add(1 as i32 as u32)
            .wrapping_rem(24 as i32 as u32);
        self.channel = self.cycles.wrapping_rem(6 as i32 as u32);
        buffer[0] = self.mol;
        buffer[1] = self.mor;
        if self.status_time != 0 {
            self.status_time = self.status_time.wrapping_sub(1)
        };
    }

    pub fn opn2_write(&mut self, mut port: u32, mut data: u8) {
        port &= 3 as i32 as u32;
        self.write_data = (port << 7 as i32 & 0x100 as i32 as u32 | data as u32) as u16;
        if port & 1 as i32 as u32 != 0 {
            /* Data */
            self.write_d = (self.write_d as i32 | 1 as i32) as u8
        } else {
            /* Address */
            self.write_a = (self.write_a as i32 | 1 as i32) as u8
        };
    }

    pub fn opn2_set_test_pin(&mut self, mut value: u32) {
        self.pin_test_in = (value & 1 as i32 as u32) as u8;
    }

    pub fn opn2_read_test_pin(&self) -> u32 {
        if self.mode_test_2c[7 as i32 as usize] == 0 {
            return 0 as i32 as u32;
        }
        return (self.cycles == 23 as i32 as u32) as i32 as u32;
    }

    pub fn opn2_read_irq_pin(&self) -> u32 {
        return (self.timer_a_overflow_flag as i32 | self.timer_b_overflow_flag as i32) as u32;
    }

    pub fn opn2_read(&mut self, mut port: u32) -> u8 {
        if port & 3 as i32 as u32 == 0 as i32 as u32
            || chip_type & ym3438_mode_readmode as i32 as u32 != 0
        {
            if self.mode_test_21[6 as i32 as usize] != 0 {
                /* Read test data */
                let mut slot = self
                    .cycles
                    .wrapping_add(18 as i32 as u32)
                    .wrapping_rem(24 as i32 as u32);
                let mut testdata = ((self.pg_read & 0x1 as i32 as u32) << 15 as i32
                    | (self.eg_read[self.mode_test_21[0 as i32 as usize] as usize]
                        & 0x1 as i32 as u32)
                        << 14 as i32) as u16;
                if self.mode_test_2c[4 as i32 as usize] != 0 {
                    testdata = (testdata as i32 | self.ch_read as i32 & 0x1ff as i32) as u16
                } else {
                    testdata =
                        (testdata as i32 | self.fm_out[slot as usize] as i32 & 0x3fff as i32) as u16
                }
                if self.mode_test_21[7 as i32 as usize] != 0 {
                    self.status = (testdata as i32 & 0xff as i32) as u8
                } else {
                    self.status = (testdata as i32 >> 8 as i32) as u8
                }
            } else {
                self.status = ((self.busy as i32) << 7 as i32
                    | (self.timer_b_overflow_flag as i32) << 1 as i32
                    | self.timer_a_overflow_flag as i32) as u8
            }
            if chip_type & ym3438_mode_ym2612 as i32 as u32 != 0 {
                self.status_time = 300000 as i32 as u32
            } else {
                self.status_time = 40000000 as i32 as u32
            }
        }
        if self.status_time != 0 {
            return self.status;
        }
        return 0 as i32 as u8;
    }
}
