//! Fixed delta encoding and decoding.

pub fn encode_u64(values: &[u64]) -> Vec<i64> {
    if values.is_empty() {
        return Vec::new();
    }
    let mut deltas = Vec::with_capacity(values.len());
    deltas.push(values[0] as i64); // base value
    for i in 1..values.len() {
        deltas.push(values[i] as i64 - values[i - 1] as i64);
    }
    deltas
}

/// Decode delta-encoded u64 values back to original values.
pub fn decode_u64(deltas: &[i64]) -> Vec<u64> {
    if deltas.is_empty() {
        return Vec::new();
    }
    let mut values = Vec::with_capacity(deltas.len());
    let mut current = deltas[0];
    values.push(current as u64);
    for &delta in &deltas[1..] {
        current += delta;
        values.push(current as u64);
    }
    values
}

/// Encode a sequence of i64 values as deltas.
pub fn encode_i64(values: &[i64]) -> Vec<i64> {
    if values.is_empty() {
        return Vec::new();
    }
    let mut deltas = Vec::with_capacity(values.len());
    deltas.push(values[0]);
    for i in 1..values.len() {
        deltas.push(values[i] - values[i - 1]);
    }
    deltas
}

/// Decode delta-encoded i64 values back to original values.
pub fn decode_i64(deltas: &[i64]) -> Vec<i64> {
    if deltas.is_empty() {
        return Vec::new();
    }
    let mut values = Vec::with_capacity(deltas.len());
    let mut current = deltas[0];
    values.push(current);
    for &delta in &deltas[1..] {
        current += delta;
        values.push(current);
    }
    values
}

/// Encode bytes as deltas (treating each byte as a value).
pub fn encode_bytes(data: &[u8]) -> Vec<i16> {
    if data.is_empty() {
        return Vec::new();
    }
    let mut deltas = Vec::with_capacity(data.len());
    deltas.push(data[0] as i16);
    for i in 1..data.len() {
        deltas.push(data[i] as i16 - data[i - 1] as i16);
    }
    deltas
}

/// Decode byte deltas back to bytes.
pub fn decode_bytes(deltas: &[i16]) -> Vec<u8> {
    if deltas.is_empty() {
        return Vec::new();
    }
    let mut values = Vec::with_capacity(deltas.len());
    let mut current = deltas[0];
    values.push(current as u8);
    for &delta in &deltas[1..] {
        current += delta;
        values.push(current as u8);
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert!(encode_u64(&[]).is_empty());
        assert!(decode_u64(&[]).is_empty());
    }

    #[test]
    fn test_u64_roundtrip() {
        let values = vec![10u64, 15, 20, 18, 25];
        let deltas = encode_u64(&values);
        let decoded = decode_u64(&deltas);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_u64_constant() {
        let values = vec![42u64; 5];
        let deltas = encode_u64(&values);
        assert_eq!(deltas[0], 42);
        assert!(deltas[1..].iter().all(|&d| d == 0));
    }

    #[test]
    fn test_i64_roundtrip() {
        let values = vec![-10i64, 5, -3, 20, -15];
        let deltas = encode_i64(&values);
        let decoded = decode_i64(&deltas);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_bytes_roundtrip() {
        let data = b"abcdef";
        let deltas = encode_bytes(data);
        let decoded = decode_bytes(&deltas);
        assert_eq!(decoded, data.to_vec());
    }

    #[test]
    fn test_deltas_small_for_smooth_data() {
        let values: Vec<u64> = (0..100).map(|i| i as u64 * 2).collect();
        let deltas = encode_u64(&values);
        assert!(deltas[1..].iter().all(|&d| d == 2));
    }
}
