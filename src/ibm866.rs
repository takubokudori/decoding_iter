//! IBM866
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-ibm866.txt");

single_byte_decoder!(Ibm866Decoder, get_char_from_index_ibm866);

make_decode_func!(Ibm866Decoder, decode_from_ibm866, decode_from_ibm866_lossy);
