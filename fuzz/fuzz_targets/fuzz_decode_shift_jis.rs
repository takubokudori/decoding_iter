#![no_main]
use decoding_iter::shift_jis::decode_from_shift_jis_lossy;
use libfuzzer_sys::fuzz_target;

fn decode_shift_jis(data: &[u8]) {
    let s1 = decode_from_shift_jis_lossy(data);
    let (s2, _) = encoding_rs::SHIFT_JIS.decode_without_bom_handling(data);
    assert_eq!(s1.as_str(), s2);
}

fuzz_target!(|data: &[u8]| {
    decode_shift_jis(data);
});
