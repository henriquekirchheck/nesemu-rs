use tracing::warn;

use super::mem::Memory;

pub struct Bus {
    cpu_vram: [u8; 2048],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu_vram: [0; 2048],
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

impl Memory for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not supported yet")
            }
            _ => {
                warn!("Ignoring mem access at {addr}, returning 0");
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[mirror_down_addr as usize] = data
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0b00100000_00000111;
                todo!("PPU is not supported yet")
            }
            _ => {
                warn!("Ignoring mem write at {addr}, nothing done");
            }
        }
    }
}
