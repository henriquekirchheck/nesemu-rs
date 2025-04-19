#![deny(unreachable_patterns)]

use super::addressing_mode::AddressingMode;

#[derive(Debug, Clone)]
pub struct OpCodeInfo {
    pub addressing_mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

macro_rules! create_opcodes {
    ($( $instruction:ident => [$( { opcode: $opcode:expr, addressing_mode: $addressing_mode:ident, bytes: $bytes:expr, cycles: $cycles:expr } ),+ $(,)?]);+;) => {
        #[derive(Debug, Clone)]
        pub enum Instruction {
            $($instruction(OpCodeInfo),)+
        }

        impl Instruction {
            pub fn from_opcode(opcode: u8) -> Self {
                match opcode {
                    $($($opcode => Self::$instruction(OpCodeInfo { addressing_mode: AddressingMode::$addressing_mode, bytes: $bytes, cycles: $cycles }),)+)+
                    _ => panic!("Not Implemented {:x}", opcode)
                }
            }

            pub fn to_opcode(&self) -> u8 {
                #[allow(unreachable_patterns)]
                match self {
                    $($(Self::$instruction(OpCodeInfo { addressing_mode: AddressingMode::$addressing_mode, bytes: $bytes, cycles: $cycles }) => $opcode,)+)+
                    _ => unreachable!("This can only be called from valid contructed Instructions, this is the invalid instruction: {self:?}")
                }
            }

            pub fn to_opcode_name(&self) -> &'static str {
                #[allow(unreachable_patterns)]
                match self {
                    $($(Self::$instruction(OpCodeInfo { addressing_mode: AddressingMode::$addressing_mode, bytes: $bytes, cycles: $cycles }) => stringify!($instruction),)+)+
                    _ => unreachable!("This can only be called from valid contructed Instructions, this is the invalid instruction: {self:?}")
                }
            }

            pub fn to_opcode_info(&self) -> &OpCodeInfo {
                #[allow(unreachable_patterns)]
                match self {
                    $(Self::$instruction(x) => x,)+
                    _ => unreachable!("This can only be called from valid contructed Instructions, this is the invalid instruction: {self:?}")
                }
            }
        }

        impl From<u8> for Instruction {
            fn from(value: u8) -> Self {
                Self::from_opcode(value)
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

    BMI => [{ opcode: 0x30, addressing_mode: Relative, bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },];
    BNE => [{ opcode: 0xD0, addressing_mode: Relative, bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },];
    BPL => [{ opcode: 0x10, addressing_mode: Relative, bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },];
    BRK => [{ opcode: 0x00, addressing_mode: Implicit, bytes: 1, cycles: 7 }];
    BVC => [{ opcode: 0x50, addressing_mode: Relative, bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },];
    BVS => [{ opcode: 0x70, addressing_mode: Relative, bytes: 2, cycles: 2 /* +1 if branch succeeds, +2 if to a new page */ },];
    CLC => [{ opcode: 0x18, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    CLD => [{ opcode: 0xD8, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    CLI => [{ opcode: 0x58, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    CLV => [{ opcode: 0xB8, addressing_mode: Implicit, bytes: 1, cycles: 2 }];

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

    DEC => [
        { opcode: 0xC6, addressing_mode: ZeroPage, bytes: 2, cycles: 5 },
        { opcode: 0xD6, addressing_mode: ZeroPage_X, bytes: 2, cycles: 6 },
        { opcode: 0xCE, addressing_mode: Absolute, bytes: 3, cycles: 6 },
        { opcode: 0xDE, addressing_mode: Absolute_X, bytes: 3, cycles: 7 },
    ];
    DEX => [{ opcode: 0xCA, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    DEY => [{ opcode: 0x88, addressing_mode: Implicit, bytes: 1, cycles: 2 }];

    EOR => [
        { opcode: 0x49, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0x45, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0x55, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0x4D, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0x5D, addressing_mode: Absolute_X, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0x59, addressing_mode: Absolute_Y, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0x41, addressing_mode: Indirect_X, bytes: 2, cycles: 6 },
        { opcode: 0x51, addressing_mode: Indirect_Y, bytes: 2, cycles: 5 /* +1 if page crossed */ },
    ];

    INC => [
        { opcode: 0xE6, addressing_mode: ZeroPage, bytes: 2, cycles: 5 },
        { opcode: 0xF6, addressing_mode: ZeroPage_X, bytes: 2, cycles: 6 },
        { opcode: 0xEE, addressing_mode: Absolute, bytes: 3, cycles: 6 },
        { opcode: 0xFE, addressing_mode: Absolute_X, bytes: 3, cycles: 7 },
    ];
    INX => [{ opcode: 0xE8, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    INY => [{ opcode: 0xC8, addressing_mode: Implicit, bytes: 1, cycles: 2 }];

    JMP => [
        { opcode: 0x4C, addressing_mode: Absolute, bytes: 3, cycles: 3 },
        { opcode: 0x6C, addressing_mode: Indirect, bytes: 3, cycles: 5 },
    ];
    JSR => [{ opcode: 0x20, addressing_mode: Absolute, bytes: 3, cycles: 6 }];

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
    LDX => [
        { opcode: 0xA2, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0xA6, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0xB6, addressing_mode: ZeroPage_Y, bytes: 2, cycles: 4 },
        { opcode: 0xAE, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0xBE, addressing_mode: Absolute_Y, bytes: 3, cycles: 4 /* +1 if page crossed */ },
    ];
    LDY => [
        { opcode: 0xA0, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0xA4, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0xB4, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0xAC, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0xBC, addressing_mode: Absolute_X, bytes: 3, cycles: 4 /* +1 if page crossed */ },
    ];

    LSR => [
        { opcode: 0x4A, addressing_mode: Accumulator, bytes: 1, cycles: 2 },
        { opcode: 0x46, addressing_mode: ZeroPage, bytes: 2, cycles: 5 },
        { opcode: 0x56, addressing_mode: ZeroPage_X, bytes: 2, cycles: 6 },
        { opcode: 0x4E, addressing_mode: Absolute, bytes: 3, cycles: 6 },
        { opcode: 0x5E, addressing_mode: Absolute_X, bytes: 3, cycles: 7 },
    ];

    NOP => [{ opcode: 0xEA, addressing_mode: Implicit, bytes: 1, cycles: 2 }];

    ORA => [
        { opcode: 0x09, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0x05, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0x15, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0x0D, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0x1D, addressing_mode: Absolute_X, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0x19, addressing_mode: Absolute_Y, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0x01, addressing_mode: Indirect_X, bytes: 2, cycles: 6 },
        { opcode: 0x11, addressing_mode: Indirect_Y, bytes: 2, cycles: 5 /* +1 if page crossed */ },
    ];

    PHA => [{ opcode: 0x48, addressing_mode: Implicit, bytes: 1, cycles: 3 }];
    PHP => [{ opcode: 0x08, addressing_mode: Implicit, bytes: 1, cycles: 3 }];
    PLA => [{ opcode: 0x68, addressing_mode: Implicit, bytes: 1, cycles: 4 }];
    PLP => [{ opcode: 0x28, addressing_mode: Implicit, bytes: 1, cycles: 4 }];

    ROL => [
        { opcode: 0x2A, addressing_mode: Accumulator, bytes: 1, cycles: 2 },
        { opcode: 0x26, addressing_mode: ZeroPage, bytes: 2, cycles: 5 },
        { opcode: 0x36, addressing_mode: ZeroPage_X, bytes: 2, cycles: 6 },
        { opcode: 0x2E, addressing_mode: Absolute, bytes: 3, cycles: 6 },
        { opcode: 0x3E, addressing_mode: Absolute_X, bytes: 3, cycles: 7 },
    ];
    ROR => [
        { opcode: 0x6A, addressing_mode: Accumulator, bytes: 1, cycles: 2 },
        { opcode: 0x66, addressing_mode: ZeroPage, bytes: 2, cycles: 5 },
        { opcode: 0x76, addressing_mode: ZeroPage_X, bytes: 2, cycles: 6 },
        { opcode: 0x6E, addressing_mode: Absolute, bytes: 3, cycles: 6 },
        { opcode: 0x7E, addressing_mode: Absolute_X, bytes: 3, cycles: 7 },
    ];

    RTI => [{ opcode: 0x40, addressing_mode: Implicit, bytes: 1, cycles: 6 }];
    RTS => [{ opcode: 0x60, addressing_mode: Implicit, bytes: 1, cycles: 6 }];

    SBC => [
        { opcode: 0xE9, addressing_mode: Immediate, bytes: 2, cycles: 2 },
        { opcode: 0xE5, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0xF5, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0xED, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0xFD, addressing_mode: Absolute_X, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0xF9, addressing_mode: Absolute_Y, bytes: 3, cycles: 4 /* +1 if page crossed */ },
        { opcode: 0xE1, addressing_mode: Indirect_X, bytes: 2, cycles: 6 },
        { opcode: 0xF1, addressing_mode: Indirect_Y, bytes: 2, cycles: 5 /* +1 if page crossed */ },
    ];

    SEC => [{ opcode: 0x38, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    SED => [{ opcode: 0xF8, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    SEI => [{ opcode: 0x78, addressing_mode: Implicit, bytes: 1, cycles: 2 }];

    STA => [
        { opcode: 0x85, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0x95, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0x8D, addressing_mode: Absolute, bytes: 3, cycles: 4 },
        { opcode: 0x9D, addressing_mode: Absolute_X, bytes: 3, cycles: 5 },
        { opcode: 0x99, addressing_mode: Absolute_Y, bytes: 3, cycles: 5 },
        { opcode: 0x81, addressing_mode: Indirect_X, bytes: 2, cycles: 6 },
        { opcode: 0x91, addressing_mode: Indirect_Y, bytes: 2, cycles: 6 },
    ];
    STX => [
        { opcode: 0x86, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0x96, addressing_mode: ZeroPage_Y, bytes: 2, cycles: 4 },
        { opcode: 0x8E, addressing_mode: Absolute, bytes: 3, cycles: 4 },
    ];
    STY => [
        { opcode: 0x84, addressing_mode: ZeroPage, bytes: 2, cycles: 3 },
        { opcode: 0x94, addressing_mode: ZeroPage_X, bytes: 2, cycles: 4 },
        { opcode: 0x8C, addressing_mode: Absolute, bytes: 3, cycles: 4 },
    ];

    TAX => [{ opcode: 0xAA, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    TAY => [{ opcode: 0xA8, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    TSX => [{ opcode: 0xBA, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    TXA => [{ opcode: 0x8A, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    TXS => [{ opcode: 0x9A, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
    TYA => [{ opcode: 0x98, addressing_mode: Implicit, bytes: 1, cycles: 2 }];
);
