#[macro_use]
mod utils;

#[cfg(test)]
#[cfg(feature = "euc-kr")]
mod test_euc_kr {
    use decoding_iter::euc_kr::EucKrDecoder as Decoder;

    #[test]
    fn test_decode() {
        assert_iter!(
            Decoder::new(b"a\xff\x81\x81d"),
            &[(1, Some('a')), (1, None), (2, Some('걖')), (1, Some('d'))]
        );
    }

    #[test]
    fn test_decode_invalid_bytes() {
        assert_iter!(Decoder::new(b"\x81"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\xff"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\x81@"), &[(1, None), (1, Some('@'))]);
        assert_iter!(Decoder::new(b"\xffX"), &[(1, None), (1, Some('X'))]);
        assert_iter!(Decoder::new(b"\x81\x5b"), &[(1, None), (1, Some('['))]);
        assert_iter!(Decoder::new(b"\xc6\xa0"), &[(2, None)]);
    }
}
