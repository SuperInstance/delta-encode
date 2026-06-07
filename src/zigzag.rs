//! Zigzag encoding for signed integers.

pub fn encode_i64(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

/// Zigzag decode a u64 back into an i64.
pub fn decode_i64(n: u64) -> i64 {
    ((n >> 1) as i64) ^ -((n & 1) as i64)
}

/// Zigzag encode an i32 into a u32.
pub fn encode_i32(n: i32) -> u32 {
    ((n << 1) ^ (n >> 31)) as u32
}

/// Zigzag decode a u32 back into an i32.
pub fn decode_i32(n: u32) -> i32 {
    ((n >> 1) as i32) ^ -((n & 1) as i32)
}

/// Zigzag encode an i16 into a u16.
pub fn encode_i16(n: i16) -> u16 {
    ((n << 1) ^ (n >> 15)) as u16
}

/// Zigzag decode a u16 back into an i16.
pub fn decode_i16(n: u16) -> i16 {
    ((n >> 1) as i16) ^ -((n & 1) as i16)
}

/// Encode a slice of i64 values using zigzag encoding.
pub fn encode_slice(values: &[i64]) -> Vec<u64> {
    values.iter().map(|&v| encode_i64(v)).collect()
}

/// Decode a slice of zigzag-encoded u64 values back to i64.
pub fn decode_slice(values: &[u64]) -> Vec<i64> {
    values.iter().map(|&v| decode_i64(v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(encode_i64(0), 0);
        assert_eq!(decode_i64(0), 0);
    }

    #[test]
    fn test_positive() {
        assert_eq!(encode_i64(1), 2);
        assert_eq!(decode_i64(2), 1);
        assert_eq!(encode_i64(2), 4);
        assert_eq!(decode_i64(4), 2);
    }

    #[test]
    fn test_negative() {
        assert_eq!(encode_i64(-1), 1);
        assert_eq!(decode_i64(1), -1);
        assert_eq!(encode_i64(-2), 3);
        assert_eq!(decode_i64(3), -2);
    }

    #[test]
    fn test_i32_roundtrip() {
        for v in [-100, -1, 0, 1, 100, i32::MIN, i32::MAX] {
            assert_eq!(decode_i32(encode_i32(v)), v);
        }
    }

    #[test]
    fn test_i16_roundtrip() {
        for v in [-100i16, -1, 0, 1, 100, i16::MIN, i16::MAX] {
            assert_eq!(decode_i16(encode_i16(v)), v);
        }
    }

    #[test]
    fn test_slice_roundtrip() {
        let values = vec![-10i64, -5, 0, 3, 7, 100, -200];
        let encoded = encode_slice(&values);
        let decoded = decode_slice(&encoded);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_small_values_map_to_small_unsigned() {
        assert!(encode_i64(0) < encode_i64(100));
        assert!(encode_i64(-1) < encode_i64(100));
        assert!(encode_i64(1) < encode_i64(100));
        assert!(encode_i64(-1) < encode_i64(-100));
    }

    #[test]
    fn test_extreme_values() {
        assert_eq!(decode_i64(encode_i64(i64::MIN)), i64::MIN);
        assert_eq!(decode_i64(encode_i64(i64::MAX)), i64::MAX);
    }
}
