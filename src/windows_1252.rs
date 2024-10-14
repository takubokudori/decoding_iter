//! Windows-1252
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-windows-1252.txt");

single_byte_decoder!(Windows1252Decoder, get_char_from_index_windows_1252);

make_decode_func!(
    Windows1252Decoder,
    decode_from_windows_1252,
    decode_from_windows_1252_lossy
);
