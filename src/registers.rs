const ZERO_POSITION: u8 = 7;
const SUBTRACT_POSITION: u8 = 6;
const HALF_CARRY_POSITION: u8 = 5;
const CARRY_POSITION: u8 = 4;

struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
}

impl Registers {
    fn get_left(reg: u16) -> u8 {
        (reg >> 8) as u8
    }

    fn get_right(reg: u16) -> u8 {
        (reg & 0xFF) as u8
    }

    fn set_left(reg: &mut u16, value: u8) {
        *reg = ((value as u16) << 8) | (*reg & 0xFF);
    }

    fn set_right(reg: &mut u16, value: u8) {
        *reg = (value as u16) | (*reg & 0xFF00);
    }

    fn get_flag(&self, position: u8) -> bool {
        ((self.af >> position) & 1) != 0
    }

    fn set_flag(&mut self, position: u8, value: bool) {
        if value {
            self.af |= 1u16 << position;
        } else {
            self.af &= !(1u16 << position);
        }
    }

    fn get_a(&self) -> u8 {
        Self::get_left(self.af)
    }

    fn get_f(&self) -> u8 {
        Self::get_right(self.af)
    }

    fn get_b(&self) -> u8 {
        Self::get_left(self.bc)
    }

    fn get_c(&self) -> u8 {
        Self::get_right(self.bc)
    }

    fn get_d(&self) -> u8 {
        Self::get_left(self.de)
    }

    fn get_e(&self) -> u8 {
        Self::get_right(self.de)
    }

    fn get_h(&self) -> u8 {
        Self::get_left(self.hl)
    }

    fn get_l(&self) -> u8 {
        Self::get_right(self.hl)
    }

    fn set_a(&mut self, value: u8) {
        Self::set_left(&mut self.af, value);
    }

    fn set_f(&mut self, value: u8) {
        Self::set_right(&mut self.af, value & 0xF0);
    }

    fn set_b(&mut self, value: u8) {
        Self::set_left(&mut self.bc, value);
    }

    fn set_c(&mut self, value: u8) {
        Self::set_right(&mut self.bc, value);
    }

    fn set_d(&mut self, value: u8) {
        Self::set_left(&mut self.de, value);
    }

    fn set_e(&mut self, value: u8) {
        Self::set_right(&mut self.de, value);
    }

    fn set_h(&mut self, value: u8) {
        Self::set_left(&mut self.hl, value);
    }

    fn set_l(&mut self, value: u8) {
        Self::set_right(&mut self.hl, value);
    }

    fn get_af(&self) -> u16 {
        self.af
    }

    fn get_bc(&self) -> u16 {
        self.bc
    }

    fn get_de(&self) -> u16 {
        self.de
    }

    fn get_hl(&self) -> u16 {
        self.hl
    }

    fn get_sp(&self) -> u16 {
        self.sp
    }

    fn set_af(&mut self, value: u16) {
        self.af = value & 0xFFF0;
    }

    fn set_bc(&mut self, value: u16) {
        self.bc = value;
    }

    fn set_de(&mut self, value: u16) {
        self.de = value;
    }

    fn set_hl(&mut self, value: u16) {
        self.hl = value;
    }

    fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    fn get_zero_flag(&self) -> bool {
        self.get_flag(ZERO_POSITION)
    }

    fn get_subtract_flag(&self) -> bool {
        self.get_flag(SUBTRACT_POSITION)
    }

    fn get_half_carry_flag(&self) -> bool {
        self.get_flag(HALF_CARRY_POSITION)
    }

    fn get_carry_flag(&self) -> bool {
        self.get_flag(CARRY_POSITION)
    }

    fn set_zero_flag(&mut self, value: bool) {
        self.set_flag(ZERO_POSITION, value);
    }

    fn set_subtract_flag(&mut self, value: bool) {
        self.set_flag(SUBTRACT_POSITION, value);
    }

    fn set_half_carry_flag(&mut self, value: bool) {
        self.set_flag(HALF_CARRY_POSITION, value);
    }

    fn set_carry_flag(&mut self, value: bool) {
        self.set_flag(CARRY_POSITION, value);
    }
}
