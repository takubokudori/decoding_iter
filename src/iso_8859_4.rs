//! ISO 8859-4
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-iso-8859-4.txt");

single_byte_decoder!(Iso8859_4Decoder, get_char_from_index_iso_8859_4);

make_decode_func!(
    Iso8859_4Decoder,
    decode_from_iso_8859_4,
    decode_from_iso_8859_4_lossy
);
