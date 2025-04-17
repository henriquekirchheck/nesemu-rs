pub trait Memory {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr);
        let hi = self.mem_read(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        for (i, byte) in data.to_le_bytes().into_iter().enumerate() {
            self.mem_write(addr + i as u16, byte);
        }
    }
}

pub const STACK: u16 = 0x0100;
pub const STACK_RESET: u8 = 0xFD;
pub trait Stack: Memory {
    fn stack_pop(&mut self) -> u8;
    fn stack_push(&mut self, data: u8);

    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop();
        let hi = self.stack_pop();

        u16::from_le_bytes([lo, hi])
    }
    fn stack_push_u16(&mut self, data: u16) {
        let le = data.to_le_bytes();
        self.stack_push(le[1]);
        self.stack_push(le[0]);
    }
}
