#![no_main]
use decoding_iter::gb18030::decode_from_gb18030_lossy;
use libfuzzer_sys::fuzz_target;

fn decode_gb18030(data: &[u8]) {
    let s1 = decode_from_gb18030_lossy(data);
    let (s2, _) = encoding_rs::GB18030.decode_without_bom_handling(data);
    assert_eq!(s1.as_str(), s2);
}

fuzz_target!(|data: &[u8]| {
    decode_gb18030(data);
});
