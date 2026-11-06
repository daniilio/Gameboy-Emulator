use crate::instructions::*;
use crate::cpu_registers::CPURegisters;
use crate::bus::MemoryBus;

// Zero-page jump vectors
RST_VECTORS = {0000, 0008, 0010, 0018, 0020, 0028, 0030, 0038}

struct CPU {
    regs: CPURegisters,
    pc: u16,
    bus: MemoryBus,
    halt: bool,
}

impl CPU {
    pub fn new() -> Self
    {
        // TODO: Complete CPU constructor
        self.pc = 0x0000;
    }

    /// Executes the opcode addressed by CPU::pc.
    ///
    /// Returns the number of clock cycles consumed by the instruction.
    pub fn execute(&mut self) -> u8 {
        if (self.halt) {
            return;
        }
        
        let opcode = self.bus.read_byte(self.pc);
        
        let instruction: Instruction;
        if opcode == 0xCB {
            // Prefix instruction
            opcode = self.bus.read_byte(self.pc + 1);
            instruction = Self::match_prefix_opcode(opcode);
        } else {
            // Non-prefix instruction
            instruction = Self::match_opcode(opcode);
        }
        
        self.pc = self.execute_instruction(instruction);
        
        // TODO: execute needs to return the number of cycles consumed by the processed opcode
    }

    fn match_opcode(opcode: u8) -> Instruction {
        match opcode {
            0x80 => Instruction::ADD(SourceByte::B),
            0x81 => Instruction::ADD(SourceByte::C),
            0x82 => Instruction::ADD(SourceByte::D),
            // TODO: Map remaining opcodes
            _ => {
                panic!("Unkown instruction found for: 0x{}", opcode);
            }
        }
    }

