//! Macintosh
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-macintosh.txt");

single_byte_decoder!(MacintoshDecoder, get_char_from_index_macintosh);

make_decode_func!(
    MacintoshDecoder,
    decode_from_macintosh,
    decode_from_macintosh_lossy
);
