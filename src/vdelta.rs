//! Variable-length delta encoding using varint.

use crate::zigzag;

/// Encode a sequence of i64 deltas using variable-length encoding.
/// Applies zigzag encoding to handle signed values, then varint encoding.
pub fn encode_varint_deltas(values: &[i64]) -> Vec<u8> {
    if values.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();

    // First value: zigzag + varint
    let first_zz = zigzag::encode_i64(values[0]);
    encode_varint_u64(&mut result, first_zz);

    // Deltas
    for i in 1..values.len() {
        let delta = values[i] - values[i - 1];
        let zz = zigzag::encode_i64(delta);
        encode_varint_u64(&mut result, zz);
    }

    result
}

/// Decode variable-length delta encoded data.
/// Returns the decoded i64 values.
pub fn decode_varint_deltas(data: &[u8], count: usize) -> Result<Vec<i64>, String> {
    if count == 0 {
        return Ok(Vec::new());
    }

    let mut result = Vec::with_capacity(count);
    let mut pos = 0;

    // First value
    let (first_zz, bytes_read) = decode_varint_u64(data, pos)?;
    pos += bytes_read;
    let first = zigzag::decode_i64(first_zz);
    result.push(first);

    // Deltas
    for _ in 1..count {
        let (zz, bytes_read) = decode_varint_u64(data, pos)?;
        pos += bytes_read;
        let delta = zigzag::decode_i64(zz);
        let value = result.last().unwrap() + delta;
        result.push(value);
    }

    Ok(result)
}

/// Encode a sequence of u64 values using unsigned varint delta encoding.
pub fn encode_u64_varint_deltas(values: &[u64]) -> Vec<u8> {
    if values.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    encode_varint_u64(&mut result, values[0]);

    for i in 1..values.len() {
        let delta = values[i] as i64 - values[i - 1] as i64;
        let zz = zigzag::encode_i64(delta);
        encode_varint_u64(&mut result, zz);
    }

    result
}

/// Encode a u64 as a varint (LEB128-style).
fn encode_varint_u64(buf: &mut Vec<u8>, mut value: u64) {
    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if value == 0 {
            break;
        }
    }
}

/// Decode a varint from a byte slice.
fn decode_varint_u64(data: &[u8], start: usize) -> Result<(u64, usize), String> {
    let mut value: u64 = 0;
    let mut shift: u32 = 0;
    let mut pos = start;

    loop {
        if pos >= data.len() {
            return Err("Unexpected end of varint data".to_string());
        }
        let byte = data[pos];
        value |= ((byte & 0x7F) as u64) << shift;
        pos += 1;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return Err("Varint too large".to_string());
        }
    }

    Ok((value, pos - start))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(encode_varint_deltas(&[]).is_empty());
        assert!(decode_varint_deltas(&[], 0).unwrap().is_empty());
    }

    #[test]
    fn test_varint_single_byte() {
        let mut buf = Vec::new();
        encode_varint_u64(&mut buf, 127);
        assert_eq!(buf.len(), 1);
        let (val, len) = decode_varint_u64(&buf, 0).unwrap();
        assert_eq!(val, 127);
        assert_eq!(len, 1);
    }

    #[test]
    fn test_varint_multi_byte() {
        let mut buf = Vec::new();
        encode_varint_u64(&mut buf, 300);
        let (val, len) = decode_varint_u64(&buf, 0).unwrap();
        assert_eq!(val, 300);
        assert!(len > 1);
    }

    #[test]
    fn test_roundtrip_small_deltas() {
        let values = vec![0i64, 1, 2, 3, 4, 5];
        let encoded = encode_varint_deltas(&values);
        let decoded = decode_varint_deltas(&encoded, values.len()).unwrap();
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_roundtrip_negative_deltas() {
        let values = vec![100i64, 95, 90, 85, 80];
        let encoded = encode_varint_deltas(&values);
        let decoded = decode_varint_deltas(&encoded, values.len()).unwrap();
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_roundtrip_mixed_signs() {
        let values = vec![-10i64, 5, -3, 20, -15, 0, 100];
        let encoded = encode_varint_deltas(&values);
        let decoded = decode_varint_deltas(&encoded, values.len()).unwrap();
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_small_deltas_produce_fewer_bytes() {
        // Monotonically increasing by 1: deltas are all 1, zigzag = 2, varint = 1 byte each
        let values: Vec<i64> = (0..100).collect();
        let encoded = encode_varint_deltas(&values);
        // First value (0) = 1 byte, each delta (1) = 1 byte = 100 bytes total
        assert!(encoded.len() < 200);
    }
}
