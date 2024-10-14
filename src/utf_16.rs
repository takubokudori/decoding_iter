use crate::{DecodeError, DecodeResult, DecoderReturn};
use core::char::REPLACEMENT_CHARACTER;

#[derive(Clone)]
pub struct Utf16Decoder<'a> {
    data: &'a [u8],
    idx: usize,
    is_be: bool,
}

impl<'a> Utf16Decoder<'a> {
    pub fn new(data: &'a [u8], is_be: bool) -> Self {
        Self {
            data,
            idx: 0,
            is_be,
        }
    }

    #[inline]
    fn make_code_unit(is_be: bool, utf16_lead_byte: u8, byte: u8) -> u16 {
        if is_be {
            ((utf16_lead_byte as u16) << 8) + byte as u16
        } else {
            ((byte as u16) << 8) + utf16_lead_byte as u16
        }
    }

    #[inline]
    fn get_code_unit(&mut self) -> Result<u16, usize> {
        if self.data.len() > self.idx + 1 {
            unsafe {
                let utf16_lead_byte = *self.data.get_unchecked(self.idx);
                let byte = *self.data.get_unchecked(self.idx + 1);
                self.idx += 2;
                return Ok(Self::make_code_unit(
                    self.is_be,
                    utf16_lead_byte,
                    byte,
                ));
            }
        }
        let utf16_lead_byte = *self.data.get(self.idx).ok_or(0usize)?;
        self.idx += 1;
        // step 2
        let byte = *self.data.get(self.idx).ok_or(1usize)?;
        self.idx += 1;
        Ok(Self::make_code_unit(self.is_be, utf16_lead_byte, byte))
    }
}

impl<'a> Iterator for Utf16Decoder<'a> {
    type Item = DecoderReturn;

    fn next(&mut self) -> Option<Self::Item> {
        // https://encoding.spec.whatwg.org/#shared-utf-16-decoder
        // step 2/4
        let code_unit = match self.get_code_unit() {
            Ok(x) => x,
            Err(x) => return if x == 0 { None } else { Some((1, None)) },
        };

        match code_unit {
            0xd800..=0xdbff => {
                // step 6
                let lead_surrogate = code_unit as u32;
                // step 5
                let code_unit = match self.get_code_unit() {
                    Ok(x) => x,
                    Err(x) => return Some((x + 2, None)),
                };
                // step 5-1
                if matches!(code_unit, 0xdc00..=0xdfff) {
                    let code_point = 0x10000u32
                        + ((lead_surrogate - 0xd800) << 10)
                        + ((code_unit as u32) - 0xdc00);
                    return Some((4, char::from_u32(code_point)));
                }
                // step 5-2 to 5-5
                self.idx -= 2;
                Some((2, None))
            }
            0xdc00..=0xdfff => Some((2, None)),
            _ => Some((2, char::from_u32(code_unit as u32))),
        }
    }
}

pub fn decode_from_utf_16(data: &[u8], is_be: bool) -> DecodeResult<String> {
    let mut valid_up_to = 0;
    let mut ret = String::with_capacity(data.len() * 2);
    for (size, ch) in Utf16Decoder::new(data, is_be) {
        match ch {
            Some(ch) => ret.push(ch),
            None => {
                return Err(DecodeError {
                    valid_up_to,
                    valid_utf8: ret,
                });
            }
        }
        valid_up_to += size;
    }
    Ok(ret)
}

pub fn decode_from_utf_16_lossy(data: &[u8], is_be: bool) -> String {
    let mut ret = String::with_capacity(data.len() * 2);
    for (_, ch) in Utf16Decoder::new(data, is_be) {
        ret.push(ch.unwrap_or(REPLACEMENT_CHARACTER));
    }
    ret
}

pub fn decode_from_utf_16be_lossy(data: &[u8]) -> String {
    decode_from_utf_16_lossy(data, true)
}

pub fn decode_from_utf_16le_lossy(data: &[u8]) -> String {
    decode_from_utf_16_lossy(data, false)
}
