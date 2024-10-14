//! Windows-1255
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-windows-1255.txt");

single_byte_decoder!(Windows1255Decoder, get_char_from_index_windows_1255);

make_decode_func!(
    Windows1255Decoder,
    decode_from_windows_1255,
    decode_from_windows_1255_lossy
);
