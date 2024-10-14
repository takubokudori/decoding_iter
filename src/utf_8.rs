use crate::{fetch_byte, make_decode_func, DecoderReturn};

/// default UTF-8 lower boundary
const DEFAULT_U8LB: u8 = 0x80;
/// default UTF-8 upper boundary
const DEFAULT_U8UB: u8 = 0xbf;

#[derive(Clone)]
pub struct Utf8Decoder<'a> {
    data: &'a [u8],
    idx: usize,
}

impl<'a> Utf8Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data, idx: 0 } }

    #[inline]
    fn handle_2nd(
        &mut self,
        utf_8_bytes_needed: usize,
        mut utf_8_code_point: u32,
        mut utf_8_lower_boundary: u8,
        mut utf_8_upper_boundary: u8,
    ) -> Option<DecoderReturn> {
        let mut utf_8_bytes_seen = 0;
        while utf_8_bytes_seen != utf_8_bytes_needed {
            let byte = fetch_byte!(self, 1 + utf_8_bytes_seen);
            self.idx += 1;
            if !(utf_8_lower_boundary..=utf_8_upper_boundary).contains(&byte) {
                // step 4
                self.idx -= 1;
                return Some((utf_8_bytes_seen + 1, None));
            }
            utf_8_lower_boundary = DEFAULT_U8LB;
            utf_8_upper_boundary = DEFAULT_U8UB;
            utf_8_code_point = (utf_8_code_point << 6) | (byte as u32 & 0x3f);
            utf_8_bytes_seen += 1;
        }
        Some((
            utf_8_bytes_needed + 1,
            core::char::from_u32(utf_8_code_point),
        ))
    }
}

impl<'a> Iterator for Utf8Decoder<'a> {
    type Item = DecoderReturn;

    fn next(&mut self) -> Option<Self::Item> {
        // https://encoding.spec.whatwg.org/#utf-8-decoder
        let byte = *self.data.get(self.idx)?;
        self.idx += 1;
        match byte {
            0x00..=0x7f => Some((1, Some(byte as char))),
            0xc2..=0xdf => self.handle_2nd(
                1,
                byte as u32 & 0x1f,
                DEFAULT_U8LB,
                DEFAULT_U8UB,
            ),
            0xe0..=0xef => self.handle_2nd(
                2,
                byte as u32 & 0xf,
                if byte == 0xe0 { 0xa0 } else { DEFAULT_U8LB },
                if byte == 0xed { 0x9f } else { DEFAULT_U8UB },
            ),
            0xf0..=0xf4 => self.handle_2nd(
                3,
                byte as u32 & 0x7,
                if byte == 0xf0 { 0x90 } else { DEFAULT_U8LB },
                if byte == 0xf4 { 0x8f } else { DEFAULT_U8UB },
            ),
            _ => Some((1, None)),
        }
    }
}

make_decode_func!(Utf8Decoder, decode_from_utf_8, decode_from_utf_8_lossy);
