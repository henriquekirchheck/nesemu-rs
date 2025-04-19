use tracing::{error, warn};

use crate::nes::{Header, Mirroring, NesRom, PRG_ROM_PAGE_SIZE, Region, RomMapper};

use crate::cpu::mem::Memory;

pub struct Bus {
    cpu_vram: [u8; 2048],
    rom: NesRom,
}

impl Bus {
    pub fn new(rom: NesRom) -> Self {
        Self {
            cpu_vram: [0; 2048],
            rom,
        }
    }
}

impl Bus {
    fn read_prg_rom(&self, addr: u16) -> u8 {
        let mut addr = (addr - PRG_ROM) as usize;
        let size = self.rom.header.len_prg_rom as usize * PRG_ROM_PAGE_SIZE;
        if addr >= size {
            addr %= size;
        }
        self.rom.prg_rom[addr as usize]
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            cpu_vram: [0; 2048],
            rom: NesRom {
                header: Header {
                    len_prg_rom: 0x00,
                    len_chr_rom: 0x00,
                    rom_mapper: RomMapper::None,
                    mirroring: Mirroring::Horizontal,
                    vs_system: false,
                    trainer: false,
                    battery_backed_ram: false,
                    len_prg_ram: 0x00,
                    region: Region::NTSC,
                },
                trainer: None,
                prg_rom: vec![],
                chr_rom: vec![],
            },
        }
    }
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const PRG_ROM: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xFFFF;

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
            PRG_ROM..=PRG_ROM_END => self.read_prg_rom(addr),
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
            PRG_ROM..=PRG_ROM_END => {
                error!("Attempted to write to Cartridge Read-only Memory Space")
            }
            _ => {
                warn!("Ignoring mem write at {addr}, nothing done");
            }
        }
    }
}
