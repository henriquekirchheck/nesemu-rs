use std::fmt::format;

use crate::cpu::{addressing_mode::AddressingResult, mem::Memory, opcodes::Instruction, status::ProcessorStatus};

use super::CPU;

pub fn trace(cpu: &mut CPU) -> String {
    let code = cpu.mem_read(cpu.program_counter);
    let instruction = Instruction::from_opcode(code);
    let addressing_mode = instruction.to_opcode_info().addressing_mode.get_operand_address(cpu);

    let arguments = match addressing_mode {
        AddressingResult::ImplicitOperation | AddressingResult::AccumulatorOperation => format!("     "),
        AddressingResult::ReadAddress(x) => format!("{x:04X}").as_bytes().chunks(2).map(std::str::from_utf8).collect::<Result<Vec<&str>, _>>().unwrap().join(" "),
        AddressingResult::RelativeOffset(x) => format!("{:02X}   ", x as u8),
    };

    // TODO:
    //   - Format memory access
    //   - Discover what the fuck SP is supposed to mean
    //   - Check if P is the processor status

    format!(
        "{:04X}  {:02X} {}  {} #$01                        A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:FD",
        cpu.program_counter,
        code,
        arguments,
        instruction.to_opcode_name(),
        cpu.registers.a,
        cpu.registers.x,
        cpu.registers.y,
        <ProcessorStatus as Into<u8>>::into(cpu.status),
    )
}

#[cfg(test)]
mod test {
    use crate::{cpu::bus::Bus, nes::NesRom};

    use super::*;

    const ROM_BYTES: &[u8] = include_bytes!("../../test/nestest.nes");

    #[test]
    fn test_format_trace() {
        let mut bus = Bus::new(NesRom::parse(ROM_BYTES).unwrap().1);
        bus.mem_write(100, 0xa2);
        bus.mem_write(101, 0x01);
        bus.mem_write(102, 0xca);
        bus.mem_write(103, 0x88);
        bus.mem_write(104, 0x00);

        let mut cpu = CPU::with_bus(bus);
        cpu.program_counter = 0x64;
        cpu.registers.a = 1;
        cpu.registers.x = 2;
        cpu.registers.y = 3;
        let mut result: Vec<String> = vec![];
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu));
        });
        assert_eq!(
            "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
            result[0]
        );
        assert_eq!(
            "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
            result[1]
        );
        assert_eq!(
            "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
            result[2]
        );
    }

    #[test]
    fn test_format_mem_access() {
        let mut bus = Bus::new(NesRom::parse(ROM_BYTES).unwrap().1);

        // ORA ($33), Y
        bus.mem_write(100, 0x11);
        bus.mem_write(101, 0x33);

        //data
        bus.mem_write(0x33, 00);
        bus.mem_write(0x34, 04);

        //target cell
        bus.mem_write(0x400, 0xAA);

        let mut cpu = CPU::with_bus(bus);
        cpu.program_counter = 0x64;
        cpu.registers.y = 0;
        let mut result: Vec<String> = vec![];
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu));
        });
        assert_eq!(
            "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
            result[0]
        );
    }
}
