#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
}

impl Registers {
    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
    }
}
