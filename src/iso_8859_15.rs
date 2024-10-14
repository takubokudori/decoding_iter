//! ISO 8859-15
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-iso-8859-15.txt");

single_byte_decoder!(Iso8859_15Decoder, get_char_from_index_iso_8859_15);

make_decode_func!(
    Iso8859_15Decoder,
    decode_from_iso_8859_15,
    decode_from_iso_8859_15_lossy
);
