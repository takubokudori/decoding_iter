//! Windows-1258
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-windows-1258.txt");

single_byte_decoder!(Windows1258Decoder, get_char_from_index_windows_1258);

make_decode_func!(
    Windows1258Decoder,
    decode_from_windows_1258,
    decode_from_windows_1258_lossy
);
