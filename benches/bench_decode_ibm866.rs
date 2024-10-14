#![cfg(feature = "ibm866")]

use criterion::{criterion_group, criterion_main, Criterion};

fn decode(data: &[u8]) {
    // let (_s, _) = encoding_rs::IBM866.decode_without_bom_handling(data);
    let _s = decoding_iter::ibm866::decode_from_ibm866_lossy(data);
}

fn decode_ibm866_long_ascii(c: &mut Criterion) {
    let data = &[0x61; 1024 * 1024];
    c.bench_function("decode_ibm866_long_ascii", |b| b.iter(|| decode(data)));
}

fn decode_ibm866_non_ascii(c: &mut Criterion) {
    let data = &[0xa0; 1024 * 1024];
    c.bench_function("decode_ibm866_non_ascii", |b| b.iter(|| decode(data)));
}

criterion_group!(
    bench_decode_ibm866,
    decode_ibm866_long_ascii,
    decode_ibm866_non_ascii,
);

criterion_main!(bench_decode_ibm866);
