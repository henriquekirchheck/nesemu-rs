#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ProcessorStatus {
    pub carry_flag: bool,
    pub zero_flag: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
    pub overflow_flag: bool,
    pub negative_flag: bool,
}

impl ProcessorStatus {
    pub fn update_zero_neg_flags(&mut self, value: u8) {
        self.zero_flag = value == 0;
        self.negative_flag = value & 0b1000_0000 != 0;
    }

    pub fn update_carry_zero_neg_cmp(&mut self, value1: u8, value2: u8) {
        self.carry_flag = value1 >= value2;
        self.zero_flag = value1 == value2;
        self.negative_flag = (value1 - value2) & 0b1000_0000 != 0;
    }

    pub fn update_carry_overflow_zero_neg(&mut self, value: u8, overflow: bool) {
        self.update_zero_neg_flags(value);
        self.carry_flag = overflow;
    }

    pub fn reset(&mut self) {
        self.carry_flag = false;
        self.zero_flag = false;
        self.interrupt_disable = false;
        self.decimal = false;
        self.overflow_flag = false;
        self.negative_flag = false;
    }
}

const CARRY_FLAG_OFFSET: u8 = 0;
const ZERO_FLAG_OFFSET: u8 = 1;
const INTERRUPT_DISABLE_OFFSET: u8 = 2;
const DECIMAL_OFFSET: u8 = 3;
const OVERFLOW_FLAG_OFFSET: u8 = 6;
const NEGATIVE_FLAG_OFFSET: u8 = 7;

impl From<ProcessorStatus> for u8 {
    fn from(value: ProcessorStatus) -> Self {
        (if value.negative_flag { 1 } else { 0 } << NEGATIVE_FLAG_OFFSET)
            | (if value.overflow_flag { 1 } else { 0 } << OVERFLOW_FLAG_OFFSET)
            | (1 << 5)
            | (0 << 4)
            | (if value.decimal { 1 } else { 0 } << DECIMAL_OFFSET)
            | (if value.interrupt_disable { 1 } else { 0 } << INTERRUPT_DISABLE_OFFSET)
            | (if value.zero_flag { 1 } else { 0 } << ZERO_FLAG_OFFSET)
            | (if value.carry_flag { 1 } else { 0 } << CARRY_FLAG_OFFSET)
    }
}

impl From<u8> for ProcessorStatus {
    fn from(value: u8) -> Self {
        Self {
            carry_flag: value & (1 << CARRY_FLAG_OFFSET) != 0,
            zero_flag: value & (1 << ZERO_FLAG_OFFSET) != 0,
            interrupt_disable: value & (1 << INTERRUPT_DISABLE_OFFSET) != 0,
            decimal: value & (1 << DECIMAL_OFFSET) != 0,
            overflow_flag: value & (1 << OVERFLOW_FLAG_OFFSET) != 0,
            negative_flag: value & (1 << NEGATIVE_FLAG_OFFSET) != 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::ProcessorStatus;

    #[test]
    fn test_from_eq() {
        let status = ProcessorStatus {
            carry_flag: true,
            zero_flag: true,
            interrupt_disable: true,
            decimal: true,
            overflow_flag: true,
            negative_flag: true,
        };
        let bin: u8 = status.into();
        let back: ProcessorStatus = bin.into();

        assert_eq!(status, back)
    }
}
