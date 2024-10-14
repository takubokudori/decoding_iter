#[cfg(feature = "big5")]
pub mod big5;
#[cfg(feature = "euc-jp")]
pub mod euc_jp;
#[cfg(feature = "euc-kr")]
pub mod euc_kr;
#[cfg(feature = "gb18030")]
pub mod gb18030;
#[cfg(feature = "gbk")]
pub mod gbk;
#[cfg(feature = "ibm866")]
pub mod ibm866;
#[cfg(feature = "iso-2022-jp")]
pub mod iso_2022_jp;
#[cfg(feature = "iso-8859-10")]
pub mod iso_8859_10;
#[cfg(feature = "iso-8859-13")]
pub mod iso_8859_13;
#[cfg(feature = "iso-8859-14")]
pub mod iso_8859_14;
#[cfg(feature = "iso-8859-15")]
pub mod iso_8859_15;
#[cfg(feature = "iso-8859-16")]
pub mod iso_8859_16;
#[cfg(feature = "iso-8859-2")]
pub mod iso_8859_2;
#[cfg(feature = "iso-8859-3")]
pub mod iso_8859_3;
#[cfg(feature = "iso-8859-4")]
pub mod iso_8859_4;
#[cfg(feature = "iso-8859-5")]
pub mod iso_8859_5;
#[cfg(feature = "iso-8859-6")]
pub mod iso_8859_6;
#[cfg(feature = "iso-8859-7")]
pub mod iso_8859_7;
#[cfg(feature = "iso-8859-8")]
pub mod iso_8859_8;
#[cfg(feature = "koi8-r")]
pub mod koi8_r;
#[cfg(feature = "koi8-u")]
pub mod koi8_u;
#[cfg(feature = "macintosh")]
pub mod macintosh;
#[cfg(feature = "shift-jis")]
pub mod shift_jis;
#[cfg(feature = "utf-16")]
pub mod utf_16;
#[cfg(feature = "utf-8")]
pub mod utf_8;
#[cfg(feature = "windows-1250")]
pub mod windows_1250;
#[cfg(feature = "windows-1251")]
pub mod windows_1251;
#[cfg(feature = "windows-1252")]
pub mod windows_1252;
#[cfg(feature = "windows-1253")]
pub mod windows_1253;
#[cfg(feature = "windows-1254")]
pub mod windows_1254;
#[cfg(feature = "windows-1255")]
pub mod windows_1255;
#[cfg(feature = "windows-1256")]
pub mod windows_1256;
#[cfg(feature = "windows-1257")]
pub mod windows_1257;
#[cfg(feature = "windows-1258")]
pub mod windows_1258;
#[cfg(feature = "windows-874")]
pub mod windows_874;
#[cfg(feature = "x-mac-cyrillic")]
pub mod x_mac_cyrillic;

pub type DecodeResult<T> = Result<T, DecodeError>;

/// DecodeIter returns `(offset, UTF-8 character)`.
///
/// 1st value is the byte length, 2nd value is the decoded character.
/// 2nd value is `None` if invalid bytes.
pub type DecoderReturn = (usize, Option<char>);

