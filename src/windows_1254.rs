//! Windows-1254
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-windows-1254.txt");

single_byte_decoder!(Windows1254Decoder, get_char_from_index_windows_1254);

make_decode_func!(
    Windows1254Decoder,
    decode_from_windows_1254,
    decode_from_windows_1254_lossy
);
