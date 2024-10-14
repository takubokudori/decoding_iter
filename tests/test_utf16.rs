#[macro_use]
mod utils;

#[cfg(test)]
#[cfg(feature = "utf-16")]
mod test_utf16 {
    use decoding_iter::utf_16::Utf16Decoder as Decoder;

    #[test]
    fn decode_utf16le_simple() {
        let data = &[
            0x69, 0x00, 0x61, 0x30, 0x00, 0xdb, 0x68, 0x00, 0x00, 0xdb, 0x3c,
            0xd8, 0x63, 0xdf,
        ];
        assert_iter!(
            Decoder::new(data, false),
            &[
                (2, Some('i')),
                (2, Some('ち')),
                (2, None),
                (2, Some('h')),
                (2, None),
                (4, Some('🍣')),
            ]
        );
    }

    #[test]
    fn decode_utf16le_incomplete_bytes() {
        let data = &[0x3c, 0xd8, 0x63, 0xdf];
        assert_iter!(Decoder::new(&data[..1], false), &[(1, None)]);
        assert_iter!(Decoder::new(&data[..2], false), &[(2, None)]);
        assert_iter!(Decoder::new(&data[..3], false), &[(3, None)]);
        assert_iter!(Decoder::new(data, false), &[(4, Some('🍣'))]);
    }
}
