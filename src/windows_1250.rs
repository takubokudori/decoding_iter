//! Windows-1250
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-windows-1250.txt");

single_byte_decoder!(Windows1250Decoder, get_char_from_index_windows_1250);

make_decode_func!(
    Windows1250Decoder,
    decode_from_windows_1250,
    decode_from_windows_1250_lossy
);
