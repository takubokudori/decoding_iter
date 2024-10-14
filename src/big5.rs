//! Big5
use crate::{fetch_byte, make_decode_func, DecoderReturn};

#[derive(Clone)]
pub struct Big5Decoder<'a> {
    data: &'a [u8],
    idx: usize,
    trail_ch: Option<char>,
}

decoding_iter_macros::array_from_index_file_u32!("indexes/index-big5.txt");

impl<'a> Big5Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            idx: 0,
            trail_ch: None,
        }
    }

    // step 3
    #[inline]
    fn handle_2nd(&mut self, big5_lead: u8) -> Option<DecoderReturn> {
        let byte = fetch_byte!(self, 1);
        let lead = big5_lead as u32;
        let offset = if byte < 0x7f { 0x40 } else { 0x62 }; // step 3-1
        self.idx += 1;
        let ch = if matches!(byte, 0x40..=0x7e) | matches!(byte, 0xa1..=0xfe) {
            let pointer = (lead - 0x81) * 157 + (byte as u32 - offset); // step 3-2
                                                                        // step 3-3
            match pointer {
                1133 => {
                    self.trail_ch = core::char::from_u32(0x0304);
                    core::char::from_u32(0x00ca)
                }
                1135 => {
                    self.trail_ch = core::char::from_u32(0x030c);
                    core::char::from_u32(0x00ca)
                }
                1164 => {
                    self.trail_ch = core::char::from_u32(0x0304);
                    core::char::from_u32(0x00ea)
                }
                1166 => {
                    self.trail_ch = core::char::from_u32(0x030c);
                    core::char::from_u32(0x00ea)
                }
                _ => {
                    // step 3-4
                    get_char_from_index_big5(pointer)
                }
            }
        } else {
            None
        };
        if ch.is_some() {
            // step 3-5
            return Some((2, ch));
        }

        if byte.is_ascii() {
            // step 3-6
            self.idx -= 1;
            return Some((1, None));
        }

        Some((2, None))
    }
}

impl<'a> Iterator for Big5Decoder<'a> {
    type Item = DecoderReturn;

    fn next(&mut self) -> Option<Self::Item> {
        // https://encoding.spec.whatwg.org/#big5-decoder
        if let Some(trail_ch) = self.trail_ch {
            self.trail_ch = None;
            return Some((0, Some(trail_ch)));
        }
        let byte = *self.data.get(self.idx)?;
        self.idx += 1;
        match byte {
            0x81..=0xfe => self.handle_2nd(byte),
            0x00..=0x7f => Some((1, Some(byte as char))),
            _ => Some((1, None)),
        }
    }
}

make_decode_func!(Big5Decoder, decode_from_big5, decode_from_big5_lossy);
