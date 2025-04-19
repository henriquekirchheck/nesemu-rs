use nom::{
    IResult, Parser,
    bits::{
        bits,
        complete::{bool, tag as bit_tag, take as bit_take},
    },
    bytes::complete::{tag, take},
    combinator::cond,
    error::{Error, ErrorKind},
    number::u8,
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const PRG_ROM_PAGE_SIZE: usize = 0x4000;
const CHR_ROM_PAGE_SIZE: usize = 0x2000;

#[derive(Debug, PartialEq)]
pub struct NesRom {
    pub header: Header,
    pub trainer: Option<Vec<u8>>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl NesRom {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, header) = Header::parse(input)?;
        let (input, trainer) = cond(header.trainer, take(512usize))
            .map(|x| x.map(ToOwned::to_owned))
            .parse(input)?;
        let (input, prg_rom) = take(header.len_prg_rom as usize * PRG_ROM_PAGE_SIZE)
            .map(ToOwned::to_owned)
            .parse(input)?;
        let (input, chr_rom) = take(header.len_chr_rom as usize * CHR_ROM_PAGE_SIZE)
            .map(ToOwned::to_owned)
            .parse(input)?;

        Ok((
            input,
            Self {
                header,
                chr_rom,
                prg_rom,
                trainer,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub len_prg_rom: u8,
    pub len_chr_rom: u8,
    pub len_prg_ram: u8,
    pub mirroring: Mirroring,
    pub battery_backed_ram: bool,
    pub trainer: bool,
    pub vs_system: bool,
    pub rom_mapper: RomMapper,
    pub region: Region,
}

impl Header {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let byte6 =
            bits::<_, _, Error<(&[u8], usize)>, _, _>((bit_take(4usize), bool, bool, bool, bool))
                .map(
                    |(lower_rom_mapper, four_screen, trainer, battery_backed_ram, mirroring): (
                        u8,
                        bool,
                        bool,
                        bool,
                        bool,
                    )| {
                        (
                            match (mirroring, four_screen) {
                                (_, true) => Mirroring::FourScreen,
                                (true, false) => Mirroring::Vertical,
                                (false, false) => Mirroring::Horizontal,
                            },
                            battery_backed_ram,
                            trainer,
                            lower_rom_mapper,
                        )
                    },
                );
        let byte7 =
            bits::<_, _, Error<(&[u8], usize)>, _, _>((bit_take(4usize), bit_tag(0, 3usize), bool))
                .map(|(higher_rom_mapper, _, vs_system): (u8, u8, bool)| {
                    (vs_system, higher_rom_mapper)
                });
        let region = bits::<_, _, Error<(&[u8], usize)>, _, _>((bit_tag(0, 7usize), bool))
            .map(|(_, region)| Region::from(region));

        let (input, _) = tag("NES\x1A")(input)?;
        let (input, len_prg_rom) = u8().parse(input)?;
        let (input, len_chr_rom) = u8().parse(input)?;
        let (
            input,
            (
                (mirroring, battery_backed_ram, trainer, lower_rom_mapper),
                (vs_system, higher_rom_mapper),
            ),
        ) = (byte6, byte7).parse(input)?;
        let (input, len_prg_ram) = u8().parse(input)?;
        let (input, (region,)) = (region,).parse(input)?;
        let (input, _) = tag("\x00\x00\x00\x00\x00\x00")(input)?;

        let rom_mapper = RomMapper::from_u8((higher_rom_mapper << 4) | lower_rom_mapper).ok_or(
            nom::Err::Failure(Error {
                input,
                code: ErrorKind::Tag,
            }),
        )?;

        Ok((
            input,
            Self {
                len_chr_rom,
                battery_backed_ram,
                len_prg_rom,
                len_prg_ram,
                mirroring,
                region,
                rom_mapper,
                trainer,
                vs_system,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum RomMapper {
    None,
    NintendoMMC1,
    CNROM_Switch,
    UNROM_Switch,
    NintendoMMC3,
    NintendoMMC5,
    FFE_F4xxx,
    AOROM_Switch,
    FFE_F3xxx,
    NintendoMMC2,
    NintendoMMC4,
    ColorDreams_Chip,
    FFE_F6xxx,
    CPROM_Switch,
    HundredInOne_Switch = 15,
    Bandai_Chip,
    FFE_F8xxx,
    JalecoSS8806_Chip,
    Namcot106_Chip,
    NintendoDiskSystem,
    KonamiVRC4a,
    KonamiVRC2a_1,
    KonamiVRC2a_2,
    KonamiVRC6,
    KonamiVRC4b,
    IremG101_Chip = 32,
    TaitoTC0190_TC0350,
    Nina1_Board,
    TengenRAMBO1_Chip = 64,
    IremH3001_Chip,
    GNROM_Switch,
    SunSoft3_Chip,
    SunSoft4_Chip,
    SunSoft5_FME7_Chip,
    Camerica_Chip = 71,
    Irem74HC161_32based = 78,
    AVENina3_Board,
    AVENina6_Board = 81,
    PirateHK_SF3_Chip = 91,
}

#[derive(Debug, PartialEq)]
pub enum Region {
    NTSC,
    PAL,
}

impl From<bool> for Region {
    fn from(value: bool) -> Self {
        match value {
            true => Self::PAL,
            false => Self::NTSC,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_header_parser() {
        const HEADER: [u8; 16] = [
            0x4e, 0x45, 0x53, 0x1a, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let (input, header) = Header::parse(&HEADER).unwrap();
        assert!(input.is_empty());
        assert_eq!(
            header,
            Header {
                len_prg_rom: 0x02,
                len_chr_rom: 0x00,
                rom_mapper: RomMapper::None,
                mirroring: Mirroring::Vertical,
                vs_system: false,
                trainer: false,
                battery_backed_ram: false,
                len_prg_ram: 0x00,
                region: Region::NTSC,
            }
        )
    }

    #[test]
    fn test_rom_parser() {
        let rom_bytes = include_bytes!("../../test/snake.nes");
        let (input, rom) = NesRom::parse(rom_bytes).unwrap();
        assert!(input.is_empty());
        let prg_rom_size = 0x02 * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = 0x00 * CHR_ROM_PAGE_SIZE;
        assert_eq!(
            rom,
            NesRom {
                header: Header {
                    len_prg_rom: 0x02,
                    len_chr_rom: 0x00,
                    rom_mapper: RomMapper::None,
                    mirroring: Mirroring::Vertical,
                    vs_system: false,
                    trainer: false,
                    battery_backed_ram: false,
                    len_prg_ram: 0x00,
                    region: Region::NTSC,
                },
                trainer: None,
                prg_rom: rom_bytes[16..(16 + prg_rom_size)].to_vec(),
                chr_rom: rom_bytes[(16 + prg_rom_size)..((16 + prg_rom_size) + chr_rom_size)]
                    .to_vec()
            }
        )
    }
}
