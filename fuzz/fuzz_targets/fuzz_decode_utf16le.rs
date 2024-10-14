#![no_main]
use decoding_iter::utf_16::decode_from_utf_16_lossy;
use libfuzzer_sys::fuzz_target;

fn decode_utf16le(data: &[u8]) {
    let s1 = decode_from_utf_16_lossy(data, false);
    let (s2, _) = encoding_rs::UTF_16LE.decode_without_bom_handling(data);
    assert_eq!(s1.as_str(), s2);
}

fuzz_target!(|data: &[u8]| {
    decode_utf16le(data);
});
