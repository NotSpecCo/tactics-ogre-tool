use super::{check_bounds, BinaryError, Endian};

pub fn read_u8(data: &[u8], offset: usize) -> Result<u8, BinaryError> {
    check_bounds(data.len(), offset, 1)?;
    Ok(data[offset])
}

pub fn read_u16(data: &[u8], offset: usize, endian: Endian) -> Result<u16, BinaryError> {
    check_bounds(data.len(), offset, 2)?;
    let bytes: [u8; 2] = data[offset..offset + 2].try_into().unwrap();
    Ok(match endian {
        Endian::Little => u16::from_le_bytes(bytes),
        Endian::Big => u16::from_be_bytes(bytes),
    })
}

pub fn read_u32(data: &[u8], offset: usize, endian: Endian) -> Result<u32, BinaryError> {
    check_bounds(data.len(), offset, 4)?;
    let bytes: [u8; 4] = data[offset..offset + 4].try_into().unwrap();
    Ok(match endian {
        Endian::Little => u32::from_le_bytes(bytes),
        Endian::Big => u32::from_be_bytes(bytes),
    })
}

pub fn read_i8(data: &[u8], offset: usize) -> Result<i8, BinaryError> {
    check_bounds(data.len(), offset, 1)?;
    Ok(data[offset] as i8)
}

pub fn read_i16(data: &[u8], offset: usize, endian: Endian) -> Result<i16, BinaryError> {
    check_bounds(data.len(), offset, 2)?;
    let bytes: [u8; 2] = data[offset..offset + 2].try_into().unwrap();
    Ok(match endian {
        Endian::Little => i16::from_le_bytes(bytes),
        Endian::Big => i16::from_be_bytes(bytes),
    })
}

pub fn read_i32(data: &[u8], offset: usize, endian: Endian) -> Result<i32, BinaryError> {
    check_bounds(data.len(), offset, 4)?;
    let bytes: [u8; 4] = data[offset..offset + 4].try_into().unwrap();
    Ok(match endian {
        Endian::Little => i32::from_le_bytes(bytes),
        Endian::Big => i32::from_be_bytes(bytes),
    })
}

