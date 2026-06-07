//! XOR-based delta encoding for floating-point data.

pub struct XorDelta {
    /// Bit length of the XOR value
    pub xor_len: u8,
    /// The XOR value itself
    pub xor_value: u64,
}

/// Encode f64 values as XOR deltas.
/// Returns the first value as f64 bits, then XOR deltas.
pub fn encode_f64(values: &[f64]) -> (u64, Vec<XorDelta>) {
    if values.is_empty() {
        return (0, Vec::new());
    }

    let first_bits = values[0].to_bits();
    let mut deltas = Vec::with_capacity(values.len() - 1);

    for i in 1..values.len() {
        let prev_bits = values[i - 1].to_bits();
        let curr_bits = values[i].to_bits();
        let xor = prev_bits ^ curr_bits;
        let xor_len = if xor == 0 { 0 } else { 64 - xor.leading_zeros() as u8 };
        deltas.push(XorDelta {
            xor_len,
            xor_value: xor,
        });
    }

    (first_bits, deltas)
}

/// Decode XOR deltas back to f64 values.
pub fn decode_f64(first_bits: u64, deltas: &[XorDelta]) -> Vec<f64> {
    let mut values = Vec::with_capacity(deltas.len() + 1);
    let mut current_bits = first_bits;
    values.push(f64::from_bits(current_bits));

    for delta in deltas {
        current_bits ^= delta.xor_value;
        values.push(f64::from_bits(current_bits));
    }

    values
}

/// Encode f64 values with efficient XOR compression.
/// Stores leading/trailing zero counts to reduce stored bits.
#[derive(Debug, Clone)]
pub struct EfficientXorDelta {
    pub leading_zeros: u8,
    pub meaningful_bits: u8,
    pub xor_value: u64,
}

/// Encode with leading/trailing zero tracking (Gorilla-style).
pub fn encode_f64_efficient(values: &[f64]) -> (u64, Vec<EfficientXorDelta>) {
    if values.is_empty() {
        return (0, Vec::new());
    }

    let first_bits = values[0].to_bits();
    let mut deltas = Vec::with_capacity(values.len() - 1);

    for i in 1..values.len() {
        let prev_bits = values[i - 1].to_bits();
        let curr_bits = values[i].to_bits();
        let xor = prev_bits ^ curr_bits;

        let leading_zeros = if xor == 0 {
            64u8
        } else {
            xor.leading_zeros() as u8
        };
        let trailing_zeros = if xor == 0 {
            64u8
        } else {
            xor.trailing_zeros() as u8
        };
        let meaningful_bits = if xor == 0 {
            0
        } else {
            64 - leading_zeros - trailing_zeros
        };

        deltas.push(EfficientXorDelta {
            leading_zeros,
            meaningful_bits,
            xor_value: xor >> trailing_zeros,
        });
    }

    (first_bits, deltas)
}

/// XOR-encode u64 values.
pub fn encode_u64(values: &[u64]) -> Vec<u64> {
    if values.is_empty() {
        return Vec::new();
    }
    let mut result = vec![values[0]];
    for i in 1..values.len() {
        result.push(values[i] ^ values[i - 1]);
    }
    result
}

/// XOR-decode u64 values.
pub fn decode_u64(xor_deltas: &[u64]) -> Vec<u64> {
    if xor_deltas.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(xor_deltas.len());
    result.push(xor_deltas[0]);
    for i in 1..xor_deltas.len() {
        result.push(xor_deltas[i] ^ result[i - 1]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let (first, deltas) = encode_f64(&[]);
        assert!(deltas.is_empty());
        assert_eq!(first, 0);
    }

    #[test]
    fn test_f64_roundtrip() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let (first, deltas) = encode_f64(&values);
        let decoded = decode_f64(first, &deltas);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_f64_similar_values() {
        let values = vec![1.0, 1.001, 1.002, 1.003];
        let (first, deltas) = encode_f64(&values);
        let decoded = decode_f64(first, &deltas);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_f64_identical() {
        let values = vec![3.14; 10];
        let (_, deltas) = encode_f64(&values);
        assert!(deltas.iter().all(|d| d.xor_value == 0));
    }

    #[test]
    fn test_u64_xor_roundtrip() {
        let values = vec![100u64, 200, 150, 300];
        let encoded = encode_u64(&values);
        let decoded = decode_u64(&encoded);
        assert_eq!(decoded, values);
    }

    #[test]
    fn test_efficient_encoding() {
        let values = vec![1.0, 1.001, 1.002];
        let (_, deltas) = encode_f64_efficient(&values);
        // Similar values should have many leading/trailing zeros
        for delta in &deltas {
            if delta.meaningful_bits > 0 {
                // Should have some zero compression
            }
        }
        // Just verify no panic and correct structure
        assert_eq!(deltas.len(), 2);
    }
}