#[derive(Debug, Clone)]
pub struct DecodeError {
    pub valid_up_to: usize,
    pub valid_utf8: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DecodeEncoding {
    ShiftJis,
    Utf16Le,
    Utf16Be,
}

#[macro_export]
macro_rules! make_decode_func {
    ($iter:ident, $decode_fn:ident, $decode_lossy_fn:ident) => {
        pub fn $decode_fn(data: &[u8]) -> $crate::DecodeResult<String> {
            let mut valid_up_to = 0;
            let mut ret = String::with_capacity(data.len() * 2);
            for (size, ch) in $iter::new(data) {
                match ch {
                    Some(ch) => ret.push(ch),
                    None => {
                        return Err($crate::DecodeError {
                            valid_up_to,
                            valid_utf8: ret,
                        })
                    }
                }
                valid_up_to += size;
            }
            Ok(ret)
        }

        pub fn $decode_lossy_fn(data: &[u8]) -> String {
            let mut ret = String::with_capacity(data.len() * 2);
            for (_, ch) in $iter::new(data) {
                ret.push(ch.unwrap_or(core::char::REPLACEMENT_CHARACTER));
            }
            ret
        }
    };
}

macro_rules! fetch_byte {
    ($self:ident, $nb:expr) => {{
        match $self.data.get($self.idx).copied() {
            Some(x) => x,
            None => {
                return Some(($nb, None));
            }
        }
    }};
}

macro_rules! single_byte_decoder {
    ($name:ident, $func:ident) => {
        #[derive(Clone)]
        pub struct $name<'a> {
            data: &'a [u8],
            idx: usize,
        }

        impl<'a> $name<'a> {
            pub fn new(data: &'a [u8]) -> Self { Self { data, idx: 0 } }
        }

        impl<'a> Iterator for $name<'a> {
            type Item = $crate::DecoderReturn;

            fn next(&mut self) -> Option<Self::Item> {
                // https://encoding.spec.whatwg.org/#single-byte-decoder
                let byte = *self.data.get(self.idx)?;
                self.idx += 1;
                if byte & 0x80 == 0 {
                    Some((1, Some(byte as char)))
                } else {
                    Some((1, $func((byte & 0x7f) as u32)))
                }
            }
        }
    };
}

pub(crate) use fetch_byte;

pub(crate) use single_byte_decoder;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum Encoding {
    #[cfg(feature = "big5")]
    Big5,
    #[cfg(feature = "euc-jp")]
    EUC_JP,
    #[cfg(feature = "euc-kr")]
    EUC_KR,
    #[cfg(feature = "gb18030")]
    GB18030,
    #[cfg(feature = "gbk")]
    GBK,
    #[cfg(feature = "ibm866")]
    IBM866,
    #[cfg(feature = "iso-2022-jp")]
    ISO_2022_JP,
    #[cfg(feature = "iso-8859-2")]
    ISO_8859_2,
    #[cfg(feature = "iso-8859-3")]
    ISO_8859_3,
    #[cfg(feature = "iso-8859-4")]
    ISO_8859_4,
    #[cfg(feature = "iso-8859-5")]
    ISO_8859_5,
    #[cfg(feature = "iso-8859-6")]
    ISO_8859_6,
    #[cfg(feature = "iso-8859-7")]
    ISO_8859_7,
    #[cfg(feature = "iso-8859-8")]
    ISO_8859_8,
    #[cfg(feature = "iso-8859-8")]
    ISO_8859_8_I,
    #[cfg(feature = "iso-8859-10")]
    ISO_8859_10,
    #[cfg(feature = "iso-8859-13")]
    ISO_8859_13,
    #[cfg(feature = "iso-8859-14")]
    ISO_8859_14,
    #[cfg(feature = "iso-8859-15")]
    ISO_8859_15,
    #[cfg(feature = "iso-8859-16")]
    ISO_8859_16,
    #[cfg(feature = "koi8-r")]
    KOI8_R,
    #[cfg(feature = "koi8-u")]
    KOI8_U,
    #[cfg(feature = "macintosh")]
    Macintosh,
    #[cfg(feature = "shift-jis")]
    Shift_JIS,
    #[cfg(feature = "utf-8")]
    UTF_8,
    #[cfg(feature = "utf-16")]
    UTF_16LE,
    #[cfg(feature = "utf-16")]
    UTF_16BE,
    #[cfg(feature = "windows-874")]
    Windows_874,
    #[cfg(feature = "windows-1250")]
    Windows_1250,
    #[cfg(feature = "windows-1251")]
    Windows_1251,
    #[cfg(feature = "windows-1252")]
    Windows_1252,
    #[cfg(feature = "windows-1253")]
    Windows_1253,
    #[cfg(feature = "windows-1254")]
    Windows_1254,
    #[cfg(feature = "windows-1255")]
    Windows_1255,
    #[cfg(feature = "windows-1256")]
    Windows_1256,
    #[cfg(feature = "windows-1257")]
    Windows_1257,
    #[cfg(feature = "windows-1258")]
    Windows_1258,
    #[cfg(feature = "x-mac-cyrillic")]
    x_mac_cyrillic,
}

impl Encoding {
    #[allow(unreachable_code)]
    pub fn from_label(x: &str) -> Option<Self> {
        let x = x.trim().to_ascii_lowercase();
        Some(match x.as_str() {
            #[cfg(feature = "big5")]
            "big5" | "big5-hkscs" | "cn-big5" | "csbig5" | "x-x-big5" => {
                Self::Big5
            }
            #[cfg(feature = "euc-jp")]
            "cseucpkdfmtjapanese" | "euc-jp" | "x-euc-jp" => Self::EUC_JP,
            #[cfg(feature = "euc-kr")]
            "cseuckr" | "csksc56011987" | "euc-kr" | "iso-ir-149"
            | "korean" | "ks_c_5601-1987" | "ks_c_5601-1989" | "ksc5601"
            | "ksc_5601" | "windows-949" => Self::EUC_KR,
            #[cfg(feature = "gb18030")]
            "gb18030" => Self::GB18030,
            #[cfg(feature = "gbk")]
            "chinese" | "csgb2312" | "csiso58gb231280" | "gb2312"
            | "gb_2312" | "gb_2312-80" | "gbk" | "iso-ir-58" | "x-gbk" => {
                Self::GBK
            }
            #[cfg(feature = "ibm866")]
            "866" | "cp866" | "csibm866" | "ibm866" => Self::IBM866,
            #[cfg(feature = "iso-2022-jp")]
            "csiso2022jp" | "iso-2022-jp" => Self::ISO_2022_JP,
            #[cfg(feature = "iso-8859-2")]
            "csisolatin2" | "iso-8859-2" | "iso-ir-101" | "iso8859-2"
            | "iso88592" | "iso_8859-2" | "iso_8859-2:1987" | "l2"
            | "latin2" => Self::ISO_8859_2,
            #[cfg(feature = "iso-8859-3")]
            "csisolatin3" | "iso-8859-3" | "iso-ir-109" | "iso8859-3"
            | "iso88593" | "iso_8859-3" | "iso_8859-3:1988" | "l3"
            | "latin3" => Self::ISO_8859_3,
            #[cfg(feature = "iso-8859-4")]
            "csisolatin4" | "iso-8859-4" | "iso-ir-110" | "iso8859-4"
            | "iso88594" | "iso_8859-4" | "iso_8859-4:1988" | "l4"
            | "latin4" => Self::ISO_8859_4,
            #[cfg(feature = "iso-8859-5")]
            "csisolatincyrillic" | "cyrillic" | "iso-8859-5" | "iso-ir-144"
            | "iso8859-5" | "iso88595" | "iso_8859-5" | "iso_8859-5:1988" => {
                Self::ISO_8859_5
            }
            #[cfg(feature = "iso-8859-6")]
            "arabic" | "asmo-708" | "csiso88596e" | "csiso88596i"
            | "csisolatinarabic" | "ecma-114" | "iso-8859-6"
            | "iso-8859-6-e" | "iso-8859-6-i" | "iso-ir-127" | "iso8859-6"
            | "iso88596" | "iso_8859-6" | "iso_8859-6:1987" => Self::ISO_8859_6,
            #[cfg(feature = "iso-8859-7")]
            "csisolatingreek" | "ecma-118" | "elot_928" | "greek"
            | "greek8" | "iso-8859-7" | "iso-ir-126" | "iso8859-7"
            | "iso88597" | "iso_8859-7" | "iso_8859-7:1987"
            | "sun_eu_greek" => Self::ISO_8859_7,
            #[cfg(feature = "iso-8859-8")]
            "csiso88598e" | "csisolatinhebrew" | "hebrew" | "iso-8859-8"
            | "iso-8859-8-e" | "iso-ir-138" | "iso8859-8" | "iso88598"
            | "iso_8859-8" | "iso_8859-8:1988" | "visual" => Self::ISO_8859_8,
            #[cfg(feature = "iso-8859-8")]
            "csiso88598i" | "iso-8859-8-i" | "logical" => Self::ISO_8859_8_I,
            #[cfg(feature = "iso-8859-10")]
            "csisolatin6" | "iso-8859-10" | "iso-ir-157" | "iso8859-10"
            | "iso885910" | "l6" | "latin6" => Self::ISO_8859_10,
            #[cfg(feature = "iso-8859-13")]
            "iso-8859-13" | "iso8859-13" | "iso885913" => Self::ISO_8859_13,
            #[cfg(feature = "iso-8859-14")]
            "iso-8859-14" | "iso8859-14" | "iso885914" => Self::ISO_8859_14,
            #[cfg(feature = "iso-8859-15")]
            "csisolatin9" | "iso-8859-15" | "iso8859-15" | "iso885915"
            | "iso_8859-15" | "l9" => Self::ISO_8859_15,
            #[cfg(feature = "iso-8859-16")]
            "iso-8859-16" => Self::ISO_8859_16,
            #[cfg(feature = "koi8-r")]
            "cskoi8r" | "koi" | "koi8" | "koi8-r" | "koi8_r" => Self::KOI8_R,
            #[cfg(feature = "koi8-u")]
            "koi8-ru" | "koi8-u" => Self::KOI8_U,
            #[cfg(feature = "macintosh")]
            "csmacintosh" | "mac" | "macintosh" | "x-mac-roman" => {
                Self::Macintosh
            }
            #[cfg(feature = "shift-jis")]
            "csshiftjis" | "ms932" | "ms_kanji" | "shift-jis" | "shift_jis"
            | "sjis" | "windows-31j" | "x-sjis" => Self::Shift_JIS,
            #[cfg(feature = "utf-8")]
            "unicode-1-1-utf-8" | "unicode11utf8" | "unicode20utf8"
            | "utf-8" | "utf8" | "x-unicode20utf8" => Self::UTF_8,
            #[cfg(feature = "utf-16")]
            "unicodefffe" | "utf-16be" => Self::UTF_16BE,
            #[cfg(feature = "utf-16")]
            "csunicode" | "iso-10646-ucs-2" | "ucs-2" | "unicode"
            | "unicodefeff" | "utf-16" | "utf-16le" => Self::UTF_16LE,
            #[cfg(feature = "windows-874")]
            "dos-874" | "iso-8859-11" | "iso8859-11" | "iso885911"
            | "tis-620" | "windows-874" => Self::Windows_874,
            #[cfg(feature = "windows-1250")]
            "cp1250" | "windows-1250" | "x-cp1250" => Self::Windows_1250,
            #[cfg(feature = "windows-1251")]
            "cp1251" | "windows-1251" | "x-cp1251" => Self::Windows_1251,
            #[cfg(feature = "windows-1252")]
            "ansi_x3.4-1968" | "ascii" | "cp1252" | "cp819" | "csisolatin1"
            | "ibm819" | "iso-8859-1" | "iso-ir-100" | "iso8859-1"
            | "iso88591" | "iso_8859-1" | "iso_8859-1:1987" | "l1"
            | "latin1" | "us-ascii" | "windows-1252" | "x-cp1252" => {
                Self::Windows_1252
            }
            #[cfg(feature = "windows-1253")]
            "cp1253" | "windows-1253" | "x-cp1253" => Self::Windows_1253,
            #[cfg(feature = "windows-1254")]
            "cp1254" | "csisolatin5" | "iso-8859-9" | "iso-ir-148"
            | "iso8859-9" | "iso88599" | "iso_8859-9" | "iso_8859-9:1989"
            | "l5" | "latin5" | "windows-1254" | "x-cp1254" => {
                Self::Windows_1254
            }
            #[cfg(feature = "windows-1255")]
            "cp1255" | "windows-1255" | "x-cp1255" => Self::Windows_1255,
            #[cfg(feature = "windows-1256")]
            "cp1256" | "windows-1256" | "x-cp1256" => Self::Windows_1256,
            #[cfg(feature = "windows-1257")]
            "cp1257" | "windows-1257" | "x-cp1257" => Self::Windows_1257,
            #[cfg(feature = "windows-1258")]
            "cp1258" | "windows-1258" | "x-cp1258" => Self::Windows_1258,
            #[cfg(feature = "x-mac-cyrillic")]
            "x-mac-cyrillic" | "x-mac-ukrainian" => Self::x_mac_cyrillic,
            _ => return None,
        })
    }
}
