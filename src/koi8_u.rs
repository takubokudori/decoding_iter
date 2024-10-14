//! KOI8-U
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-koi8-u.txt");

single_byte_decoder!(Koi8UDecoder, get_char_from_index_koi8_u);

make_decode_func!(Koi8UDecoder, decode_from_koi8_u, decode_from_koi8_u_lossy);
