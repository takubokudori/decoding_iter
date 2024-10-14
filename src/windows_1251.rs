//! Windows-1251
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-windows-1251.txt");

single_byte_decoder!(Windows1251Decoder, get_char_from_index_windows_1251);

make_decode_func!(
    Windows1251Decoder,
    decode_from_windows_1251,
    decode_from_windows_1251_lossy
);
