# delta-encode

A pure-Rust library for delta encoding and its variants — fixed delta, variable-length delta, XOR delta, zigzag encoding, and prediction-based encoding.

## Features

- **Fixed delta** — Encode differences between consecutive values
- **Variable-length delta** — Efficient encoding of small deltas using varint
- **XOR delta** — XOR-based encoding ideal for floating-point data
- **Zigzag encoding** — Map signed integers to unsigned for efficient varint encoding
- **Prediction-based** — Encode the difference between actual and predicted values

## Usage

```rust
use delta_encode::{delta, zigzag};

let values = vec![10, 15, 20, 18, 25];
let deltas = delta::encode(&values);
let decoded = delta::decode(&deltas, values[0]);
assert_eq!(decoded, values);

let zigzagged = zigzag::encode_i64(-5);
assert_eq!(zigzag::decode_i64(zigzagged), -5);
```

## Modules

- `delta` — Fixed delta encoding/decoding
- `vdelta` — Variable-length delta encoding
- `xor` — XOR-based delta encoding
- `zigzag` — Zigzag encoding for signed integers
- `predict` — Prediction-based delta encoding

## License

MIT
