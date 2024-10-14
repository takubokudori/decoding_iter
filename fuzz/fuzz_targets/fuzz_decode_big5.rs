#![no_main]
use decoding_iter::big5::decode_from_big5_lossy;
use libfuzzer_sys::fuzz_target;

fn decode_big5(data: &[u8]) {
    let s1 = decode_from_big5_lossy(data);
    let (s2, _) = encoding_rs::BIG5.decode_without_bom_handling(data);
    assert_eq!(s1.as_str(), s2);
}

fuzz_target!(|data: &[u8]| {
    decode_big5(data);
});
