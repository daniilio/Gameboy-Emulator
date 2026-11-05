enum Instruction {
    ADDByte(SourceByte),
    ADDWord(SourceWord),
    ADDSP(SourceSP),
    ADC(SourceByte),
    SUB(SourceByte),
    SBC(SourceByte),
    AND(SourceByte),
    OR(SourceByte),
    XOR(SourceByte),
    CP(TargetByte),
    INCByte(TargetByte),
    INCWord(TargetWord),
    DECByte(TargetByte),
    DECWord(TargetWord),
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RLCA,
    CPL,
    BIT(SourceBit, TargetByte),
    RES(SourceBit, TargetByte),
    SET(SourceBit, TargetByte),
    SRL(TargetByte),
    RR(TargetByte),
    RL(TargetByte),
    RRC(TargetByte),
    RLC(TargetByte),
    SRA(TargetByte),
    SLA(TargetByte),
    SWAP(TargetByte),
    JP(SourceCondition),
    JR(SourceCondition),
    JPHL(SourceCondition),
    LD(SourceByte, TargetByte),
    LDWord(SourceLDWord, TargetWord),
    LDAFromIndirect(SourceLDIndirect),
    LDIndirectFromA(TargetLDIndirect),
    LDAFromByteAddress,
    LDByteAddressFromA,
    PUSH(SourceStack),
    POP(TargetStack),
    CALL(SourceCondition),
    RET(SourceCondition),
    NOP,
    HALT,
    RST,  // TODO: 
}


// ==================================================
// Instruction Sources
// ==================================================
enum SourceBit {
    Bit0,
    Bit1,
    Bit2,
    Bit3,
    Bit4,
    Bit5,
    Bit6,
    Bit7,
}

enum SourceByte {
    A, 
    B, 
    C, 
    D, 
    E, 
    H,
    L,
    D8,
    IndirectHL,
}

enum SourceWord {
    BC, 
    DE, 
    HL, 
    SP,
}

enum SourceCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

enum SourceSP {
    D8,
}


// ==================================================
// Instruction Targets
// ==================================================
enum TargetByte {
    A, 
    B, 
    C, 
    D, 
    E, 
    H, 
    L, 
    IndirectHL,
}

enum TargetWord {
    BC, 
    DE, 
    HL, 
    SP,
}


// ==================================================
// Instruction Sources Misc.
// ==================================================
enum SourceLDWord {
    D16,
}

enum SourceLDIndirect {
    BC,
    DE,
    HLPlus,
    HLMinus,
}

enum SourceStack {
    BC,
    DE,
    HL,
    AF,
}


// ==================================================
// Instruction Targets Misc.
// ==================================================
enum TargetLDIndirect {
    BC,
    DE,
    HLPlus,
    HLMinus,
}

enum TargetStack {
    BC,
    DE,
    HL,
    AF,
}


// ==================================================
// Instruction Implementations
// ==================================================
pub fn add_byte(cpu: &mut CPU, value: u8) -> u8 {
    let a = cpu.regs.get_a();
    let (result, did_overflow) = a.overflowing_add(value);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(value & 0xF + a & 0xF > 0xF);
    cpu.regs.set_carry_flag(did_overflow);

    result
}

pub fn add_word(cpu: &mut CPU, value: u16) -> u16 {
    let hl = cpu.regs.get_hl();
    let (result, did_overflow) = hl.overflowing_add(value);

    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_carry_flag(did_overflow);

    // Half Carry is set if there's a carry from bit 11 to bit 12.
    cpu.regs.set_half_carry_flag(
        (hl & 0xFFF) + (value & 0xFFF) > 0xFFF
    );

    result
}

pub fn add_sp(cpu: &mut CPU, value: i8) -> u16 {
    let sp = cpu.regs.get_sp();
    let value_u16 = value as u16;

    let result = sp.wrapping_add(value_u16);

    // ADDByte SP, e8 always clears Z and N.
    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);

    // H and C are based on the low byte of SP
    // and the 8-bit two's-complement immediate.
    cpu.regs.set_half_carry_flag(
        (sp & 0xF) + (value_u16 & 0xF) > 0xF
    );

    cpu.regs.set_carry_flag(
        (sp & 0xFF) + (value_u16 & 0xFF) > 0xFF
    );

    result
}

