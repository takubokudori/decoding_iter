//! ISO 8859-8
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-iso-8859-8.txt");

single_byte_decoder!(Iso8859_8Decoder, get_char_from_index_iso_8859_8);

make_decode_func!(
    Iso8859_8Decoder,
    decode_from_iso_8859_8,
    decode_from_iso_8859_8_lossy
);
