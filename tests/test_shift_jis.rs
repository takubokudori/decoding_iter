#[macro_use]
mod utils;

#[cfg(test)]
#[cfg(feature = "shift-jis")]
mod test_shift_jis {
    use decoding_iter::shift_jis::ShiftJisDecoder as Decoder;

    #[test]
    fn test_decode() {
        let data = &[0x69, 0x82, 0xbf, 0xca, 0x84, 0xfa, 0x03];
        assert_iter!(
            Decoder::new(data),
            &[
                (1, Some('i')),
                (2, Some('ち')),
                (1, Some('ﾊ')),
                (2, None),
                (1, Some('\x03')),
            ]
        );
    }

    #[test]
    fn test_decode_invalid_bytes() {
        assert_iter!(Decoder::new(b"\xff"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\x82"), &[(1, None)]);
        assert_iter!(Decoder::new(b"\x82\x30"), &[(1, None), (1, Some('0'))]);
        assert_iter!(Decoder::new(b"\x82\x80"), &[(2, None)]);
    }
}
