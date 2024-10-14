#![no_main]
use decoding_iter::iso_2022_jp::decode_from_iso_2022_jp_lossy;
use libfuzzer_sys::fuzz_target;

fn decode_iso_2022_jp(data: &[u8]) {
    let s1 = decode_from_iso_2022_jp_lossy(data);
    let (s2, _) = encoding_rs::ISO_2022_JP.decode_without_bom_handling(data);
    assert_eq!(s1.as_str(), s2);
}

fuzz_target!(|data: &[u8]| {
    decode_iso_2022_jp(data);
});
