use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BogoCryptError {
    BufferTooSmall(usize),
}

impl fmt::Display for BogoCryptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferTooSmall(size) => {
                write!(f, "buffer too small: got {size} bytes, need at least 16")
            }
        }
    }
}

impl std::error::Error for BogoCryptError {}

#[rustfmt::skip]
const TABLE2: [u8; 60] = [
    0xB0, 0xBC, 0x6D, 0x19, 0xE5, 0x14, 0xB6, 0xEA,
    0xF4, 0xCB, 0x16, 0xB1, 0x7D, 0xE5, 0x7F, 0xB5,
    0x0B, 0x7C, 0x5E, 0xC4, 0x2D, 0x4F, 0x31, 0x99,
    0x67, 0x98, 0x19, 0xE8, 0x28, 0x58, 0xDE, 0xC9,
    0xC1, 0x9B, 0xC9, 0x83, 0x34, 0xC4, 0x64, 0x85,
    0x36, 0xFA, 0x0E, 0xDE, 0xFB, 0xE7, 0x68, 0x99,
    0x71, 0x32, 0x34, 0x36, 0xE5, 0x01, 0x7D, 0x63,
    0x5C, 0x81, 0x77, 0x7C,
];

#[rustfmt::skip]
const TABLE3: [u8; 68] = [
    0xC6, 0x53, 0x98, 0xED, 0xD0, 0xAA, 0xE0, 0x1B,
    0x43, 0x31, 0x16, 0x32, 0xCE, 0x0D, 0x52, 0x57,
    0x7B, 0x72, 0x0F, 0xFB, 0xB7, 0xA9, 0xC6, 0xF1,
    0xF1, 0x72, 0x2C, 0x89, 0x5F, 0xB2, 0x1B, 0xFA,
    0xA8, 0xDB, 0xE3, 0xE2, 0x41, 0xB5, 0xBE, 0xC9,
    0x5D, 0xAE, 0xC6, 0x21, 0xE4, 0xD2, 0x88, 0x74,
    0x97, 0x26, 0x4F, 0x87, 0x7A, 0xF0, 0x69, 0x7A,
    0xB7, 0x7E, 0x15, 0x09, 0xBA, 0x55, 0xAD, 0xCA,
    0xB1, 0x60, 0x34, 0x76,
];

fn deshift(buf: &mut [u8], shift: u32) {
    if buf.is_empty() {
        return;
    }
    let first = buf[0];
    let right_shift = 8 - shift;
    for i in 0..buf.len() - 1 {
        buf[i] = (buf[i] << shift) | (buf[i + 1] >> right_shift);
    }
    let last = buf.len() - 1;
    buf[last] = (buf[last] << shift) | (first >> right_shift);
}

fn enshift(buf: &mut [u8], shift: u32) {
    if buf.is_empty() {
        return;
    }
    let last_byte = buf[buf.len() - 1];
    let left_shift = 8 - shift;
    for i in (1..buf.len()).rev() {
        buf[i] = (buf[i - 1] << left_shift) | (buf[i] >> shift);
    }
    buf[0] = (last_byte << left_shift) | (buf[0] >> shift);
}

fn xor_with_table(buf: &mut [u8], table: &[u8], seed: usize) {
    let table_len = table.len() / 4;
    let mut table_index = seed;
    let mut i = 0;
    while i + 4 <= buf.len() {
        table_index %= table_len;
        let t = table_index * 4;
        table_index += 1;
        buf[i] ^= table[t];
        buf[i + 1] ^= table[t + 1];
        buf[i + 2] ^= table[t + 2];
        buf[i + 3] ^= table[t + 3];
        i += 4;
    }
}

pub fn decrypt(buf: &mut [u8]) -> Result<(), BogoCryptError> {
    if buf.is_empty() {
        return Ok(());
    }
    if buf.len() < 16 {
        return Err(BogoCryptError::BufferTooSmall(buf.len()));
    }
    deshift(buf, 3);
    xor_with_table(buf, &TABLE2, 8);
    let seed = (buf[15] & 0x7F) as usize;
    xor_with_table(&mut buf[16..], &TABLE3, seed);
    Ok(())
}

