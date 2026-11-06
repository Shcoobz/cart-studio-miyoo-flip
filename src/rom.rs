//! What the page needs from a ROM's bytes: its game code, for the shell colour, and its CRC32,
//! to find it in the No-Intro DAT.

use slot_store::Platform;

/// The iNES / NES 2.0 header: 16 bytes starting `NES\x1a`.
const INES_MAGIC: [u8; 4] = *b"NES\x1a";
const INES_LEN: u32 = 16;
/// A copier's header in front of a SNES rom: 512 bytes, there when the size is 512 past a
/// multiple of 1 KiB, since every real SNES rom is a whole number of KiB.
const COPIER_LEN: u32 = 512;

/// How many bytes at the front of a rom the No-Intro CRC leaves out. No-Intro hashes the NES rom
/// without its 16-byte iNES header (libretro's DAT lists that dump as the `.unh` beside the
/// headered `.nes`, so the headerless CRC matches whatever header a dumper wrote) and the SNES
/// rom without a copier header. `head` is the rom's first bytes and `size` its length.
pub fn crc_skip(platform: Platform, head: &[u8], size: u64) -> u32 {
    match platform {
        Platform::Nes if head.starts_with(&INES_MAGIC) && size > u64::from(INES_LEN) => INES_LEN,
        Platform::Snes if size % 1024 == u64::from(COPIER_LEN) => COPIER_LEN,
        _ => 0,
    }
}

/// The four character game code at 0xAC, read the way `slot_store::gba` reads it: up to the
/// first NUL, trimmed, and empty when there's nothing there. A cart then gets the same shell it
/// gets on the device.
pub fn header_code(head: &[u8]) -> String {
    let Some(field) = head.get(0xac..0xb0) else {
        return String::new();
    };
    let end = field.iter().position(|b| *b == 0).unwrap_or(field.len());
    std::str::from_utf8(&field[..end])
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

/// A ROM is up to 32 MB and the page reads it as a stream, so the hash is fed in pieces.
#[derive(Default)]
pub struct Crc32(crc32fast::Hasher);

impl Crc32 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, chunk: &[u8]) {
        self.0.update(chunk);
    }

    pub fn finish(self) -> u32 {
        self.0.finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_check_value_is_the_standard_one() {
        let mut crc = Crc32::new();
        crc.update(b"123456789");
        assert_eq!(crc.finish(), 0xCBF4_3926);
    }

    #[test]
    fn chunks_hash_the_same_as_one_update() {
        let data: Vec<u8> = (0..100_000u32).map(|i| (i * 7 % 251) as u8).collect();
        let mut whole = Crc32::new();
        whole.update(&data);
        let mut parts = Crc32::new();
        for chunk in data.chunks(4113) {
            parts.update(chunk);
        }
        assert_eq!(whole.finish(), parts.finish());
    }

    #[test]
    fn an_ines_header_is_left_out_of_the_crc() {
        let mut head = vec![0u8; 0x150];
        head[..4].copy_from_slice(b"NES\x1a");
        assert_eq!(crc_skip(Platform::Nes, &head, 40976), 16);
        // No magic, no header: a headerless .nes is hashed whole.
        assert_eq!(crc_skip(Platform::Nes, &[0u8; 0x150], 40960), 0);
        // The magic means nothing on another platform.
        assert_eq!(crc_skip(Platform::Gb, &head, 40976), 0);
    }

    #[test]
    fn a_snes_copier_header_is_left_out_of_the_crc() {
        assert_eq!(crc_skip(Platform::Snes, &[], 1_048_576 + 512), 512);
        assert_eq!(crc_skip(Platform::Snes, &[], 1_048_576), 0);
        assert_eq!(crc_skip(Platform::Gba, &[], 1_048_576 + 512), 0);
    }

    #[test]
    fn the_code_stops_at_a_nul_and_is_empty_when_missing() {
        let mut head = vec![0u8; 0xb0];
        head[0xac..0xb0].copy_from_slice(b"AB\0\0");
        assert_eq!(header_code(&head), "AB");
        assert_eq!(header_code(&[0u8; 0xb0]), "");
        assert_eq!(header_code(&[0u8; 0x20]), "");
    }
}
