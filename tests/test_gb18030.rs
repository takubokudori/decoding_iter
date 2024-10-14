#[macro_use]
mod utils;

#[cfg(test)]
#[cfg(feature = "gb18030")]
mod test_gb18030 {
    use decoding_iter::gb18030::Gb18030Decoder as Decoder;

    #[test]
    fn test_decode() {
        assert_iter!(
            Decoder::new(b"a\x81\x35\xf4\x37"),
            &[(1, Some('a')), (4, core::char::from_u32(0xe7c7))]
        );
        assert_iter!(
            Decoder::new(b"a\x80\x81\x40"),
            &[(1, Some('a')), (1, Some('€')), (2, Some('丂'))]
        );
        assert_iter!(
            Decoder::new(b"a\x80\x81\x40"),
            &[(1, Some('a')), (1, Some('€')), (2, Some('丂'))]
        );
        assert_iter!(
            Decoder::new(b"\x81\x30\x81\x30a"),
            &[(4, core::char::from_u32(0x80)), (1, Some('a')),]
        );
        assert_iter!(
            Decoder::new(b"\x81\x30\x81\x31a"),
            &[(4, core::char::from_u32(0x81)), (1, Some('a')),]
        );
    }

    #[test]
    fn test_decode_invalid_bytes() {
        assert_iter!(Decoder::new(b"\xff"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\xffX"), &[(1, None), (1, Some('X'))]);
        assert_iter!(Decoder::new(b"\x81"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\x81\x30"), &[(2, None)]);
        assert_iter!(Decoder::new(b"\x81\xff"), &[(2, None)]);
        assert_iter!(
            Decoder::new(b"\x81\x30\xff"),
            &[(1, None), (1, Some('0')), (1, None)]
        );
        assert_iter!(Decoder::new(b"\x81\x30\x81"), &[(3, None)]);
        assert_iter!(
            Decoder::new(b"\x81\x30\x81\x50"),
            &[(1, None), (1, Some('0')), (2, Some('丳'))]
        );
    }
}
