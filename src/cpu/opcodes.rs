#![deny(unreachable_patterns)]

use super::addressing_mode::AddressingMode;

pub struct OpCodeInfo {
    pub addressing_mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

macro_rules! create_opcodes {
    ($( $instruction:ident => [$( { opcode: $opcode:expr, addressing_mode: $addressing_mode:ident, bytes: $bytes:expr, cycles: $cycles:expr } ),+ $(,)?]);+;) => {

        pub enum Instruction {
            $($instruction(OpCodeInfo),)+
        }

        impl Instruction {
            pub fn from_opcode(opcode: u8) -> Self {
                match opcode {
                    $($($opcode =>  Self::$instruction(OpCodeInfo { addressing_mode: AddressingMode::$addressing_mode, bytes: $bytes, cycles: $cycles }),)+)+
                    _ => panic!("Not Implemented {:x}", opcode)
                }
            }
        }
    };
}

create_opcodes!(
    ADC => [
        { opcode: 0x69, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0x65, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0x75, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0x6D, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0x7D, addressing_mode: Absolute_X, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0x79, addressing_mode: Absolute_Y, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0x61, addressing_mode: Indirect_X, bytes: 2, cycles: 6 },
        { opcode: 0x71, addressing_mode: Indirect_Y, bytes: 2, cycles: 5 /* +1 if page crossed */ },
    ];
    AND => [
        { opcode: 0x29, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0x25, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0x35, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0x2D, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0x3D, addressing_mode: Absolute_X, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0x39, addressing_mode: Absolute_Y, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0x21, addressing_mode: Indirect_X, bytes: 2, cycles: 6 },
        { opcode: 0x31, addressing_mode: Indirect_Y, bytes: 2, cycles: 5 /* +1 if page crossed */ },
    ];
    ASL => [
        { opcode: 0x0A, addressing_mode: Accumulator, bytes: 1, cycles: 2 },
        { opcode: 0x06, addressing_mode: ZeroPage, bytes: 2, cycles: 5 },
        { opcode: 0x16, addressing_mode: ZeroPage_X, bytes: 2, cycles: 6 },
        { opcode: 0x0E, addressing_mode: Absolute, bytes: 3, cycles: 6 },
        { opcode: 0x1E, addressing_mode: Absolute_X, bytes: 3, cycles: 7 },
    ];

    BCC => [{ opcode: 0x90, addressing_mode: Relative, bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },];
    BCS => [{ opcode: 0xB0, addressing_mode: Relative, bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },];
    BEQ => [{ opcode: 0xF0, addressing_mode: Relative, bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },];

    BIT => [
        { opcode: 0x24, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0x2C, addressing_mode: Absolute, bytes: 3, cycles: 4 },
    ];
// 
    BRK => [{ opcode: 0x00, addressing_mode: Implicit, bytes: 1, cycles: 7 }];
    TAX => [{ opcode: 0xAA, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    INX => [{ opcode: 0xE8, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    LDA => [
        { opcode: 0xA9, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0xA5, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0xB5, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0xAD, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0xBD, addressing_mode: Absolute_X, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0xB9, addressing_mode: Absolute_Y, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0xA1, addressing_mode: Indirect_X, bytes: 2, cycles: 6 },
        { opcode: 0xB1, addressing_mode: Indirect_Y, bytes: 2, cycles: 5 },
    ];
    STA => [
        { opcode: 0x85, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0x95, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0x8D, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0x9D, addressing_mode: Absolute_X, bytes: 3, cycles: 5 },
        { opcode: 0x99, addressing_mode: Absolute_Y, bytes: 3, cycles: 5 },
        { opcode: 0x81, addressing_mode: Indirect_X, bytes: 2, cycles: 6 },
        { opcode: 0x91, addressing_mode: Indirect_Y, bytes: 2, cycles: 6 },
    ];
    CMP => [
        { opcode: 0xC9, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0xC5, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0xD5, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0xCD, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0xDD, addressing_mode: Absolute, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0xD9, addressing_mode: Absolute, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0xC1, addressing_mode: Indirect_X, bytes: 2, cycles: 6 },
        { opcode: 0xD1, addressing_mode: Indirect_Y, bytes: 2, cycles: 5 /* +1 if page crossed */ },
    ];
    CPX => [
        { opcode: 0xE0, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0xE4, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0xEC, addressing_mode: Absolute, bytes: 3, cycles: 4 },
    ];
    CPY => [
        { opcode: 0xC0, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0xC4, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0xCC, addressing_mode: Absolute, bytes: 3, cycles: 4 },
    ];
);