    fn match_prefix_opcode(opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction::RLC(TargetByte::B),
            0x01 => Instruction::RLC(TargetByte::C),
            0x02 => Instruction::RLC(TargetByte::D),
            // TODO: Map remaining opcodes
            _ => {
                panic!("Unkown instruction found for: 0x{}:{}", 0xCB, opcode);
            }
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> u16 {
        // TODO: Return the appropriate instruction length for pc
        match instruction {
            Instruction::ADD(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::add_byte(self, value);
                self.regs.set_a(result);
                self.pc.wrapping_add(1)
            }
            Instruction::ADDHL(source) => {
                let value = self.get_source_word(source);
                let result = Instruction::add_word(self, value);
                self.regs.set_hl(result);
            }
            Instruction::ADDSP(source) => {
                let value = self.get_source_sp(source);
                let result = Instruction::add_sp(self, value);
                self.regs.set_sp(result);
            }
            Instruction::ADC(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::adc(self, value);
                self.regs.set_a(result);
            }
            Instruction::SUB(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::sub(self, value);
                self.regs.set_a(result);
            }
            Instruction::SBC(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::sbc(self, value);
                self.regs.set_a(result);
            }
            Instruction::AND(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::and(self, value);
                self.regs.set_a(result);
            }
            Instruction::OR(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::or(self, value);
                self.regs.set_a(result);
            }
            Instruction::XOR(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::xor(self, value);
                self.regs.set_a(result);
            }
            Instruction::CP(source) => {
                let value = self.get_source_byte(source);
                Instruction::cp(self, value);
            }
            Instruction::INCByte(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::inc_byte(self, value);
                self.set_target_byte(source, result);
            }
            Instruction::INCWord(source) => {
                let value = self.get_source_word(source);
                let result = Instruction::inc_word(self, value);
                self.set_target_word(source, result);
            }
            Instruction::DECByte(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::dec_byte(self, value);
                self.set_target_byte(source, result);
            }
            Instruction::DECWord(source) => {
                let value = self.get_source_word(source);
                let result = Instruction::dec_word(cpu, value);
                self.set_target_word(source, result);
            }
            Instruction::CCF => {
                Instruction::ccf(self);
            }
            Instruction::SCF => {
                Instruction::scf(self);
            }
            Instruction::RRA => {
                let result = Instruction::rra(self);
                self.regs.set_a(result);
            }
            Instruction::RLA => {
                let result = Instruction::rla(self);
                self.regs.set_a(result);
            }
            Instruction::RRCA => {
                let result = Instruction::rrca(self);
                self.regs.set_a(result);
            }
            Instruction::RLCA => {
                let result = Instruction::rlca(self);
                self.regs.set_a(result);
            }
            Instruction::CPL => {
                let result = Instruction::cpl(self);
                self.regs.set_a(result);
            }
            Instruction::BIT(bit, source) => {
                let value = self.get_source_byte(source);
                let bit_value = Self::get_source_bit(bit);
                Instruction::bit(self, bit_value, value);
            }
            Instruction::RES(bit, source) => {
                let value = self.get_source_byte(source);
                let bit_value = Self::get_source_bit(bit);
                let result = Instruction::res(bit_value, value);
                self.set_target_byte(source, result);
            }
            Instruction::SET(bit, source) => {
                let value = self.get_source_byte(source);
                let bit_value = Self::get_source_bit(bit);
                let result = Instruction::set(bit_value, value);
                self.set_target_byte(source, result);
            }
            Instruction::SRL(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::srl(self, value);
                self.set_target_byte(source, result);
            }
            Instruction::RR(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::rr(self, value);
                self.set_target_byte(source, result);
            }
            Instruction::RL(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::rl(self, value);
                self.set_target_byte(source, result);
            }
            Instruction::RRC(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::rrc(self, value);
                self.set_target_byte(source, result);
            }
            Instruction::RLC(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::rlc(self, value);
                self.set_target_byte(source, result);
            }
            Instruction::SRA(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::sra(self, value);
                self.set_target_byte(source, result);
            }
            Instruction::SLA(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::sla(self, value);
                self.set_target_byte(source, result);
            }
            Instruction::SWAP(source) => {
                let value = self.get_source_byte(source);
                let result = Instruction::swap(self, value);
                self.set_target_byte(source, result);
            }
            Instruction::JP(test) => {
                let jump_condition = self.get_source_condition(test);
                Instruction::jp(self, jump_condition)
            }
            Instruction::JR(test) => {
                let jump_condition = self.get_source_condition(test);
                Instruction::jr(self, jump_condition)
            }
            Instruction::JPHL(test) => {
                Instruction::jphl(self)
            }
            Instruction::LD(source_byte, target_byte) => {
                let source_value = self.get_source_byte(source_byte);
                self.set_target_byte(target_byte, source_value);
                match source_byte {
                    SourceByte::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1),
                }
            }
            Instruction::LDWord(source_word, target_word) => {
                let source_value = self.get_source_ld_word(source_word);
                self.set_target_word(target_word);
                self.pc.wrapping_add(3)
            }
            Instruction::LDAFromIndirect(source) => {
                let source_value = self.get_source_ld_indirect(source);
                self.regs.set_a(source_value);
                match source {
                    SourceLDIndirect::HLPlus => {
                        let value = self.regs.get_hl();
                        self.regs.set_hl(value.wrapping_add(1));
                    }
                    SourceLDIndirect::HLMinus => {
                        let value = self.regs.get_hl();
                        self.regs.set_hl(value.wrapping_sub(1));
                    }
                    _ => {}
                }
                self.pc.wrapping_add(1)
            }
            Instruction::LDIndirectFromA(target) => {
                let value = self.regs.get_a() as u16;
                self.set_target_ld_indirect(target, value);
                match target {
                    TargetLDIndirect::HLPlus => {
                        let value = self.regs.get_hl();
                        self.regs.set_hl(value.wrapping_add(1));
                    }
                    TargetLDIndirect::HLMinus => {
                        let value = self.regs.get_hl();
                        self.regs.set_hl(value.wrapping_sub(1));
                    }
                    _ => {}
                }
                self.pc.wrapping_add(1)
            }
            Instruction::LDAFromByteAddress => {
                let addr = self.bus.read_word(self.pc.wrapping_add(1));
                let value = self.bus.read_byte(addr);
                self.regs.set_a(value);

                self.pc.wrapping_add(3)
            }
            Instruction::LDByteAddressFromA => {
                let addr = self.bus.read_word(self.pc.wrapping_add(1));
                let value = self.regs.get_a();
                self.bus.write_byte(addr, value);

                self.pc.wrapping_add(3)
            }
            Instruction::PUSH(source) => {
                let value = self.get_source_stack(source);
                Instruction::push(self, value);
                
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let value = Instruction::pop(self);
                self.set_target_stack(target, value);
                
                self.pc.wrapping_add(1)
            }
            Instruction::CALL(test) => {
                let cond = self.get_source_condition(test);
                Instruction::call(cpu, cond)
            }
            Instruction::RET(test) => {
                let cond = self.get_source_condition(test);
                Instruction::ret(cpu, cond)
            }
            Instruction::NOP => {
                self.pc.wrapping_add(1)
            }
            Instruction::HALT => {
                self.halt = true;
                self.pc.wrapping_add(1)
            }
        }
    }

