//! ISO 8859-3
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-iso-8859-3.txt");

single_byte_decoder!(Iso8859_3Decoder, get_char_from_index_iso_8859_3);

make_decode_func!(
    Iso8859_3Decoder,
    decode_from_iso_8859_3,
    decode_from_iso_8859_3_lossy
);
