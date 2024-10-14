//! x-mac-cyrillic
use crate::{make_decode_func, single_byte_decoder};

decoding_iter_macros::array_from_index_file!(
    "indexes/index-x-mac-cyrillic.txt"
);

single_byte_decoder!(XMacCyrillicDecoder, get_char_from_index_x_mac_cyrillic);

make_decode_func!(
    XMacCyrillicDecoder,
    decode_from_x_mac_cyrillic,
    decode_from_x_mac_cyrillic_lossy
);
