//! ISO 8859-2
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-iso-8859-2.txt");

single_byte_decoder!(Iso8859_2Decoder, get_char_from_index_iso_8859_2);

make_decode_func!(
    Iso8859_2Decoder,
    decode_from_iso_8859_2,
    decode_from_iso_8859_2_lossy
);
