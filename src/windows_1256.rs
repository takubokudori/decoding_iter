//! Windows-1256
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-windows-1256.txt");

single_byte_decoder!(Windows1256Decoder, get_char_from_index_windows_1256);

make_decode_func!(
    Windows1256Decoder,
    decode_from_windows_1256,
    decode_from_windows_1256_lossy
);
