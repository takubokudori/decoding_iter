//! KOI8-R
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-koi8-r.txt");

single_byte_decoder!(Koi8RDecoder, get_char_from_index_koi8_r);

make_decode_func!(Koi8RDecoder, decode_from_koi8_r, decode_from_koi8_r_lossy);
