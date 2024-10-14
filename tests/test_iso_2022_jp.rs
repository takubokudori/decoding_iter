#[macro_use]
mod utils;

#[cfg(test)]
#[cfg(feature = "iso-2022-jp")]
mod test_iso_2022_jp {
    use decoding_iter::iso_2022_jp::Iso2022JpDecoder as Decoder;

    #[test]
    fn test_decode() {
        let data = b"a\x1b$B$3\x1b(Bxyz";
        assert_iter!(
            Decoder::new(data),
            &[
                (1, Some('a')),
                (5, Some('こ')),
                (4, Some('x')),
                (1, Some('y')),
                (1, Some('z')),
            ]
        );

        assert_iter!(Decoder::new(b"\x1b$@"), &[]);
    }

    #[test]
    fn test_decode_invalid_bytes() {
        assert_iter!(Decoder::new(b"\x1b"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\x1b$"), &[(1, None), (1, Some('$'))]);
        assert_iter!(Decoder::new(b"\x1b("), &[(1, None), (1, Some('('))]);
        assert_iter!(Decoder::new(b"\x1bX"), &[(1, None), (1, Some('X'))]);
        assert_iter!(Decoder::new(b"\x1b$@D"), &[(5, None)]);
    }
}
