//! Windows-1257
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-windows-1257.txt");

single_byte_decoder!(Windows1257Decoder, get_char_from_index_windows_1257);

make_decode_func!(
    Windows1257Decoder,
    decode_from_windows_1257,
    decode_from_windows_1257_lossy
);
