#[cfg(test)]
mod diff_test_decode {
    macro_rules! diff_test {
        ($t:ty, $etype:expr, $func:ident) => {
            for i in 0..=<$t>::MAX {
                let data = i.to_be_bytes();
                let actual = $func(&data);
                let (encoding_rs_output, _) =
                    $etype.decode_without_bom_handling(&data);
                assert_eq!(
                    actual, encoding_rs_output,
                    "Test failed. data is {data:x?}"
                );
            }
        };
        (data: $data:expr, $etype:expr, $func:ident) => {{
            let data = $data;
            let actual = $func(&data);
            let (encoding_rs_output, _) =
                $etype.decode_without_bom_handling(&data);
            assert_eq!(
                actual, encoding_rs_output,
                "Test failed. data is {data:x?}"
            );
        }};
    }

    #[test]
    #[cfg(feature = "big5")]
    fn test_big5() {
        use decoding_iter::big5::decode_from_big5_lossy;
        diff_test!(u16, encoding_rs::BIG5, decode_from_big5_lossy);
    }

    #[test]
    #[cfg(feature = "euc-jp")]
    #[ignore]
    fn test_euc_jp() {
        use decoding_iter::euc_jp::decode_from_euc_jp_lossy;
        diff_test!(u32, encoding_rs::EUC_JP, decode_from_euc_jp_lossy);
    }

    #[test]
    #[cfg(feature = "euc-kr")]
    fn test_euc_kr() {
        use decoding_iter::euc_kr::decode_from_euc_kr_lossy;
        diff_test!(u16, encoding_rs::EUC_KR, decode_from_euc_kr_lossy);
    }

    #[test]
    #[cfg(feature = "gb18030")]
    #[ignore]
    fn test_gb18030() {
        use decoding_iter::gb18030::decode_from_gb18030_lossy;
        diff_test!(u32, encoding_rs::GB18030, decode_from_gb18030_lossy);
    }

    /*
    // The differential test for GBK is omitted because GBK decoder == GB 18030 decoder.
    #[test]
    #[cfg(feature = "gbk")]
    #[ignore]
    fn test_gbk() {
        use decoding_iter::gbk::decode_from_gbk_lossy;
        diff_test!(u32, encoding_rs::GBK, decode_from_gbk_lossy);
    }
    */

    #[test]
    #[cfg(feature = "ibm866")]
    fn test_ibm866() {
        use decoding_iter::ibm866::decode_from_ibm866_lossy;
        diff_test!(u8, encoding_rs::IBM866, decode_from_ibm866_lossy);
    }

    #[test]
    #[cfg(feature = "iso-2022-jp")]
    #[ignore]
    fn test_iso_2022_jp() {
        use decoding_iter::iso_2022_jp::decode_from_iso_2022_jp_lossy;
        diff_test!(
            u32,
            encoding_rs::ISO_2022_JP,
            decode_from_iso_2022_jp_lossy
        );
    }

