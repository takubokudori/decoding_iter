//! ISO 8859-14
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-iso-8859-14.txt");

single_byte_decoder!(Iso8859_14Decoder, get_char_from_index_iso_8859_14);

make_decode_func!(
    Iso8859_14Decoder,
    decode_from_iso_8859_14,
    decode_from_iso_8859_14_lossy
);
