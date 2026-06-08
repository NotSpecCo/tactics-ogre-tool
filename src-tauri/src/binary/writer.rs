use super::{check_bounds, BinaryError, Endian};

pub fn write_u8(data: &mut [u8], offset: usize, value: u8) -> Result<(), BinaryError> {
    check_bounds(data.len(), offset, 1)?;
    data[offset] = value;
    Ok(())
}

pub fn write_u16(
    data: &mut [u8],
    offset: usize,
    value: u16,
    endian: Endian,
) -> Result<(), BinaryError> {
    check_bounds(data.len(), offset, 2)?;
    let bytes = match endian {
        Endian::Little => value.to_le_bytes(),
        Endian::Big => value.to_be_bytes(),
    };
    data[offset..offset + 2].copy_from_slice(&bytes);
    Ok(())
}

pub fn write_u32(
    data: &mut [u8],
    offset: usize,
    value: u32,
    endian: Endian,
) -> Result<(), BinaryError> {
    check_bounds(data.len(), offset, 4)?;
    let bytes = match endian {
        Endian::Little => value.to_le_bytes(),
        Endian::Big => value.to_be_bytes(),
    };
    data[offset..offset + 4].copy_from_slice(&bytes);
    Ok(())
}

pub fn write_i8(data: &mut [u8], offset: usize, value: i8) -> Result<(), BinaryError> {
    check_bounds(data.len(), offset, 1)?;
    data[offset] = value as u8;
    Ok(())
}

pub fn write_i16(
    data: &mut [u8],
    offset: usize,
    value: i16,
    endian: Endian,
) -> Result<(), BinaryError> {
    check_bounds(data.len(), offset, 2)?;
    let bytes = match endian {
        Endian::Little => value.to_le_bytes(),
        Endian::Big => value.to_be_bytes(),
    };
    data[offset..offset + 2].copy_from_slice(&bytes);
    Ok(())
}

pub fn write_i32(
    data: &mut [u8],
    offset: usize,
    value: i32,
    endian: Endian,
) -> Result<(), BinaryError> {
    check_bounds(data.len(), offset, 4)?;
    let bytes = match endian {
        Endian::Little => value.to_le_bytes(),
        Endian::Big => value.to_be_bytes(),
    };
    data[offset..offset + 4].copy_from_slice(&bytes);
    Ok(())
}

