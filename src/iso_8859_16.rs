//! ISO 8859-16
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-iso-8859-16.txt");

single_byte_decoder!(Iso8859_16Decoder, get_char_from_index_iso_8859_16);

make_decode_func!(
    Iso8859_16Decoder,
    decode_from_iso_8859_16,
    decode_from_iso_8859_16_lossy
);
