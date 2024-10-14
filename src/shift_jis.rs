//! Shift_JIS
use crate::{fetch_byte, make_decode_func, DecoderReturn};

#[derive(Clone)]
pub struct ShiftJisDecoder<'a> {
    data: &'a [u8],
    idx: usize,
}

decoding_iter_macros::array_from_index_file!("indexes/index-jis0208.txt");

impl<'a> ShiftJisDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data, idx: 0 } }

    /// step 3
    #[inline]
    fn handle_2nd(&mut self, shift_jis_lead: u8) -> Option<DecoderReturn> {
        let lead = shift_jis_lead as u32;
        let byte = fetch_byte!(self, 1);
        self.idx += 1;

        // step 3-1
        let offset: u32 = if byte < 0x7f { 0x40 } else { 0x41 };
        // step 3-2
        let lead_offset: u32 = if lead < 0xa0 { 0x81 } else { 0xc1 };
        // step 3-3
        let ch = if matches!(byte, 0x40..=0x7e|0x80..=0xfc) {
            let pointer = (lead - lead_offset) * 188 + byte as u32 - offset;
            if matches!(pointer, 8836..=10715) {
                char::from_u32(0xe000_u32 - 8836 + pointer) // step 3-4
            } else {
                get_char_from_index_jis0208(pointer) // step 3-5 (otherwise statement)
            }
        } else {
            None
        };
        if ch.is_some() {
            return Some((2, ch));
        }
        if byte.is_ascii() {
            // step 3-7
            self.idx -= 1;
            return Some((1, None));
        }
        // step 3-8
        Some((2, None))
    }
}

impl<'a> Iterator for ShiftJisDecoder<'a> {
    type Item = DecoderReturn;

    fn next(&mut self) -> Option<Self::Item> {
        // https://encoding.spec.whatwg.org/#shift_jis-decoder
        let byte = *self.data.get(self.idx)?; // step 2
        self.idx += 1;
        match byte {
            0x00..=0x80 => Some((1, Some(byte as char))), // step 4
            0x81..=0x9f | 0xe0..=0xfc => self.handle_2nd(byte), // step 6
            0xa1..=0xdf => {
                // step 5
                let code_point = 0xff61 - 0xa1 + byte as u32;
                Some((1, char::from_u32(code_point)))
            }
            _ => Some((1, None)),
        }
    }
}

make_decode_func!(
    ShiftJisDecoder,
    decode_from_shift_jis,
    decode_from_shift_jis_lossy
);