pub fn adc(cpu: &mut CPU, target: u8) -> u8 {
    let a = cpu.regs.get_a();
    let carry = cpu.regs.is_carry_flag_set();

    // Save the old carry before modifying the flags.
    let carry_value: u8 = if carry { 1 } else { 0 };

    let (result1, overflow1) = a.overflowing_add(target);
    let (result, overflow2) = result1.overflowing_add(carry_value);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_carry_flag(overflow1 || overflow2);

    // Half Carry is set if there's a carry from bit 3 to bit 4.
    cpu.regs.set_half_carry_flag(
        (a & 0xF) + (target & 0xF) + carry_value > 0xF
    );

    result
}

pub fn sub(cpu: &mut CPU, target: u8) -> u8 {
    let a = cpu.regs.get_a();

    let (result, did_underflow) = a.overflowing_sub(target);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_carry_flag(did_underflow);

    // Half Carry/Borrow from bit 4.
    cpu.regs.set_half_carry_flag(
        (a & 0xF) < (target & 0xF)
    );

    result
}

pub fn sbc(cpu: &mut CPU, target: u8) -> u8 {
    let a = cpu.regs.get_a();
    let carry = cpu.regs.is_carry_flag_set();

    // Save the old carry before modifying the flags.
    let carry_value: u8 = if carry { 1 } else { 0 };

    let (result1, underflow1) = a.overflowing_sub(target);
    let (result, underflow2) = result1.overflowing_sub(carry_value);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_carry_flag(underflow1 || underflow2);

    cpu.regs.set_half_carry_flag(
        (a & 0xF) < (target & 0xF) + carry_value
    );

    result
}

pub fn and(cpu: &mut CPU, target: u8) -> u8 {
    let a = cpu.regs.get_a();
    let result = a & target;

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(true);
    cpu.regs.set_carry_flag(false);

    result
}

pub fn or(cpu: &mut CPU, target: u8) -> u8 {
    let a = cpu.regs.get_a();
    let result = a | target;

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);

    result
}

pub fn xor(cpu: &mut CPU, target: u8) -> u8 {
    let a = cpu.regs.get_a();
    let result = a ^ target;

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);

    result
}

pub fn cp(cpu: &mut CPU, target: u8) {
    let a = cpu.regs.get_a();

    let (result, did_underflow) = a.overflowing_sub(target);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_carry_flag(did_underflow);

    // Half Carry/Borrow from bit 4.
    cpu.regs.set_half_carry_flag(
        (a & 0xF) < (target & 0xF)
    );

    // CP does NOT modify A.
}

pub fn inc_byte(cpu: &mut CPU, value: u8) -> u8 {
    let result = value.wrapping_add(1);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(
        (value & 0xF) + 1 > 0xF
    );
    // INC does not modify Carry flag.

    result
}

pub fn inc_word(cpu: &mut CPU, value: u16) -> u16 {
    // INC rr does not modify any flags.
    value.wrapping_add(1)
}

pub fn dec_byte(cpu: &mut CPU, value: u8) -> u8 {
    let result = value.wrapping_sub(1);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag(
        (value & 0xF) < 1
    );
    // DEC does not modify Carry flag.

    result
}

pub fn dec_word(cpu: &mut CPU, value: u16) -> u16 {
    // DEC rr does not modify any flags.
    value.wrapping_sub(1)
}

pub fn ccf(cpu: &mut CPU) {
    // CCF does not modify ZERO flag.
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(!cpu.regs.is_carry_flag_set());
}

pub fn scf(cpu: &mut CPU) {
    // SCF does not modify the ZERO flag.
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(true);
}

pub fn rra(cpu: &mut CPU) -> u8 {
    let a = cpu.regs.get_a();

    let mut result = a >> 1;
    let c = if cpu.regs.is_carry_flag_set() {1 << 7} else {0};
    result = result | c;

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag((a & 0b1) == 1);
    
    result
}

pub fn rla(cpu: &mut CPU) -> u8 {
    let a = cpu.regs.get_a();

    let mut result = a << 1;
    let c = if cpu.regs.is_carry_flag_set() {1} else {0};
    result = result | c;

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(((a >> 7) & 0b1) == 1);

    result
}

pub fn rrca(cpu: &mut CPU) -> u8 {
    let a = cpu.regs.get_a();

    let mut result = a >> 1;
    result = result | ((a & 0b1) << 7);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag((a & 0b1) == 1);

    result
}

pub fn rlca(cpu: &mut CPU) -> u8 {
    let a = cpu.regs.get_a();

    let mut result = a << 1;
    result = result | ((a >> 7) & 0b1);

    cpu.regs.set_zero_flag(false);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(((a >> 7) & 0b1) == 1);

    result
}

