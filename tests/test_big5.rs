#[macro_use]
mod utils;

#[cfg(test)]
#[cfg(feature = "big5")]
mod test_big5 {
    use decoding_iter::big5::Big5Decoder as Decoder;

    #[test]
    fn test_decode() {
        assert_iter!(
            Decoder::new(b"a\x81\x81\x87\x40\x88\x62x"),
            &[
                (1, Some('a')),
                (2, None),
                (2, Some('䏰')),
                (2, Some(core::char::from_u32(0x00ca).unwrap())),
                (0, Some(core::char::from_u32(0x0304).unwrap())),
                (1, Some('x')),
            ]
        );
    }

    #[test]
    fn test_decode_invalid_bytes() {
        assert_iter!(Decoder::new(b"\x81"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\xff"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\x81@"), &[(1, None), (1, Some('@'))]);
        assert_iter!(Decoder::new(b"\xffX"), &[(1, None), (1, Some('X'))]);
        assert_iter!(Decoder::new(b"\x81\x81"), &[(2, None)]);
    }
}
