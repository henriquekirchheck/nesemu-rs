mod addressing_mode;
pub mod mem;
mod opcodes;
mod registers;
mod status;

use addressing_mode::AddressingMode;
use mem::{Memory, Stack, STACK, STACK_RESET};
use registers::Registers;
use status::ProcessorStatus;

use self::opcodes::{Instruction, OpCodeInfo};

pub struct CPU {
    program_counter: u16,
    registers: Registers,
    status: ProcessorStatus,
    memory: [u8; 0xFFFF],
    stack_pointer: u8,
}

impl Memory for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl Stack for CPU {
    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(STACK + self.stack_pointer as u16)
    }

    fn stack_push(&mut self, data: u8) {
        self.mem_write(STACK + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            stack_pointer: STACK_RESET,
            registers: Registers::default(),
            status: ProcessorStatus::default(),
            memory: [0; 0xFFFF],
        }
    }

    pub fn reset(&mut self) {
        self.registers.reset();
        self.status.reset();
        self.stack_pointer = STACK_RESET;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: &[u8]) {
        self.memory[0x0600..(0x0600 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xFFFC, 0x0600)
    }

    pub fn load_and_run(&mut self, program: &[u8]) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load_modify_and_run<Func>(&mut self, program: &[u8], modify: Func)
    where
        Func: Fn(&mut Self),
    {
        self.load(program);
        self.reset();
        modify(self);
        self.run();
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback(&mut self, mut callback: impl FnMut(&mut CPU)) {
        loop {
            callback(self);
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let instruction = Instruction::from_opcode(code);

            let advance = match instruction {
                Instruction::ADC(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.adc(addressing_mode);
                    bytes
                }
                Instruction::AND(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.and(addressing_mode);
                    bytes
                }
                Instruction::ASL(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.asl(addressing_mode);
                    bytes
                }
                Instruction::BCC(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.branch(
                        !self.status.carry_flag,
                        addressing_mode
                            .get_operand_address(self)
                            .unwrap_relative_offset(),
                    );
                    bytes
                }
                Instruction::BCS(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.branch(
                        self.status.carry_flag,
                        addressing_mode
                            .get_operand_address(self)
                            .unwrap_relative_offset(),
                    );
                    bytes
                }
                Instruction::BEQ(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.branch(
                        self.status.zero_flag,
                        addressing_mode
                            .get_operand_address(self)
                            .unwrap_relative_offset(),
                    );
                    bytes
                }
                Instruction::BIT(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.bit(addressing_mode);
                    bytes
                }
                Instruction::BMI(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.branch(
                        self.status.negative_flag,
                        addressing_mode
                            .get_operand_address(self)
                            .unwrap_relative_offset(),
                    );
                    bytes
                }
                Instruction::BNE(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.branch(
                        !self.status.zero_flag,
                        addressing_mode
                            .get_operand_address(self)
                            .unwrap_relative_offset(),
                    );
                    bytes
                }
                Instruction::BPL(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.branch(
                        !self.status.negative_flag,
                        addressing_mode
                            .get_operand_address(self)
                            .unwrap_relative_offset(),
                    );
                    bytes
                }
                Instruction::BRK(_) => {
                    return;
                }
                Instruction::BVC(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.branch(
                        !self.status.overflow_flag,
                        addressing_mode
                            .get_operand_address(self)
                            .unwrap_relative_offset(),
                    );
                    bytes
                }
                Instruction::BVS(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.branch(
                        self.status.overflow_flag,
                        addressing_mode
                            .get_operand_address(self)
                            .unwrap_relative_offset(),
                    );
                    bytes
                }
                Instruction::CLC(OpCodeInfo { bytes, .. }) => {
                    self.status.carry_flag = false;
                    bytes
                }
                Instruction::CLD(OpCodeInfo { bytes, .. }) => {
                    self.status.decimal = false;
                    bytes
                }
                Instruction::CLI(OpCodeInfo { bytes, .. }) => {
                    self.status.interrupt_disable = false;
                    bytes
                }
                Instruction::CLV(OpCodeInfo { bytes, .. }) => {
                    self.status.overflow_flag = false;
                    bytes
                }
                Instruction::CMP(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.cmp(addressing_mode);
                    bytes
                }
                Instruction::CPX(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.cpx(addressing_mode);
                    bytes
                }
                Instruction::CPY(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.cpy(addressing_mode);
                    bytes
                }
                Instruction::DEC(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.dec(addressing_mode);
                    bytes
                }
                Instruction::DEX(OpCodeInfo { bytes, .. }) => {
                    self.dex();
                    bytes
                }
                Instruction::DEY(OpCodeInfo { bytes, .. }) => {
                    self.dey();
                    bytes
                }
                Instruction::EOR(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.eor(addressing_mode);
                    bytes
                }
                Instruction::INC(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.inc(addressing_mode);
                    bytes
                }
                Instruction::INX(OpCodeInfo { bytes, .. }) => {
                    self.inx();
                    bytes
                }
                Instruction::INY(OpCodeInfo { bytes, .. }) => {
                    self.iny();
                    bytes
                }
                Instruction::JMP(OpCodeInfo {
                    ref addressing_mode,
                    ..
                }) => {
                    self.jmp(addressing_mode);
                    1
                }
                Instruction::JSR(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.jsr(addressing_mode, bytes);
                    1
                }
                Instruction::LDA(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.lda(addressing_mode);
                    bytes
                }
                Instruction::LDX(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.ldx(addressing_mode);
                    bytes
                }
                Instruction::LDY(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.ldy(addressing_mode);
                    bytes
                }
                Instruction::LSR(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.lsr(addressing_mode);
                    bytes
                }
                Instruction::NOP(OpCodeInfo { bytes, .. }) => bytes,
                Instruction::ORA(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.ora(addressing_mode);
                    bytes
                }
                Instruction::PHA(OpCodeInfo { bytes, .. }) => {
                    self.pha();
                    bytes
                }
                Instruction::PHP(OpCodeInfo { bytes, .. }) => {
                    self.php();
                    bytes
                }
                Instruction::PLA(OpCodeInfo { bytes, .. }) => {
                    self.pla();
                    bytes
                }
                Instruction::PLP(OpCodeInfo { bytes, .. }) => {
                    self.plp();
                    bytes
                }
                Instruction::ROL(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.rol(addressing_mode);
                    bytes
                }
                Instruction::ROR(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.rol(addressing_mode);
                    bytes
                }
                Instruction::RTI(_) => {
                    self.plp();
                    self.program_counter = self.stack_pop_u16();
                    1
                }
                Instruction::RTS(_) => {
                    self.program_counter = self.stack_pop_u16() - 1;
                    1
                }
                Instruction::SBC(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.sbc(addressing_mode);
                    bytes
                }
                Instruction::SEC(OpCodeInfo { bytes, .. }) => {
                    self.status.carry_flag = true;
                    bytes
                }
                Instruction::SED(OpCodeInfo { bytes, .. }) => {
                    self.status.decimal = true;
                    bytes
                }
                Instruction::SEI(OpCodeInfo { bytes, .. }) => {
                    self.status.interrupt_disable = true;
                    bytes
                }
                Instruction::STA(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.sta(addressing_mode);
                    bytes
                }
                Instruction::STX(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.stx(addressing_mode);
                    bytes
                }
                Instruction::STY(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.sty(addressing_mode);
                    bytes
                }
                Instruction::TAX(OpCodeInfo { bytes, .. }) => {
                    self.tax();
                    bytes
                }
                Instruction::TAY(OpCodeInfo { bytes, .. }) => {
                    self.tay();
                    bytes
                }
                Instruction::TSX(OpCodeInfo { bytes, .. }) => {
                    self.tsx();
                    bytes
                }
                Instruction::TXA(OpCodeInfo { bytes, .. }) => {
                    self.txa();
                    bytes
                }
                Instruction::TXS(OpCodeInfo { bytes, .. }) => {
                    self.txs();
                    bytes
                }
                Instruction::TYA(OpCodeInfo { bytes, .. }) => {
                    self.tya();
                    bytes
                }
            };

            self.program_counter += (advance - 1) as u16
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        let (result, overflow) = self.registers.a.overflowing_add(value);
        self.registers.a = result + self.status.carry_flag as u8;
        self.status
            .update_carry_overflow_zero_neg(self.registers.a, overflow);
    }
    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        let (result, overflow) = self.registers.a.overflowing_sub(value);
        self.registers.a = result - !self.status.carry_flag as u8;
        self.status
            .update_carry_overflow_zero_neg(self.registers.a, overflow);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        self.registers.a = self.registers.a & value;
        self.status.update_zero_neg_flags(self.registers.a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr_res = mode.get_operand_address(self);
        let (result, overflow) = match addr_res {
            addressing_mode::AddressingResult::AccumulatorOperation => {
                let (result, overflow) = self.registers.a.overflowing_shl(1);
                self.registers.a = result;
                (result, overflow)
            }
            addressing_mode::AddressingResult::ReadAddress(addr) => {
                let (result, overflow) = self.mem_read(addr).overflowing_shl(1);
                self.mem_write(addr, result);
                (result, overflow)
            }
            x => panic!("asl does not support {x:?}"),
        };
        self.status.update_carry_overflow_zero_neg(result, overflow);
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let addr_res = mode.get_operand_address(self);
        let (old_bit7, new_bit7) = match addr_res {
            addressing_mode::AddressingResult::AccumulatorOperation => {
                let old_bit7 = self.registers.a & 0b1000_0000 != 0;
                let result = (self.registers.a << 1) | self.status.carry_flag as u8;
                self.registers.a = result;
                let new_bit7 = result & 0b1000_0000 != 0;
                (old_bit7, new_bit7)
            }
            addressing_mode::AddressingResult::ReadAddress(addr) => {
                let value = self.mem_read(addr);
                let old_bit7 = value & 0b1000_0000 != 0;
                let result = (value << 1) | self.status.carry_flag as u8;
                self.mem_write(addr, result);
                let new_bit7 = result & 0b1000_0000 != 0;
                (old_bit7, new_bit7)
            }
            x => panic!("rol does not support {x:?}"),
        };
        self.status.zero_flag = self.registers.a == 0;
        self.status.negative_flag = new_bit7;
        self.status.carry_flag = old_bit7;
    }
    fn ror(&mut self, mode: &AddressingMode) {
        let addr_res = mode.get_operand_address(self);
        let (old_bit0, new_bit7) = match addr_res {
            addressing_mode::AddressingResult::AccumulatorOperation => {
                let old_bit0 = self.registers.a & 1 != 0;
                let result = (self.registers.a >> 1) | ((self.status.carry_flag as u8) << 7);
                self.registers.a = result;
                let new_bit7 = result & 0b1000_0000 != 0;
                (old_bit0, new_bit7)
            }
            addressing_mode::AddressingResult::ReadAddress(addr) => {
                let value = self.mem_read(addr);
                let old_bit0 = value & 1 != 0;
                let result = (value >> 1) | ((self.status.carry_flag as u8) << 7);
                self.mem_write(addr, result);
                let new_bit7 = result & 0b1000_0000 != 0;
                (old_bit0, new_bit7)
            }
            x => panic!("ror does not support {x:?}"),
        };
        self.status.zero_flag = self.registers.a == 0;
        self.status.negative_flag = new_bit7;
        self.status.carry_flag = old_bit0;
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let addr_res = mode.get_operand_address(self);
        let (result, carry) = match addr_res {
            addressing_mode::AddressingResult::AccumulatorOperation => {
                let carry = self.registers.a & 1 == 1;
                let result = self.registers.a >> 1;
                self.registers.a = result;
                (result, carry)
            }
            addressing_mode::AddressingResult::ReadAddress(addr) => {
                let value = self.mem_read(addr);
                let carry = value & 1 == 1;
                let result = value >> 1;
                self.mem_write(addr, result);
                (result, carry)
            }
            x => panic!("lsr does not support {x:?}"),
        };
        self.status.update_zero_neg_flags(result);
        self.status.carry_flag = carry;
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        let result = self.registers.a & value;
        self.status.zero_flag = result == 0;
        self.status.negative_flag = (value & 0b1000_0000) != 0;
        self.status.overflow_flag = (value & 0b0100_0000) != 0;
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        let result = value.wrapping_sub(1);
        self.mem_write(addr, result);
        self.status.update_zero_neg_flags(result);
    }

    fn dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.status.update_zero_neg_flags(self.registers.x);
    }

    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.status.update_zero_neg_flags(self.registers.y);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        self.registers.a = self.registers.a ^ value;
        self.status.update_zero_neg_flags(self.registers.a)
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        let result = value.wrapping_add(1);
        self.mem_write(addr, result);
        self.status.update_zero_neg_flags(result);
    }

    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.status.update_zero_neg_flags(self.registers.x);
    }

    fn iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.status.update_zero_neg_flags(self.registers.y);
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        let mem_address = AddressingMode::Absolute
            .get_operand_address(self)
            .unwrap_read_address();
        match mode {
            AddressingMode::Absolute => self.program_counter = mem_address,
            AddressingMode::Indirect => {
                self.program_counter = if mem_address & 0x00FF == 0x00FF {
                    let lo = self.mem_read(mem_address);
                    let hi = self.mem_read(mem_address & 0xFF00);
                    (hi as u16) << 8 | (lo as u16)
                } else {
                    self.mem_read_u16(mem_address)
                };
            }
            x => panic!("Invalid addressing mode for jmp: {x:?}"),
        }
    }

    fn jsr(&mut self, mode: &AddressingMode, bytes: u8) {
        self.stack_push_u16(self.program_counter + (bytes as u16 - 1) - 1);
        self.program_counter = mode.get_operand_address(self).unwrap_read_address();
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);

        self.registers.a = value;
        self.status.update_zero_neg_flags(self.registers.a);
    }
    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);

        self.registers.x = value;
        self.status.update_zero_neg_flags(self.registers.x);
    }
    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);

        self.registers.y = value;
        self.status.update_zero_neg_flags(self.registers.y);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        self.registers.a = self.registers.a | value;
        self.status.update_zero_neg_flags(self.registers.a);
    }

    fn pha(&mut self) {
        self.stack_push(self.registers.a);
    }

    fn php(&mut self) {
        self.stack_push(self.status.into());
    }

    fn pla(&mut self) {
        self.registers.a = self.stack_pop();
        self.status.update_zero_neg_flags(self.registers.a);
    }

    fn plp(&mut self) {
        self.status = self.stack_pop().into();
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        self.mem_write(addr, self.registers.a)
    }
    fn stx(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        self.mem_write(addr, self.registers.x)
    }
    fn sty(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        self.mem_write(addr, self.registers.y)
    }

    fn cmp(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        self.status
            .update_carry_zero_neg_cmp(self.registers.a, value);
    }

    fn cpx(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        self.status
            .update_carry_zero_neg_cmp(self.registers.x, value);
    }

    fn cpy(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);
        self.status
            .update_carry_zero_neg_cmp(self.registers.y, value);
    }

    fn tax(&mut self) {
        self.registers.x = self.registers.a;
        self.status.update_zero_neg_flags(self.registers.x)
    }
    fn tay(&mut self) {
        self.registers.y = self.registers.a;
        self.status.update_zero_neg_flags(self.registers.y)
    }
    fn tsx(&mut self) {
        self.registers.x = self.stack_pointer;
        self.status.update_zero_neg_flags(self.registers.x)
    }
    fn txa(&mut self) {
        self.registers.a = self.registers.x;
        self.status.update_zero_neg_flags(self.registers.a)
    }
    fn txs(&mut self) {
        self.stack_pointer = self.registers.x;
    }
    fn tya(&mut self) {
        self.registers.a = self.registers.y;
        self.status.update_zero_neg_flags(self.registers.a)
    }

    fn branch(&mut self, condition: bool, offset: i8) {
        if condition {
            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(offset as u16);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&[0xa9, 0x05, 0x00]);
        assert_eq!(cpu.registers.a, 0x05);
        assert!(<ProcessorStatus as Into<u8>>::into(cpu.status) & 0b0000_0010 == 0b00);
        assert!(<ProcessorStatus as Into<u8>>::into(cpu.status) & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&[0xa9, 0x00, 0x00]);
        assert!(<ProcessorStatus as Into<u8>>::into(cpu.status) & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_modify_and_run(&[0xaa, 0x00], |cpu| cpu.registers.a = 10);

        assert_eq!(cpu.registers.x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(&[0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.registers.x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_modify_and_run(&[0xe8, 0xe8, 0x00], |cpu| cpu.registers.x = 0xff);

        assert_eq!(cpu.registers.x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(&[0xa5, 0x10, 0x00]);

        assert_eq!(cpu.registers.a, 0x55);
    }
}
