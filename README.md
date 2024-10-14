# decoding_iter

decoding_iter provides a decoding iterator for character encodings.

The implementation of the encoding algorithm for this crate is based on [Encoding Standard](https://encoding.spec.whatwg.org/).
Therefore, the encoding algorithm is subject to the WHATWG BSD 3-Clause License.
All other parts are subject to the MIT or Apache License.

# Examples

```rust
fn main() {
    let data = &[0x69, 0x82, 0xbf, 0xca, 0x84, 0xfa, 0x03];
    let mut it = decoding_iter::shift_jis::ShiftJisDecoder::new(data);
    for (nof_byte, ch) in it {
        println!("{nof_byte}, {ch}");
    }
}
```

# Licence

(MIT or Apache-2.0 License) and [WHATWG BSD 3-Clause License](https://encoding.spec.whatwg.org/#ipr).
