mod addressing_mode;
mod mem;
mod opcodes;
mod registers;
mod status;

use addressing_mode::AddressingMode;
use mem::Memory;
use registers::Registers;
use status::ProcessorStatus;

use self::opcodes::{Instruction, OpCodeInfo};

pub struct CPU {
    program_counter: u16,
    registers: Registers,
    status: ProcessorStatus,
    memory: [u8; 0xFFFF],
}

impl Memory for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            program_counter: 0,
            registers: Registers::default(),
            status: ProcessorStatus::default(),
            memory: [0; 0xFFFF],
        }
    }

    pub fn reset(&mut self) {
        self.registers.reset();
        self.status.reset();
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: &[u8]) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xFFFC, 0x8000)
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
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let instruction = Instruction::from_opcode(code);

            match instruction {
                Instruction::BRK(_) => return,
                Instruction::TAX(_) => self.tax(),
                Instruction::INX(_) => self.inx(),
                Instruction::LDA(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.lda(addressing_mode);
                    self.program_counter += (bytes - 1) as u16;
                }
                Instruction::STA(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.sta(addressing_mode);
                    self.program_counter += (bytes - 1) as u16;
                }
                Instruction::CMP(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.cpx(addressing_mode);
                    self.program_counter += (bytes - 1) as u16;
                }
                Instruction::CPX(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.cpx(addressing_mode);
                    self.program_counter += (bytes - 1) as u16;
                }
                Instruction::CPY(OpCodeInfo {
                    ref addressing_mode,
                    bytes,
                    ..
                }) => {
                    self.cpy(addressing_mode);
                    self.program_counter += (bytes - 1) as u16;
                }

                // _ => todo!()
            }
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        let value = self.mem_read(addr);

        self.registers.a = value;
        self.status.update_zero_neg_flags(self.registers.a);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = mode.get_operand_address(self).unwrap_read_address();
        self.mem_write(addr, self.registers.a)
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

    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.status.update_zero_neg_flags(self.registers.x)
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
