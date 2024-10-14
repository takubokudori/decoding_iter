//! EUC-JP
use crate::{fetch_byte, make_decode_func, DecoderReturn};

decoding_iter_macros::array_from_index_file!("indexes/index-jis0208.txt");

decoding_iter_macros::array_from_index_file!("indexes/index-jis0212.txt");

#[derive(Clone)]
pub struct EucJpDecoder<'a> {
    data: &'a [u8],
    idx: usize,
}

impl<'a> EucJpDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data, idx: 0 } }

    // step 5 (euc_jp_jis0212 is false)
    #[inline]
    fn handle_2nd(&mut self, euc_jp_lead: u8) -> Option<DecoderReturn> {
        let byte = fetch_byte!(self, 1);
        self.idx += 1;
        // println!("euc_jp_lead={euc_jp_lead}, byte={byte}, false");

        if euc_jp_lead == 0x8e && matches!(byte, 0xa1..=0xdf) {
            return Some((
                2,
                core::char::from_u32(0xff61 - 0xa1 + byte as u32),
            ));
        }
        if euc_jp_lead == 0x8f && matches!(byte, 0xa1..=0xfe) {
            return self.handle_3rd(byte);
        }

        let lead = euc_jp_lead as u32;
        let ch = if matches!(lead, 0xa1..=0xfe) && matches!(byte, 0xa1..=0xfe) {
            let pointer = (lead - 0xa1) * 94 + byte as u32 - 0xa1;
            get_char_from_index_jis0208(pointer)
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

    #[inline]
    fn handle_3rd(&mut self, euc_jp_lead: u8) -> Option<DecoderReturn> {
        let byte = fetch_byte!(self, 2);
        self.idx += 1;

        let lead = euc_jp_lead as u32;
        let ch = if matches!(lead, 0xa1..=0xfe) && matches!(byte, 0xa1..=0xfe) {
            let pointer = (lead - 0xa1) * 94 + byte as u32 - 0xa1;
            get_char_from_index_jis0212(pointer)
        } else {
            None
        };
        if ch.is_some() {
            return Some((3, ch));
        }
        if byte.is_ascii() {
            self.idx -= 1;
            return Some((2, None));
        }
        Some((3, None))
    }
}

impl<'a> Iterator for EucJpDecoder<'a> {
    type Item = DecoderReturn;

    fn next(&mut self) -> Option<Self::Item> {
        // https://encoding.spec.whatwg.org/#euc-jp-decoder
        let byte = *self.data.get(self.idx)?;
        self.idx += 1;
        match byte {
            0x00..=0x7f => Some((1, Some(byte as char))),
            0x8e | 0x8f | 0xa1..=0xfe => self.handle_2nd(byte),
            _ => Some((1, None)),
        }
    }
}

make_decode_func!(EucJpDecoder, decode_from_euc_jp, decode_from_euc_jp_lossy);
