#[macro_use]
mod utils;

#[cfg(test)]
#[cfg(feature = "utf-8")]
mod test_utf_8 {
    use decoding_iter::utf_8::Utf8Decoder as Decoder;

    #[test]
    fn test_decode() {
        assert_iter!(
            Decoder::new("ABあ😀☺".as_bytes()),
            &[
                (1, Some('A')),
                (1, Some('B')),
                (3, Some('あ')),
                (4, Some('😀')),
                (3, Some('☺'))
            ]
        );
    }

    #[test]
    fn test_decode_invalid_bytes() {
        assert_iter!(Decoder::new(b"\xff"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\xc2\x40"), &[(1, None), (1, Some('@'))]);
        assert_iter!(Decoder::new(b"\xc2"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\xe2"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\xe2\x86"), &[(2, None)]);
        assert_iter!(
            Decoder::new(b"\xe2\x86\x40"),
            &[(2, None), (1, Some('@'))]
        );
        assert_iter!(Decoder::new(b"\xf0"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\xf0\x40"), &[(1, None), (1, Some('@'))]);
        assert_iter!(Decoder::new(b"\xf0\x90"), &[(2, None)]);
        assert_iter!(
            Decoder::new(b"\xf0\x90\x40"),
            &[(2, None), (1, Some('@'))]
        );
        assert_iter!(Decoder::new(b"\xf0\x90\x80"), &[(3, None)]);
        assert_iter!(
            Decoder::new(b"\xf0\x90\x80\x40"),
            &[(3, None), (1, Some('@'))]
        );
    }
}