    // ==================================================
    // Resolve Source Operands
    // ==================================================
    fn get_source_byte(&self, source: SourceByte) -> u8 {
        let value: u8;
        match source {
            SourceByte::A => {
                value = self.regs.get_a();
            }
            SourceByte::B => {
                value = self.regs.get_b();
            }
            SourceByte::C => {
                value = self.regs.get_c();
            }
            SourceByte::D => {
                value = self.regs.get_d();
            }
            SourceByte::E => {
                value = self.regs.get_e();
            }
            SourceByte::H => {
                value = self.regs.get_h();
            }
            SourceByte::L => {
                value = self.regs.get_l();
            }
            SourceByte::D8 => {
                value = self.bus.read_byte(self.pc + 1);
            }
            SourceByte::IndirectHL => {
                value = self.bus.read_byte(self.regs.get_hl());
            }
        }
        value
    }
    
    fn get_source_word(&self, source: SourceWord) -> u16 {
        let value: u16;
        match source {
            SourceWord::BC => {
                value = self.regs.get_bc();
            }
            SourceWord::DE => {
                value = self.regs.get_de();
            }
            SourceWord::HL => {
                value = self.regs.get_hl();
            }
            SourceWord::SP => {
                value = self.regs.get_sp();
            }
        }
        value
    }

    fn get_source_sp(&self, source: SourceSP) -> i8 {
        let value: i8;
        match source {
            SourceSP::D8 => {
                value = self.bus.read_byte(self.pc + 1);
            }
        }
        value
    }

    fn get_source_bit(source: SourceBit) -> u8 {
        match source {
            SourceBit::Bit0 => 0,
            SourceBit::Bit1 => 1,
            SourceBit::Bit2 => 2,
            SourceBit::Bit3 => 3,
            SourceBit::Bit4 => 4,
            SourceBit::Bit5 => 5,
            SourceBit::Bit6 => 6,
            SourceBit::Bit7 => 7,
        }
    }

    fn get_source_condition(&self, source: SourceCondition) -> bool {
        match source {
            SourceCondition::NotZero => !self.registers.f.zero,
            SourceCondition::NotCarry => !self.registers.f.carry,
            SourceCondition::Zero => self.registers.f.zero,
            SourceCondition::Carry => self.registers.f.carry,
            SourceCondition::Always => true,
        }
    }


    // ==================================================
    // Set Target Operands
    // ==================================================
    fn set_target_byte(&mut self, target: TargetByte, value: u8) {
        match target {
            TargetByte::A => {
                self.regs.set_a(value);
            }
            TargetByte::B => {
                self.regs.set_b(value);
            }
            TargetByte::C => {
                self.regs.set_c(value);
            }
            TargetByte::D => {
                self.regs.set_d(value);
            }
            TargetByte::E => {
                self.regs.set_e(value);
            }
            TargetByte::H => {
                self.regs.set_h(value);
            }
            TargetByte::L => {
                self.regs.set_l(value);
            }
            TargetByte::IndirectHL => {
                self.bus.write_byte(self.regs.get_hl());
            }
        }
    }

    fn set_target_word(&mut self, target: TargetWord, value: u16) {
        match target {
            TargetWord::BC => {
                self.regs.set_bc(value);
            }
            TargetWord::DE => {
                self.regs.set_de(value);
            }
            TargetWord::HL => {
                self.regs.set_hl(value);
            }
            TargetWord::SP => {
                self.regs.set_sp(value);
            }
        }
    }

    
    // ==================================================
    // Resolve Misc. Source Operands
    // ==================================================
    fn get_source_ld_word(&mut self, source: SourceLDWord) -> u16 {
        match source {
            SourceLDWord::D16 => {
                self.bus.read_word(self.pc.wrapping_add(1))
            }
        }
    }

    fn get_source_stack(&mut self, source: SourceStack) -> u16 {
        match source {
            SourceStack::BC => self.regs.get_bc(),
            SourceStack::DE => self.regs.get_de(),
            SourceStack::HL => self.regs.get_hl(),
            SourceStack::AF => self.regs.get_af(),
        }
    }


    // ==================================================
    // Set Misc. Target Operands
    // ==================================================
    fn set_target_ld_indirect(&mut self, target: TargetLDIndirect, value: u16) {
        match target {
            TargetLDIndirect::BC => {
                self.regs.set_bc(value);
            }
            TargetLDIndirect::DE => {
                self.regs.set_de(value);
            }
            TargetLDIndirect::HLPlus => {
                self.regs.set_hl(value);
            }
            TargetLDIndirect::HLMinus => {
                self.regs.set_hl(value);
            }
        }

        fn set_target_stack(&mut self, target: TargetStack, value: u16) {
            match target {
                TargetStack::BC => {
                    self.regs.set_bc(value);
                }
                TargetStack::DE => {
                    self.regs.set_de(value);
                }
                TargetStack::HL => {
                    self.regs.set_hl(value);
                }
                TargetStack::AF => {
                    self.regs.set_af(value);
                }
            }
        }
    }
}