//! ISO 8859-5
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-iso-8859-5.txt");

single_byte_decoder!(Iso8859_5Decoder, get_char_from_index_iso_8859_5);

make_decode_func!(
    Iso8859_5Decoder,
    decode_from_iso_8859_5,
    decode_from_iso_8859_5_lossy
);
