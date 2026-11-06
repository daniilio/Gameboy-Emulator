use crate::helpers::*;

// Flags register bits
const ZERO_POSITION: u8 = 7;
const SUBTRACT_POSITION: u8 = 6;
const HALF_CARRY_POSITION: u8 = 5;
const CARRY_POSITION: u8 = 4;

pub struct CPURegisters {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
}

impl CPURegisters {
    pub fn get_a(&self) -> u8 {
        get_msb(self.af)
    }

    pub fn get_f(&self) -> u8 {
        get_lsb(self.af)
    }

    pub fn get_b(&self) -> u8 {
        get_msb(self.bc)
    }

    pub fn get_c(&self) -> u8 {
        get_lsb(self.bc)
    }

    pub fn get_d(&self) -> u8 {
        get_msb(self.de)
    }

    pub fn get_e(&self) -> u8 {
        get_lsb(self.de)
    }

    pub fn get_h(&self) -> u8 {
        get_msb(self.hl)
    }

    pub fn get_l(&self) -> u8 {
        get_lsb(self.hl)
    }

    pub fn set_a(&mut self, value: u8) {
        set_msb(&mut self.af, value);
    }

    pub fn set_f(&mut self, value: u8) {
        set_lsb(&mut self.af, value & 0xF0);
    }

    pub fn set_b(&mut self, value: u8) {
        set_msb(&mut self.bc, value);
    }

    pub fn set_c(&mut self, value: u8) {
        set_lsb(&mut self.bc, value);
    }

    pub fn set_d(&mut self, value: u8) {
        set_msb(&mut self.de, value);
    }

    pub fn set_e(&mut self, value: u8) {
        set_lsb(&mut self.de, value);
    }

    pub fn set_h(&mut self, value: u8) {
        set_msb(&mut self.hl, value);
    }

    pub fn set_l(&mut self, value: u8) {
        set_lsb(&mut self.hl, value);
    }

    pub fn get_af(&self) -> u16 {
        self.af
    }

    pub fn get_bc(&self) -> u16 {
        self.bc
    }

    pub fn get_de(&self) -> u16 {
        self.de
    }

    pub fn get_hl(&self) -> u16 {
        self.hl
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn set_af(&mut self, value: u16) {
        self.af = value & 0xFFF0;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.bc = value;
    }

    pub fn set_de(&mut self, value: u16) {
        self.de = value;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.hl = value;
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    pub fn is_zero_flag_set(&self) -> bool {
        is_bit_set(&self.af, ZERO_POSITION)
    }

    pub fn is_subtract_flag_set(&self) -> bool {
        is_bit_set(&self.af, SUBTRACT_POSITION)
    }

    pub fn is_half_carry_flag_set(&self) -> bool {
        is_bit_set(&self.af, HALF_CARRY_POSITION)
    }

    pub fn is_carry_flag_set(&self) -> bool {
        is_bit_set(&self.af, CARRY_POSITION)
    }

    pub fn set_zero_flag(&mut self, value: bool) {
        set_bit(&mut self.af, ZERO_POSITION, value);
    }

    pub fn set_subtract_flag(&mut self, value: bool) {
        set_bit(&mut self.af, SUBTRACT_POSITION, value);
    }

    pub fn set_half_carry_flag(&mut self, value: bool) {
        set_bit(&mut self.af, HALF_CARRY_POSITION, value);
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        set_bit(&mut self.af, CARRY_POSITION, value);
    }
}
