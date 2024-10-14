//! ISO-2022-JP
use crate::{make_decode_func, DecoderReturn};

#[derive(Copy, Clone)]
enum Iso2022JpState {
    Ascii,
    Roman,
    Katakana,
    LeadByte,
    TrailByte,
    EscapeStart,
    Escape,
}

#[derive(Clone)]
pub struct Iso2022JpDecoder<'a> {
    data: &'a [u8],
    idx: usize,
    iso_2022_jp_decoder_state: Iso2022JpState,
    iso_2022_jp_decoder_output_state: Iso2022JpState,
    iso_2022_jp_output: bool,
}

decoding_iter_macros::array_from_index_file!("indexes/index-jis0208.txt");

impl<'a> Iso2022JpDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            idx: 0,
            iso_2022_jp_decoder_state: Iso2022JpState::Ascii,
            iso_2022_jp_decoder_output_state: Iso2022JpState::Ascii,
            iso_2022_jp_output: false,
        }
    }

    /// step ASCII
    #[inline]
    fn handle_ascii(&mut self, processed: usize) -> Option<DecoderReturn> {
        let byte = *self.data.get(self.idx)?;
        self.idx += 1;

        match byte {
            0x1b => {
                self.iso_2022_jp_decoder_state = Iso2022JpState::EscapeStart;
                self.handle_escape_start(processed + 1)
            }
            0x0e | 0x0f => {
                self.iso_2022_jp_output = false;
                Some((processed, None))
            }
            0x00..=0x7f => {
                self.iso_2022_jp_output = false;
                Some((processed, Some(byte as char)))
            }
            _ => {
                self.iso_2022_jp_output = false;
                Some((processed, None))
            }
        }
    }

    /// step Roman
    #[inline]
    fn handle_roman(&mut self, processed: usize) -> Option<DecoderReturn> {
        let byte = *self.data.get(self.idx)?;
        self.idx += 1;

        match byte {
            0x1b => {
                self.iso_2022_jp_decoder_state = Iso2022JpState::EscapeStart;
                self.handle_escape_start(processed + 1)
            }
            0x5c => {
                self.iso_2022_jp_output = false;
                Some((processed, core::char::from_u32(0xa5)))
            }
            0x7e => {
                self.iso_2022_jp_output = false;
                Some((processed, core::char::from_u32(0x203e)))
            }
            0x0e | 0x0f => {
                self.iso_2022_jp_output = false;
                Some((processed, None))
            }
            0x00..=0x7f => {
                self.iso_2022_jp_output = false;
                Some((processed, Some(byte as char)))
            }
            _ => {
                self.iso_2022_jp_output = false;
                Some((processed, None))
            }
        }
    }

    /// step katakana
    #[inline]
    fn handle_katakana(&mut self, processed: usize) -> Option<DecoderReturn> {
        let byte = *self.data.get(self.idx)?;
        self.idx += 1;

        match byte {
            0x1b => {
                self.iso_2022_jp_decoder_state = Iso2022JpState::EscapeStart;
                self.handle_escape_start(processed + 1)
            }
            0x21..=0x5f => {
                self.iso_2022_jp_output = false;
                Some((
                    processed,
                    core::char::from_u32(0xff61 - 0x21 + byte as u32),
                ))
            }
            _ => {
                self.iso_2022_jp_output = false;
                Some((processed, None))
            }
        }
    }

    /// step lead byte
    #[inline]
    fn handle_lead_byte(&mut self, processed: usize) -> Option<DecoderReturn> {
        let byte = *self.data.get(self.idx)?;
        self.idx += 1;

        match byte {
            0x1b => {
                self.iso_2022_jp_decoder_state = Iso2022JpState::EscapeStart;
                self.handle_escape_start(processed + 1)
            }
            0x21..=0x7e => {
                self.iso_2022_jp_output = false;
                self.iso_2022_jp_decoder_state = Iso2022JpState::TrailByte;
                self.handle_trail_byte(processed + 1, byte)
            }
            _ => {
                self.iso_2022_jp_output = false;
                Some((processed, None))
            }
        }
    }

    /// step trail byte
    #[inline]
    fn handle_trail_byte(
        &mut self,
        processed: usize,
        iso_2022_jp_lead: u8,
    ) -> Option<DecoderReturn> {
        let byte = match self.data.get(self.idx).copied() {
            Some(x) => x,
            None => {
                self.iso_2022_jp_decoder_state = Iso2022JpState::LeadByte;
                return Some((processed, None));
            }
        };
        self.idx += 1;

        match byte {
            0x1b => {
                self.iso_2022_jp_decoder_state = Iso2022JpState::EscapeStart;
                Some((processed, None))
            }
            0x21..=0x7e => {
                self.iso_2022_jp_decoder_state = Iso2022JpState::LeadByte;
                let pointer =
                    (iso_2022_jp_lead as u32 - 0x21) * 94 + byte as u32 - 0x21;
                let ch = get_char_from_index_jis0208(pointer);
                Some((processed, ch))
            }
            _ => {
                self.iso_2022_jp_decoder_state = Iso2022JpState::LeadByte;
                Some((processed, None))
            }
        }
    }

    /// step escape start
    #[inline]
    fn handle_escape_start(
        &mut self,
        processed: usize,
    ) -> Option<DecoderReturn> {
        if let Some(byte) = self.data.get(self.idx).copied() {
            if byte == 0x24 || byte == 0x28 {
                // 0x24 = '$', 0x28 = '('
                self.idx += 1;
                self.iso_2022_jp_decoder_state = Iso2022JpState::Escape;
                return self.handle_escape(processed + 1, byte);
            }
        }
        self.iso_2022_jp_output = false;
        self.iso_2022_jp_decoder_state = self.iso_2022_jp_decoder_output_state;
        Some((processed - 1, None))
    }

    /// step escape
    #[inline]
    fn handle_escape(
        &mut self,
        processed: usize,
        iso_2022_jp_lead: u8,
    ) -> Option<DecoderReturn> {
        let byte = match self.data.get(self.idx).copied() {
            Some(x) => x,
            None => {
                self.idx -= 1;
                self.iso_2022_jp_decoder_state =
                    self.iso_2022_jp_decoder_output_state;
                return Some((processed - 2, None));
            }
        };
        self.idx += 1;
        let lead = iso_2022_jp_lead as u32;

        let mut state = None;
        if lead == 0x28 {
            // 0x28 = '('
            match byte {
                0x42 => state = Some(Iso2022JpState::Ascii), // 0x42 = 'B'
                0x4a => state = Some(Iso2022JpState::Roman), // 0x4a = 'J'
                0x49 => state = Some(Iso2022JpState::Katakana), // 0x49 = 'I'
                _ => {}
            }
        } else if lead == 0x24 && (byte == 0x40 || byte == 0x42) {
            // 0x24 = '$', 0x40 = '@', 0x42 = 'B'
            state = Some(Iso2022JpState::LeadByte);
        }
        if let Some(state) = state {
            self.iso_2022_jp_decoder_state = state;
            self.iso_2022_jp_decoder_output_state = state;
            let output = self.iso_2022_jp_output;
            self.iso_2022_jp_output = true;
            if output {
                return Some((processed, None));
            }
            return match self.iso_2022_jp_decoder_state {
                Iso2022JpState::Ascii => self.handle_ascii(processed + 1),
                Iso2022JpState::Roman => self.handle_roman(processed + 1),
                Iso2022JpState::Katakana => self.handle_katakana(processed + 1),
                Iso2022JpState::LeadByte => {
                    self.handle_lead_byte(processed + 1)
                }
                _ => unreachable!(),
            };
        }
        self.idx -= 2;
        self.iso_2022_jp_decoder_state = self.iso_2022_jp_decoder_output_state;
        self.iso_2022_jp_output = false;
        Some((processed - 2, None))
    }
}

impl<'a> Iterator for Iso2022JpDecoder<'a> {
    type Item = DecoderReturn;

    fn next(&mut self) -> Option<Self::Item> {
        // https://encoding.spec.whatwg.org/#iso-2022-jp-decoder
        match self.iso_2022_jp_decoder_state {
            Iso2022JpState::Ascii => self.handle_ascii(1),
            Iso2022JpState::Roman => self.handle_roman(1),
            Iso2022JpState::Katakana => self.handle_katakana(1),
            Iso2022JpState::LeadByte => self.handle_lead_byte(1),
            Iso2022JpState::TrailByte => unreachable!(),
            Iso2022JpState::EscapeStart => self.handle_escape_start(1),
            Iso2022JpState::Escape => unreachable!(),
        }
    }
}

make_decode_func!(
    Iso2022JpDecoder,
    decode_from_iso_2022_jp,
    decode_from_iso_2022_jp_lossy
);
