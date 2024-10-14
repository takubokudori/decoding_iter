//! GBK
use crate::{gb18030::Gb18030Decoder, make_decode_func, DecoderReturn};

#[derive(Clone)]
pub struct GbkDecoder<'a> {
    gb18030decoder: Gb18030Decoder<'a>,
}

impl<'a> GbkDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            gb18030decoder: Gb18030Decoder::new(data),
        }
    }
}

impl<'a> Iterator for GbkDecoder<'a> {
    type Item = DecoderReturn;

    fn next(&mut self) -> Option<Self::Item> {
        // https://encoding.spec.whatwg.org/#gbk-decoder
        self.gb18030decoder.next()
    }
}

make_decode_func!(GbkDecoder, decode_from_gbk, decode_from_gbk_lossy);