pub fn read_bytes(data: &[u8], offset: usize, len: usize) -> Result<&[u8], BinaryError> {
    check_bounds(data.len(), offset, len)?;
    Ok(&data[offset..offset + len])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_u8_basic() {
        let data = [0x42, 0xFF, 0x00];
        assert_eq!(read_u8(&data, 0).unwrap(), 0x42);
        assert_eq!(read_u8(&data, 1).unwrap(), 0xFF);
        assert_eq!(read_u8(&data, 2).unwrap(), 0x00);
    }

    #[test]
    fn read_u8_out_of_bounds() {
        let data = [0x42];
        let err = read_u8(&data, 1).unwrap_err();
        assert!(matches!(
            err,
            BinaryError::OutOfBounds {
                offset: 1,
                size: 1,
                len: 1
            }
        ));
    }

    #[test]
    fn read_u8_empty() {
        let data: [u8; 0] = [];
        assert!(read_u8(&data, 0).is_err());
    }

    #[test]
    fn read_u16_little_endian() {
        let data = [0x34, 0x12];
        assert_eq!(read_u16(&data, 0, Endian::Little).unwrap(), 0x1234);
    }

    #[test]
    fn read_u16_big_endian() {
        let data = [0x12, 0x34];
        assert_eq!(read_u16(&data, 0, Endian::Big).unwrap(), 0x1234);
    }

    #[test]
    fn read_u16_at_offset() {
        let data = [0x00, 0x34, 0x12, 0x00];
        assert_eq!(read_u16(&data, 1, Endian::Little).unwrap(), 0x1234);
    }

    #[test]
    fn read_u16_out_of_bounds() {
        let data = [0x42];
        let err = read_u16(&data, 0, Endian::Little).unwrap_err();
        assert!(matches!(
            err,
            BinaryError::OutOfBounds {
                offset: 0,
                size: 2,
                len: 1
            }
        ));
    }

    #[test]
    fn read_u16_partial_overlap() {
        let data = [0x00, 0x42];
        let err = read_u16(&data, 1, Endian::Little).unwrap_err();
        assert!(matches!(
            err,
            BinaryError::OutOfBounds {
                offset: 1,
                size: 2,
                len: 2
            }
        ));
    }

    #[test]
    fn read_u32_little_endian() {
        let data = [0x78, 0x56, 0x34, 0x12];
        assert_eq!(read_u32(&data, 0, Endian::Little).unwrap(), 0x12345678);
    }

    #[test]
    fn read_u32_big_endian() {
        let data = [0x12, 0x34, 0x56, 0x78];
        assert_eq!(read_u32(&data, 0, Endian::Big).unwrap(), 0x12345678);
    }

    #[test]
    fn read_u32_at_offset() {
        let data = [0xFF, 0xFF, 0x78, 0x56, 0x34, 0x12];
        assert_eq!(read_u32(&data, 2, Endian::Little).unwrap(), 0x12345678);
    }

    #[test]
    fn read_u32_out_of_bounds() {
        let data = [0x00, 0x00, 0x00];
        assert!(read_u32(&data, 0, Endian::Little).is_err());
    }

    #[test]
    fn read_i8_positive() {
        let data = [0x7F];
        assert_eq!(read_i8(&data, 0).unwrap(), 127);
    }

    #[test]
    fn read_i8_negative() {
        let data = [0x80];
        assert_eq!(read_i8(&data, 0).unwrap(), -128);
    }

    #[test]
    fn read_i8_minus_one() {
        let data = [0xFF];
        assert_eq!(read_i8(&data, 0).unwrap(), -1);
    }

    #[test]
    fn read_i16_positive_le() {
        let data = [0xFF, 0x7F];
        assert_eq!(read_i16(&data, 0, Endian::Little).unwrap(), 32767);
    }

    #[test]
    fn read_i16_negative_le() {
        let data = [0x00, 0x80];
        assert_eq!(read_i16(&data, 0, Endian::Little).unwrap(), -32768);
    }

    #[test]
    fn read_i16_negative_be() {
        let data = [0x80, 0x00];
        assert_eq!(read_i16(&data, 0, Endian::Big).unwrap(), -32768);
    }

    #[test]
    fn read_i32_positive_le() {
        let data = [0x01, 0x00, 0x00, 0x00];
        assert_eq!(read_i32(&data, 0, Endian::Little).unwrap(), 1);
    }

    #[test]
    fn read_i32_negative_le() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(read_i32(&data, 0, Endian::Little).unwrap(), -1);
    }

    #[test]
    fn read_i32_negative_be() {
        let data = [0xFF, 0xFF, 0xFF, 0xFE];
        assert_eq!(read_i32(&data, 0, Endian::Big).unwrap(), -2);
    }

    #[test]
    fn read_i32_min_le() {
        let data = [0x00, 0x00, 0x00, 0x80];
        assert_eq!(read_i32(&data, 0, Endian::Little).unwrap(), i32::MIN);
    }

    #[test]
    fn read_bytes_basic() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05];
        assert_eq!(read_bytes(&data, 1, 3).unwrap(), &[0x02, 0x03, 0x04]);
    }

    #[test]
    fn read_bytes_full() {
        let data = [0xAA, 0xBB];
        assert_eq!(read_bytes(&data, 0, 2).unwrap(), &[0xAA, 0xBB]);
    }

    #[test]
    fn read_bytes_zero_len() {
        let data = [0x01];
        assert_eq!(read_bytes(&data, 0, 0).unwrap(), &[] as &[u8]);
    }

    #[test]
    fn read_bytes_out_of_bounds() {
        let data = [0x01, 0x02];
        assert!(read_bytes(&data, 1, 2).is_err());
    }

    #[test]
    fn read_u16_max_value() {
        let data = [0xFF, 0xFF];
        assert_eq!(read_u16(&data, 0, Endian::Little).unwrap(), u16::MAX);
        assert_eq!(read_u16(&data, 0, Endian::Big).unwrap(), u16::MAX);
    }

    #[test]
    fn read_u32_max_value() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(read_u32(&data, 0, Endian::Little).unwrap(), u32::MAX);
        assert_eq!(read_u32(&data, 0, Endian::Big).unwrap(), u32::MAX);
    }

    #[test]
    fn read_at_exact_end_fails() {
        let data = [0x01, 0x02];
        assert!(read_u8(&data, 2).is_err());
        assert!(read_u16(&data, 2, Endian::Little).is_err());
    }

    #[test]
    fn read_multiple_fields_from_buffer() {
        // Simulate a record: u8 at 0, u16 at 1, u32 at 3, i8 at 7
        let data = [
            0x0A, // u8 = 10
            0xE8, 0x03, // u16 LE = 1000
            0x40, 0x42, 0x0F, 0x00, // u32 LE = 1_000_000
            0xFE, // i8 = -2
        ];
        assert_eq!(read_u8(&data, 0).unwrap(), 10);
        assert_eq!(read_u16(&data, 1, Endian::Little).unwrap(), 1000);
        assert_eq!(read_u32(&data, 3, Endian::Little).unwrap(), 1_000_000);
        assert_eq!(read_i8(&data, 7).unwrap(), -2);
    }
}