    #[test]
    #[cfg(feature = "iso-2022-jp")]
    #[ignore]
    fn test_iso_2022_jp2() {
        use decoding_iter::iso_2022_jp::decode_from_iso_2022_jp_lossy;

        let mut data = [b'\x1b', 0, 0, 0, 0];
        for d0 in [b'$', b'('] {
            for d1 in 0..=255 {
                for d2 in 0..=255 {
                    for d3 in 0..=255 {
                        unsafe {
                            *data.get_unchecked_mut(1) = d0;
                            *data.get_unchecked_mut(2) = d1;
                            *data.get_unchecked_mut(3) = d2;
                            *data.get_unchecked_mut(4) = d3;
                        }
                        let actual = decode_from_iso_2022_jp_lossy(&data);
                        let (encoding_rs_output, _) = encoding_rs::ISO_2022_JP
                            .decode_without_bom_handling(&data);
                        assert_eq!(
                            actual, encoding_rs_output,
                            "Test failed. data is {data:x?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    #[cfg(feature = "iso-8859-2")]
    fn test_iso_8859_2() {
        use decoding_iter::iso_8859_2::decode_from_iso_8859_2_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_2, decode_from_iso_8859_2_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-3")]
    fn test_iso_8859_3() {
        use decoding_iter::iso_8859_3::decode_from_iso_8859_3_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_3, decode_from_iso_8859_3_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-4")]
    fn test_iso_8859_4() {
        use decoding_iter::iso_8859_4::decode_from_iso_8859_4_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_4, decode_from_iso_8859_4_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-5")]
    fn test_iso_8859_5() {
        use decoding_iter::iso_8859_5::decode_from_iso_8859_5_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_5, decode_from_iso_8859_5_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-6")]
    fn test_iso_8859_6() {
        use decoding_iter::iso_8859_6::decode_from_iso_8859_6_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_6, decode_from_iso_8859_6_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-7")]
    fn test_iso_8859_7() {
        use decoding_iter::iso_8859_7::decode_from_iso_8859_7_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_7, decode_from_iso_8859_7_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-8")]
    fn test_iso_8859_8() {
        use decoding_iter::iso_8859_8::decode_from_iso_8859_8_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_8, decode_from_iso_8859_8_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-10")]
    fn test_iso_8859_10() {
        use decoding_iter::iso_8859_10::decode_from_iso_8859_10_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_10, decode_from_iso_8859_10_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-13")]
    fn test_iso_8859_13() {
        use decoding_iter::iso_8859_13::decode_from_iso_8859_13_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_13, decode_from_iso_8859_13_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-14")]
    fn test_iso_8859_14() {
        use decoding_iter::iso_8859_14::decode_from_iso_8859_14_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_14, decode_from_iso_8859_14_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-15")]
    fn test_iso_8859_15() {
        use decoding_iter::iso_8859_15::decode_from_iso_8859_15_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_15, decode_from_iso_8859_15_lossy);
    }

    #[test]
    #[cfg(feature = "iso-8859-16")]
    fn test_iso_8859_16() {
        use decoding_iter::iso_8859_16::decode_from_iso_8859_16_lossy;
        diff_test!(u8, encoding_rs::ISO_8859_16, decode_from_iso_8859_16_lossy);
    }

    #[test]
    #[cfg(feature = "koi8-r")]
    fn test_koi8_r() {
        use decoding_iter::koi8_r::decode_from_koi8_r_lossy;
        diff_test!(u8, encoding_rs::KOI8_R, decode_from_koi8_r_lossy);
    }

    #[test]
    #[cfg(feature = "koi8-u")]
    fn test_koi8_u() {
        use decoding_iter::koi8_u::decode_from_koi8_u_lossy;
        diff_test!(u8, encoding_rs::KOI8_U, decode_from_koi8_u_lossy);
    }

    #[test]
    #[cfg(feature = "macintosh")]
    fn test_macintosh() {
        use decoding_iter::macintosh::decode_from_macintosh_lossy;
        diff_test!(u8, encoding_rs::MACINTOSH, decode_from_macintosh_lossy);
    }

    #[test]
    #[cfg(feature = "shift-jis")]
    fn test_shift_jis() {
        use decoding_iter::shift_jis::decode_from_shift_jis_lossy;
        diff_test!(u16, encoding_rs::SHIFT_JIS, decode_from_shift_jis_lossy);
    }

    #[test]
    #[cfg(feature = "utf-8")]
    #[ignore]
    fn test_utf_8() {
        use decoding_iter::utf_8::decode_from_utf_8_lossy;
        diff_test!(u32, encoding_rs::UTF_8, decode_from_utf_8_lossy);
    }

    #[test]
    #[cfg(feature = "utf-16")]
    #[ignore]
    fn test_utf_16be() {
        use decoding_iter::utf_16::decode_from_utf_16be_lossy;
        diff_test!(u32, encoding_rs::UTF_16BE, decode_from_utf_16be_lossy);
    }

    #[test]
    #[cfg(feature = "utf-16")]
    #[ignore]
    fn test_utf_16le() {
        use decoding_iter::utf_16::decode_from_utf_16le_lossy;
        diff_test!(u32, encoding_rs::UTF_16LE, decode_from_utf_16le_lossy);
    }

    #[test]
    #[cfg(feature = "windows-874")]
    fn test_windows_874() {
        use decoding_iter::windows_874::decode_from_windows_874_lossy;
        diff_test!(u8, encoding_rs::WINDOWS_874, decode_from_windows_874_lossy);
    }

    #[test]
    #[cfg(feature = "windows-1250")]
    fn test_windows_1250() {
        use decoding_iter::windows_1250::decode_from_windows_1250_lossy;
        diff_test!(
            u8,
            encoding_rs::WINDOWS_1250,
            decode_from_windows_1250_lossy
        );
    }

    #[test]
    #[cfg(feature = "windows-1251")]
    fn test_windows_1251() {
        use decoding_iter::windows_1251::decode_from_windows_1251_lossy;
        diff_test!(
            u8,
            encoding_rs::WINDOWS_1251,
            decode_from_windows_1251_lossy
        );
    }

    #[test]
    #[cfg(feature = "windows-1252")]
    fn test_windows_1252() {
        use decoding_iter::windows_1252::decode_from_windows_1252_lossy;
        diff_test!(
            u8,
            encoding_rs::WINDOWS_1252,
            decode_from_windows_1252_lossy
        );
    }

    #[test]
    #[cfg(feature = "windows-1253")]
    fn test_windows_1253() {
        use decoding_iter::windows_1253::decode_from_windows_1253_lossy;
        diff_test!(
            u8,
            encoding_rs::WINDOWS_1253,
            decode_from_windows_1253_lossy
        );
    }

    #[test]
    #[cfg(feature = "windows-1254")]
    fn test_windows_1254() {
        use decoding_iter::windows_1254::decode_from_windows_1254_lossy;
        diff_test!(
            u8,
            encoding_rs::WINDOWS_1254,
            decode_from_windows_1254_lossy
        );
    }

    #[test]
    #[cfg(feature = "windows-1255")]
    fn test_windows_1255() {
        use decoding_iter::windows_1255::decode_from_windows_1255_lossy;
        diff_test!(
            u8,
            encoding_rs::WINDOWS_1255,
            decode_from_windows_1255_lossy
        );
    }

    #[test]
    #[cfg(feature = "windows-1256")]
    fn test_windows_1256() {
        use decoding_iter::windows_1256::decode_from_windows_1256_lossy;
        diff_test!(
            u8,
            encoding_rs::WINDOWS_1256,
            decode_from_windows_1256_lossy
        );
    }

    #[test]
    #[cfg(feature = "windows-1257")]
    fn test_windows_1257() {
        use decoding_iter::windows_1257::decode_from_windows_1257_lossy;
        diff_test!(
            u8,
            encoding_rs::WINDOWS_1257,
            decode_from_windows_1257_lossy
        );
    }

    #[test]
    #[cfg(feature = "windows-1258")]
    fn test_windows_1258() {
        use decoding_iter::windows_1258::decode_from_windows_1258_lossy;
        diff_test!(
            u8,
            encoding_rs::WINDOWS_1258,
            decode_from_windows_1258_lossy
        );
    }

    #[test]
    #[cfg(feature = "x-mac-cyrillic")]
    fn test_x_mac_cyrillic() {
        use decoding_iter::x_mac_cyrillic::decode_from_x_mac_cyrillic_lossy;
        diff_test!(
            u8,
            encoding_rs::X_MAC_CYRILLIC,
            decode_from_x_mac_cyrillic_lossy
        );
    }
}