pub fn write_bytes(data: &mut [u8], offset: usize, bytes: &[u8]) -> Result<(), BinaryError> {
    check_bounds(data.len(), offset, bytes.len())?;
    data[offset..offset + bytes.len()].copy_from_slice(bytes);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_u8_basic() {
        let mut data = [0x00; 3];
        write_u8(&mut data, 1, 0x42).unwrap();
        assert_eq!(data, [0x00, 0x42, 0x00]);
    }

    #[test]
    fn write_u8_out_of_bounds() {
        let mut data = [0x00];
        assert!(write_u8(&mut data, 1, 0x42).is_err());
    }

    #[test]
    fn write_u16_little_endian() {
        let mut data = [0x00; 4];
        write_u16(&mut data, 1, 0x1234, Endian::Little).unwrap();
        assert_eq!(data, [0x00, 0x34, 0x12, 0x00]);
    }

    #[test]
    fn write_u16_big_endian() {
        let mut data = [0x00; 4];
        write_u16(&mut data, 1, 0x1234, Endian::Big).unwrap();
        assert_eq!(data, [0x00, 0x12, 0x34, 0x00]);
    }

    #[test]
    fn write_u16_out_of_bounds() {
        let mut data = [0x00; 2];
        assert!(write_u16(&mut data, 1, 0x1234, Endian::Little).is_err());
    }

    #[test]
    fn write_u32_little_endian() {
        let mut data = [0x00; 6];
        write_u32(&mut data, 1, 0x12345678, Endian::Little).unwrap();
        assert_eq!(data, [0x00, 0x78, 0x56, 0x34, 0x12, 0x00]);
    }

    #[test]
    fn write_u32_big_endian() {
        let mut data = [0x00; 6];
        write_u32(&mut data, 1, 0x12345678, Endian::Big).unwrap();
        assert_eq!(data, [0x00, 0x12, 0x34, 0x56, 0x78, 0x00]);
    }

    #[test]
    fn write_u32_out_of_bounds() {
        let mut data = [0x00; 3];
        assert!(write_u32(&mut data, 0, 0x12345678, Endian::Little).is_err());
    }

    #[test]
    fn write_i8_positive() {
        let mut data = [0x00];
        write_i8(&mut data, 0, 127).unwrap();
        assert_eq!(data, [0x7F]);
    }

    #[test]
    fn write_i8_negative() {
        let mut data = [0x00];
        write_i8(&mut data, 0, -1).unwrap();
        assert_eq!(data, [0xFF]);
    }

    #[test]
    fn write_i8_min() {
        let mut data = [0x00];
        write_i8(&mut data, 0, -128).unwrap();
        assert_eq!(data, [0x80]);
    }

    #[test]
    fn write_i16_positive_le() {
        let mut data = [0x00; 2];
        write_i16(&mut data, 0, 32767, Endian::Little).unwrap();
        assert_eq!(data, [0xFF, 0x7F]);
    }

    #[test]
    fn write_i16_negative_le() {
        let mut data = [0x00; 2];
        write_i16(&mut data, 0, -1, Endian::Little).unwrap();
        assert_eq!(data, [0xFF, 0xFF]);
    }

    #[test]
    fn write_i16_negative_be() {
        let mut data = [0x00; 2];
        write_i16(&mut data, 0, -32768, Endian::Big).unwrap();
        assert_eq!(data, [0x80, 0x00]);
    }

    #[test]
    fn write_i32_positive_le() {
        let mut data = [0x00; 4];
        write_i32(&mut data, 0, 1_000_000, Endian::Little).unwrap();
        assert_eq!(data, [0x40, 0x42, 0x0F, 0x00]);
    }

    #[test]
    fn write_i32_negative_le() {
        let mut data = [0x00; 4];
        write_i32(&mut data, 0, -1, Endian::Little).unwrap();
        assert_eq!(data, [0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn write_i32_min_le() {
        let mut data = [0x00; 4];
        write_i32(&mut data, 0, i32::MIN, Endian::Little).unwrap();
        assert_eq!(data, [0x00, 0x00, 0x00, 0x80]);
    }

    #[test]
    fn write_bytes_basic() {
        let mut data = [0x00; 5];
        write_bytes(&mut data, 1, &[0xAA, 0xBB, 0xCC]).unwrap();
        assert_eq!(data, [0x00, 0xAA, 0xBB, 0xCC, 0x00]);
    }

    #[test]
    fn write_bytes_empty() {
        let mut data = [0x42; 2];
        write_bytes(&mut data, 1, &[]).unwrap();
        assert_eq!(data, [0x42, 0x42]);
    }

    #[test]
    fn write_bytes_out_of_bounds() {
        let mut data = [0x00; 3];
        assert!(write_bytes(&mut data, 2, &[0x01, 0x02]).is_err());
    }

    #[test]
    fn write_overwrites_existing() {
        let mut data = [0xFF; 4];
        write_u16(&mut data, 1, 0x0000, Endian::Little).unwrap();
        assert_eq!(data, [0xFF, 0x00, 0x00, 0xFF]);
    }

    #[test]
    fn write_multiple_fields() {
        let mut data = [0x00; 8];
        write_u8(&mut data, 0, 0x0A).unwrap();
        write_u16(&mut data, 1, 1000, Endian::Little).unwrap();
        write_u32(&mut data, 3, 1_000_000, Endian::Little).unwrap();
        write_i8(&mut data, 7, -2).unwrap();
        assert_eq!(data, [0x0A, 0xE8, 0x03, 0x40, 0x42, 0x0F, 0x00, 0xFE]);
    }

    #[test]
    fn write_u16_max() {
        let mut data = [0x00; 2];
        write_u16(&mut data, 0, u16::MAX, Endian::Little).unwrap();
        assert_eq!(data, [0xFF, 0xFF]);
    }

    #[test]
    fn write_u32_max() {
        let mut data = [0x00; 4];
        write_u32(&mut data, 0, u32::MAX, Endian::Little).unwrap();
        assert_eq!(data, [0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn write_at_exact_end_fails() {
        let mut data = [0x00; 2];
        assert!(write_u8(&mut data, 2, 0x42).is_err());
        assert!(write_u16(&mut data, 2, 0x1234, Endian::Little).is_err());
    }

    // --- Read/write round-trip tests ---

    #[test]
    fn roundtrip_u8() {
        let mut data = [0x00; 4];
        write_u8(&mut data, 2, 0xAB).unwrap();
        assert_eq!(crate::binary::reader::read_u8(&data, 2).unwrap(), 0xAB);
    }

    #[test]
    fn roundtrip_u16_le() {
        let mut data = [0x00; 4];
        write_u16(&mut data, 1, 0xBEEF, Endian::Little).unwrap();
        assert_eq!(
            crate::binary::reader::read_u16(&data, 1, Endian::Little).unwrap(),
            0xBEEF
        );
    }

    #[test]
    fn roundtrip_u16_be() {
        let mut data = [0x00; 4];
        write_u16(&mut data, 1, 0xBEEF, Endian::Big).unwrap();
        assert_eq!(
            crate::binary::reader::read_u16(&data, 1, Endian::Big).unwrap(),
            0xBEEF
        );
    }

    #[test]
    fn roundtrip_u32_le() {
        let mut data = [0x00; 8];
        write_u32(&mut data, 2, 0xDEADBEEF, Endian::Little).unwrap();
        assert_eq!(
            crate::binary::reader::read_u32(&data, 2, Endian::Little).unwrap(),
            0xDEADBEEF
        );
    }

    #[test]
    fn roundtrip_i8() {
        let mut data = [0x00; 2];
        write_i8(&mut data, 0, -42).unwrap();
        assert_eq!(crate::binary::reader::read_i8(&data, 0).unwrap(), -42);
    }

    #[test]
    fn roundtrip_i16_le() {
        let mut data = [0x00; 4];
        write_i16(&mut data, 1, -12345, Endian::Little).unwrap();
        assert_eq!(
            crate::binary::reader::read_i16(&data, 1, Endian::Little).unwrap(),
            -12345
        );
    }

    #[test]
    fn roundtrip_i32_le() {
        let mut data = [0x00; 8];
        write_i32(&mut data, 2, -1_000_000, Endian::Little).unwrap();
        assert_eq!(
            crate::binary::reader::read_i32(&data, 2, Endian::Little).unwrap(),
            -1_000_000
        );
    }

    #[test]
    fn roundtrip_bytes() {
        let mut data = [0x00; 8];
        write_bytes(&mut data, 2, &[0xDE, 0xAD, 0xBE, 0xEF]).unwrap();
        assert_eq!(
            crate::binary::reader::read_bytes(&data, 2, 4).unwrap(),
            &[0xDE, 0xAD, 0xBE, 0xEF]
        );
    }

    #[test]
    fn roundtrip_mixed_record() {
        let mut data = [0x00; 16];
        let e = Endian::Little;

        write_u8(&mut data, 0, 10).unwrap();
        write_u16(&mut data, 1, 1000, e).unwrap();
        write_i32(&mut data, 4, -999, e).unwrap();
        write_bytes(&mut data, 8, &[0xCA, 0xFE]).unwrap();
        write_u32(&mut data, 12, 0x12345678, e).unwrap();

        assert_eq!(crate::binary::reader::read_u8(&data, 0).unwrap(), 10);
        assert_eq!(crate::binary::reader::read_u16(&data, 1, e).unwrap(), 1000);
        assert_eq!(crate::binary::reader::read_i32(&data, 4, e).unwrap(), -999);
        assert_eq!(
            crate::binary::reader::read_bytes(&data, 8, 2).unwrap(),
            &[0xCA, 0xFE]
        );
        assert_eq!(
            crate::binary::reader::read_u32(&data, 12, e).unwrap(),
            0x12345678
        );
    }
}
