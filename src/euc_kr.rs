//! EUC-KR
use crate::{fetch_byte, make_decode_func, DecoderReturn};

#[derive(Clone)]
pub struct EucKrDecoder<'a> {
    data: &'a [u8],
    idx: usize,
}

decoding_iter_macros::array_from_index_file!("indexes/index-euc-kr.txt");

impl<'a> EucKrDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data, idx: 0 } }

    // step 3
    #[inline]
    fn handle_2nd(&mut self, lead: u8) -> Option<DecoderReturn> {
        let byte = fetch_byte!(self, 1);
        self.idx += 1;
        let ch = if matches!(byte, 0x41..=0xfe) {
            let pointer = (lead as u32 - 0x81) * 190 + (byte as u32 - 0x41);
            get_char_from_index_euc_kr(pointer)
        } else {
            None
        };
        if ch.is_some() {
            return Some((2, ch));
        }

        if byte.is_ascii() {
            self.idx -= 1;
            return Some((1, None));
        }

        Some((2, None))
    }
}

impl<'a> Iterator for EucKrDecoder<'a> {
    type Item = DecoderReturn;

    fn next(&mut self) -> Option<Self::Item> {
        // https://encoding.spec.whatwg.org/#euc-kr-decoder
        let byte = *self.data.get(self.idx)?;
        self.idx += 1;
        match byte {
            0x81..=0xfe => self.handle_2nd(byte),
            0x00..=0x7f => Some((1, Some(byte as char))),
            _ => Some((1, None)),
        }
    }
}

make_decode_func!(EucKrDecoder, decode_from_euc_kr, decode_from_euc_kr_lossy);
