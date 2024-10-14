//! ISO 8859-7
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-iso-8859-7.txt");

single_byte_decoder!(Iso8859_7Decoder, get_char_from_index_iso_8859_7);

make_decode_func!(
    Iso8859_7Decoder,
    decode_from_iso_8859_7,
    decode_from_iso_8859_7_lossy
);