pub fn encrypt(buf: &mut [u8]) -> Result<(), BogoCryptError> {
    if buf.is_empty() {
        return Ok(());
    }
    if buf.len() < 16 {
        return Err(BogoCryptError::BufferTooSmall(buf.len()));
    }
    let seed = (buf[15] & 0x7F) as usize;
    xor_with_table(&mut buf[16..], &TABLE3, seed);
    xor_with_table(buf, &TABLE2, 8);
    enshift(buf, 3);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_data_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("original_game_files")
    }

    #[test]
    fn deshift_enshift_roundtrip() {
        let original: Vec<u8> = (0..64).collect();
        let mut buf = original.clone();
        deshift(&mut buf, 3);
        assert_ne!(buf, original);
        enshift(&mut buf, 3);
        assert_eq!(buf, original);
    }

    #[test]
    fn enshift_deshift_roundtrip() {
        let original: Vec<u8> = (0..64).collect();
        let mut buf = original.clone();
        enshift(&mut buf, 3);
        assert_ne!(buf, original);
        deshift(&mut buf, 3);
        assert_eq!(buf, original);
    }

    #[test]
    fn deshift_enshift_single_byte() {
        let original = vec![0xAB];
        let mut buf = original.clone();
        deshift(&mut buf, 3);
        enshift(&mut buf, 3);
        assert_eq!(buf, original);
    }

    #[test]
    fn xor_with_table_self_inverse() {
        let original: Vec<u8> = (0..64).collect();
        let mut buf = original.clone();
        xor_with_table(&mut buf, &TABLE2, 0);
        assert_ne!(buf, original);
        xor_with_table(&mut buf, &TABLE2, 0);
        assert_eq!(buf, original);
    }

    #[test]
    fn xor_with_table_trailing_bytes_untouched() {
        let original: Vec<u8> = (0..6).collect();
        let mut buf = original.clone();
        xor_with_table(&mut buf, &TABLE2, 0);
        // First 4 bytes changed, last 2 untouched
        assert_ne!(&buf[..4], &original[..4]);
        assert_eq!(&buf[4..], &original[4..]);
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let original: Vec<u8> = (0u8..=255).cycle().take(1024).collect();
        let mut buf = original.clone();
        encrypt(&mut buf).unwrap();
        assert_ne!(buf, original);
        decrypt(&mut buf).unwrap();
        assert_eq!(buf, original);
    }

    #[test]
    fn decrypt_encrypt_roundtrip() {
        let original: Vec<u8> = (0u8..=255).cycle().take(1024).collect();
        let mut buf = original.clone();
        decrypt(&mut buf).unwrap();
        encrypt(&mut buf).unwrap();
        assert_eq!(buf, original);
    }

    #[test]
    fn roundtrip_exactly_16_bytes() {
        let original: Vec<u8> = (0..16).collect();
        let mut buf = original.clone();
        encrypt(&mut buf).unwrap();
        decrypt(&mut buf).unwrap();
        assert_eq!(buf, original);
    }

    #[test]
    fn roundtrip_17_bytes() {
        let original: Vec<u8> = (0..17).collect();
        let mut buf = original.clone();
        encrypt(&mut buf).unwrap();
        decrypt(&mut buf).unwrap();
        assert_eq!(buf, original);
    }

    #[test]
    fn roundtrip_non_multiple_of_4() {
        let original: Vec<u8> = (0..35).collect();
        let mut buf = original.clone();
        encrypt(&mut buf).unwrap();
        decrypt(&mut buf).unwrap();
        assert_eq!(buf, original);
    }

    #[test]
    fn empty_buffer_is_noop() {
        let mut buf: Vec<u8> = vec![];
        assert!(encrypt(&mut buf).is_ok());
        assert!(decrypt(&mut buf).is_ok());
    }

    #[test]
    fn buffer_too_small() {
        let mut buf = vec![0u8; 15];
        assert_eq!(
            encrypt(&mut buf).unwrap_err(),
            BogoCryptError::BufferTooSmall(15)
        );
        assert_eq!(
            decrypt(&mut buf).unwrap_err(),
            BogoCryptError::BufferTooSmall(15)
        );
    }

    #[test]
    fn buffer_too_small_single_byte() {
        let mut buf = vec![0xFF];
        assert_eq!(
            encrypt(&mut buf).unwrap_err(),
            BogoCryptError::BufferTooSmall(1)
        );
    }

    #[test]
    fn decrypt_real_dat_produces_zip() {
        let dat_path = test_data_dir().join("battle/battle_data_release.dat");
        if !dat_path.exists() {
            eprintln!("skipping: test data not found at {}", dat_path.display());
            return;
        }
        let mut buf = std::fs::read(&dat_path).unwrap();
        decrypt(&mut buf).unwrap();
        assert_eq!(&buf[0..2], b"PK", "decrypted .dat should be a ZIP archive");
    }

    #[test]
    fn roundtrip_real_dat_is_exact() {
        let dat_path = test_data_dir().join("battle/battle_data_release.dat");
        if !dat_path.exists() {
            eprintln!("skipping: test data not found at {}", dat_path.display());
            return;
        }
        let original = std::fs::read(&dat_path).unwrap();
        let mut buf = original.clone();
        decrypt(&mut buf).unwrap();
        encrypt(&mut buf).unwrap();
        assert_eq!(
            buf, original,
            "decrypt→encrypt must produce identical bytes"
        );
    }

    #[test]
    fn roundtrip_multiple_real_dats() {
        let data_dir = test_data_dir();
        let dats = ["battle/battle_data_release.dat", "menu/menu_data.dat"];
        for dat in &dats {
            let dat_path = data_dir.join(dat);
            if !dat_path.exists() {
                eprintln!("skipping: {dat}");
                continue;
            }
            let original = std::fs::read(&dat_path).unwrap();
            let mut buf = original.clone();
            decrypt(&mut buf).unwrap();
            assert_eq!(&buf[0..2], b"PK", "{dat} should decrypt to ZIP");
            encrypt(&mut buf).unwrap();
            assert_eq!(buf, original, "{dat} round-trip must be byte-exact");
        }
    }

    #[test]
    fn error_display() {
        let err = BogoCryptError::BufferTooSmall(5);
        assert_eq!(
            err.to_string(),
            "buffer too small: got 5 bytes, need at least 16"
        );
    }
}
