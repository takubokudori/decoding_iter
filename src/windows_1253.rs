//! Windows-1253
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-windows-1253.txt");

single_byte_decoder!(Windows1253Decoder, get_char_from_index_windows_1253);

make_decode_func!(
    Windows1253Decoder,
    decode_from_windows_1253,
    decode_from_windows_1253_lossy
);