pub fn cpl(cpu: &mut CPU) -> u8 {
    let a = cpu.regs.get_a();

    let result = !a;

    cpu.regs.set_subtract_flag(true);
    cpu.regs.set_half_carry_flag(true);

    result
}

pub fn bit(cpu: &mut CPU, bit: u8, value: u8) {
    cpu.regs.set_zero_flag(((value >> bit) & 0b1) != 1);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(true);
}

pub fn res(bit: u8, value: u8) -> u8 {
    (!(1 << bit)) & value
}

pub fn set(bit: u8, value: u8) -> u8 {
    (1 << bit) | value
}

pub fn srl(cpu: &mut CPU, value: u8) -> u8 {
    let result = value >> 1;

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag((value & 0b1) == 1);

    result
}

pub fn rr(cpu: &mut CPU, value: u8) -> u8 {
    let mut result = value >> 1;

    let c = if (cpu.regs.is_carry_flag_set()) {1} else {0};
    result = result | (c << 7);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag((value & 0b1) == 1);

    result
}

pub fn rl(cpu: &mut CPU, value: u8) -> u8 {
    let mut result = value << 1;

    let c = if (cpu.regs.is_carry_flag_set()) {1} else {0};
    result = result | c;

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(((value >> 7) & 0b1) == 1);

    result
}

pub fn rrc(cpu: &mut CPU, value: u8) -> u8 {
    let result = (value >> 1) | ((value & 0b1) << 7);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag((value & 0b1) == 1);

    result
}

pub fn rlc(cpu: &mut CPU, value: u8) -> u8 {
    let result = (value << 1) | ((value >> 7) & 0b1);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(((value >> 7) & 0b1) == 1);

    result
}

pub fn sra(cpu: &mut CPU, value: u8) -> u8 {
    let result = ((value as i8) >> 1) as u8;

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag((value & 0b1) == 1);

    result
}

pub fn sla(cpu: &mut CPU, value: u8) -> u8 {
    let result = ((value as i8) << 1) as u8;

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(((value >> 7) & 0b1) == 1);

    result
}

pub fn swap(cpu: &mut CPU, value: u8) -> u8 {
    let result = ((value & 0xF) << 4) | ((value & 0xF0) >> 4);

    cpu.regs.set_zero_flag(result == 0);
    cpu.regs.set_subtract_flag(false);
    cpu.regs.set_half_carry_flag(false);
    cpu.regs.set_carry_flag(false);

    result
}

pub fn jp(cpu: &CPU, should_jump: bool) -> u16 {
    if should_jump {
        let addr = cpu.bus.read_word(cpu.pc.wrapping_add(1));
        addr
    } else {
        cpu.pc.wrapping_add(3)
    }
}

pub fn jr(cpu: &CPU, should_jump: bool) -> u16 {
    if should_jump {
        let offset = cpu.bus.read_byte(cpu.pc.wrapping_add(1)) as i8;
        cpu.pc.wrapping_add(offset)
    } else {
        cpu.pc.wrapping_Add(2)
    }
}

pub fn jphl(cpu: &CPU) -> u16 {
    cpu.regs.get_hl()
}

pub fn push(cpu: &mut CPU, value: u16) {
    let least_sig_byte = (value & 0xFF) as u8;
    let most_sig_byte = ((value & 0xFF00) >> 8) as u8;

    let sp = cpu.regs.get_sp();
    sp = sp.wrapping_sub(1);
    cpu.bus.write_byte(sp, most_sig_byte);

    sp = sp.wrapping_sub(1);
    cpu.bus.write_byte(sp, least_sig_byte);

    cpu.regs.set_sp(sp);
}

pub fn pop(cpu: &mut CPU) -> u16 {
    let sp = cpu.regs.get_sp();

    let least_sig_byte = cpu.bus.read_byte(sp);
    sp = sp.wrapping_add(1);
    
    let most_sig_byte = cpu.bus.read_byte(sp);
    sp = sp.wrapping_add(1);

    cpu.regs.set_sp(sp);

    ((most_sig_byte as u16) << 8) | (least_sig_byte as u16)
}

pub fn call(cpu: &mut CPU, condition: bool) -> u16 {
    if (condition) {
        let addr = cpu.bus.read_word(cpu.pc.wrapping_add(1));
        Self::push(cpu, cpu.pc.wrapping_add(3));
        addr
    }
    cpu.pc.wrapping_add(3)
}

pub fn ret(cpu: &mut CPU, condition: bool) -> u16 {
    if (condition) {
        let addr = Self::pop(cpu);
        addr
    }
    self.pc.wrapping_add(1)
}
