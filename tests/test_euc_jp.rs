#[macro_use]
mod utils;

#[cfg(test)]
#[cfg(feature = "euc-jp")]
mod test_euc_jp {
    use decoding_iter::euc_jp::EucJpDecoder as Decoder;

    #[test]
    fn test_decode() {
        assert_iter!(
            Decoder::new(b"AB\xa4\xb3\x8f\xb0\xa1"),
            &[
                (1, Some('A')),
                (1, Some('B')),
                (2, Some('こ')),
                (3, Some('丂'))
            ]
        );
    }

    #[test]
    fn test_decode_invalid_bytes() {
        assert_iter!(Decoder::new(b"\xff"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\x8e"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\x8e@"), &[(1, None), (1, Some('@'))]);
        assert_iter!(Decoder::new(b"\x8e\xa0"), &[(2, None)]);
        assert_iter!(Decoder::new(b"\x8f\xa1"), &[(2, None)]);
        assert_iter!(Decoder::new(b"\x8f\xa1\xff"), &[(3, None)]);
        assert_iter!(Decoder::new(b"\x8f\xa1\xa1"), &[(3, None)]);
        assert_iter!(
            Decoder::new(b"\x8f\xa1\x30"),
            &[(2, None), (1, Some('0'))]
        );
        assert_iter!(Decoder::new(b"\xffX"), &[(1, None), (1, Some('X'))]);
        assert_iter!(Decoder::new(b"\x81\x5b"), &[(1, None), (1, Some('['))]);
        assert_iter!(Decoder::new(b"\xc6\xa0"), &[(2, None)]);
    }
}
