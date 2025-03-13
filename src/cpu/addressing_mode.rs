use super::{mem::Memory, CPU};

/**
 * If the OpCode does not support Implicit, Accumulator, or Relative addressing modes
   you should be able to call unwrap on the info you want
 */
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Relative,

    Immediate,

    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,

    Absolute,
    Absolute_X,
    Absolute_Y,

    Indirect,
    Indirect_X,
    Indirect_Y,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressingResult {
    ReadAddress(u16),
    ImplicitOperation,
    AccumulatorOperation,
    RelativeOffset(i8),
}

impl AddressingMode {
    pub fn get_operand_address(&self, cpu: &CPU) -> AddressingResult {
        AddressingResult::ReadAddress(match self {
            AddressingMode::Implicit => return AddressingResult::ImplicitOperation,
            AddressingMode::Accumulator => return AddressingResult::AccumulatorOperation,
            AddressingMode::Relative => {
                let offset = cpu.mem_read(cpu.program_counter);
                return AddressingResult::RelativeOffset(offset as i8);
            }

            AddressingMode::Immediate => cpu.program_counter,

            AddressingMode::ZeroPage => cpu.mem_read(cpu.program_counter) as u16,
            AddressingMode::ZeroPage_X => {
                let pos = cpu.mem_read(cpu.program_counter);
                let addr = pos.wrapping_add(cpu.registers.x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = cpu.mem_read(cpu.program_counter);
                let addr = pos.wrapping_add(cpu.registers.y) as u16;
                addr
            }

            AddressingMode::Absolute => cpu.mem_read_u16(cpu.program_counter),
            AddressingMode::Absolute_X => {
                let base = cpu.mem_read_u16(cpu.program_counter);
                let addr = base.wrapping_add(cpu.registers.x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = cpu.mem_read_u16(cpu.program_counter);
                let addr = base.wrapping_add(cpu.registers.y as u16);
                addr
            }

            AddressingMode::Indirect => {
                let ptr = cpu.mem_read(cpu.program_counter);
                let lo = cpu.mem_read(ptr as u16);
                let hi = cpu.mem_read(ptr.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::Indirect_X => {
                let base = cpu.mem_read(cpu.program_counter);

                let ptr = base.wrapping_add(cpu.registers.x);
                let lo = cpu.mem_read(ptr as u16);
                let hi = cpu.mem_read(ptr.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::Indirect_Y => {
                let base = cpu.mem_read(cpu.program_counter);

                let lo = cpu.mem_read(base as u16);
                let hi = cpu.mem_read(base.wrapping_add(1) as u16);
                let deref_base = u16::from_le_bytes([lo, hi]);
                let deref = deref_base.wrapping_add(cpu.registers.y as u16);
                deref
            }
        })
    }
}

impl AddressingResult {
    pub const fn unwrap_read_address(self) -> u16 {
        match self {
            Self::ReadAddress(val) => val,
            _ => panic!("called `AddressingResult::unwrap_read_address()` on a non `AddressingResult::ReadAddress` value"),
        }
    }
    pub const fn unwrap_relative_offset(self) -> i8 {
        match self {
            Self::RelativeOffset(val) => val,
            _ => panic!("called `AddressingResult::unwrap_relative_offset()` on a non `AddressingResult::RelativeOffset` value"),
        }
    }
}
