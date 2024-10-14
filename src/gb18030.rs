//! GB18030
use crate::{fetch_byte, make_decode_func, DecoderReturn};
use core::cmp::Ordering;

#[derive(Clone)]
pub struct Gb18030Decoder<'a> {
    data: &'a [u8],
    idx: usize,
}

decoding_iter_macros::array_from_index_file!("indexes/index-gb18030.txt");

decoding_iter_macros::array_from_index_ranges_file!(
    "indexes/index-gb18030-ranges.txt"
);

#[inline]
fn gb18030_ranges_lower_bound(pointer: u32) -> (u32, u32) {
    let data: &[(u32, u32)] = INDEX_GB18030_RANGES;
    let mut low = 0;
    let mut high = data.len() - 1;
    while low < high {
        let mid = low + (high - low) / 2;
        let x = data[mid];

        match x.0.cmp(&pointer) {
            Ordering::Greater => high = mid - 1,
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return x,
        }
    }
    let l1 = data[low - 1];
    let l2 = data[low];
    if l1.0 <= pointer && pointer < l2.0 {
        l1
    } else {
        l2
    }
}

#[test]
fn test_ranges() {
    assert_eq!(gb18030_ranges_lower_bound(0), (0, 0x80));
    assert_eq!(gb18030_ranges_lower_bound(1), (0, 0x80));
    assert_eq!(gb18030_ranges_lower_bound(35), (0, 0x80));
    assert_eq!(gb18030_ranges_lower_bound(36), (36, 0xa5));
    assert_eq!(gb18030_ranges_lower_bound(37), (36, 0xa5));
    assert_eq!(gb18030_ranges_lower_bound(38), (38, 0xa9));
    assert_eq!(gb18030_ranges_lower_bound(188999), (39394, 0xffe6));
    assert_eq!(gb18030_ranges_lower_bound(189000), (189000, 0x10000));
    assert_eq!(gb18030_ranges_lower_bound(189001), (189000, 0x10000));
}

impl<'a> Gb18030Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data, idx: 0 } }

    /// step 5
    #[inline]
    fn handle_2nd(&mut self, gb18030_first: u8) -> Option<DecoderReturn> {
        let byte = fetch_byte!(self, 1);
        self.idx += 1;
        if matches!(byte, 0x30..=0x39) {
            return self.handle_3rd(gb18030_first, byte); // step 5-1
        }
        let lead = gb18030_first as u32; // step 5-2
        let offset = if byte < 0x7f { 0x40 } else { 0x41 }; // step 5-3
        let ch = if matches!(byte, 0x40..=0x7e) || matches!(byte, 0x80..=0xfe) {
            let pointer = (lead - 0x81) * 190 + (byte as u32 - offset); // step 5-4
            get_char_from_index_gb18030(pointer) // step 5-5
        } else {
            None
        };
        if ch.is_some() {
            // step 5-6
            return Some((2, ch));
        }
        if byte.is_ascii() {
            // step 5-7
            self.idx -= 1;
            return Some((1, None));
        }
        Some((2, None)) // step 5-8
    }

    /// step 4
    #[inline]
    fn handle_3rd(
        &mut self,
        gb18030_first: u8,
        gb18030_second: u8,
    ) -> Option<DecoderReturn> {
        let byte = fetch_byte!(self, 2);
        self.idx += 1;
        if matches!(byte, 0x81..=0xfe) {
            return self.handle_4th(gb18030_first, gb18030_second, byte); // step 4-1
        }

        // step 4-2
        self.idx -= 2;
        Some((1, None))
    }

    /// step 3
    #[inline]
    fn handle_4th(
        &mut self,
        gb18030_first: u8,
        gb18030_second: u8,
        gb18030_third: u8,
    ) -> Option<DecoderReturn> {
        let byte = fetch_byte!(self, 3);
        self.idx += 1;

        if !matches!(byte, 0x30..=0x39) {
            // step 3-1
            self.idx -= 3;
            return Some((1, None));
        }

        // step 3-2
        let pointer = ((gb18030_first as u32 - 0x81) * (10 * 126 * 10))
            + ((gb18030_second as u32 - 0x30) * (10 * 126))
            + ((gb18030_third as u32 - 0x81) * 10)
            + byte as u32
            - 0x30;

        if 39419 < pointer && pointer < 189000 || 1237575 < pointer {
            return Some((4, None));
        }
        if pointer == 7457 {
            return Some((4, core::char::from_u32(0xe7c7)));
        }
        let offset = gb18030_ranges_lower_bound(pointer);
        // println!("pointer={pointer}, offset={offset:?}");
        let code_point_offset = offset.1 + (pointer - offset.0);

        let ch = core::char::from_u32(code_point_offset);
        Some((4, ch)) // step 3-5
    }
}

impl<'a> Iterator for Gb18030Decoder<'a> {
    type Item = DecoderReturn;

    fn next(&mut self) -> Option<Self::Item> {
        // https://encoding.spec.whatwg.org/#gb18030-decoder
        let byte = *self.data.get(self.idx)?; // step 2
        self.idx += 1;
        match byte {
            0x00..=0x7F => Some((1, Some(byte as char))), // step 6
            0x80 => Some((1, core::char::from_u32(0x20ac))), // step 7
            0x81..=0xfe => self.handle_2nd(byte),         // step 8
            _ => Some((1, None)),                         // step 9
        }
    }
}

make_decode_func!(
    Gb18030Decoder,
    decode_from_gb18030,
    decode_from_gb18030_lossy
);
