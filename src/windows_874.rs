//! Windows-874
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!("indexes/index-windows-874.txt");

single_byte_decoder!(Windows874Decoder, get_char_from_index_windows_874);

make_decode_func!(
    Windows874Decoder,
    decode_from_windows_874,
    decode_from_windows_874_lossy
);
