#![cfg(feature = "shift-jis")]

use criterion::{criterion_group, criterion_main, Criterion};

fn decode(data: &[u8]) {
    // let (_s, _) = encoding_rs::SHIFT_JIS.decode_without_bom_handling(data);
    let _s = decoding_iter::shift_jis::decode_from_shift_jis_lossy(data);
}

fn decode_shift_jis_simple(c: &mut Criterion) {
    let data = &[
        0x82, 0xb1, 0x82, 0xf1, 0x6e, 0x69, 0x82, 0xbf, 0xfc, 0x68, 0x84, 0xfa,
        0x61, 0x03,
    ];
    c.bench_function("decode_shift_jis_simple", |b| b.iter(|| decode(data)));
}

fn decode_shift_jis_long_ascii(c: &mut Criterion) {
    let data = vec![0x61; 1024 * 1024];

    c.bench_function("decode_shift_jis_long_ascii", |b| {
        b.iter(|| decode(&data))
    });
}

fn decode_shift_jis_long_mb(c: &mut Criterion) {
    let mut data = Vec::with_capacity(1024 * 1024); // 1MB
    for _ in 0..(1024 * 512) {
        data.push(0x82);
        data.push(0xb1);
    }
    c.bench_function("decode_shift_jis_long_mb", |b| b.iter(|| decode(&data)));
}

fn decode_shift_jis_long_invalid_mb(c: &mut Criterion) {
    let mut data = Vec::with_capacity(1024 * 1024); // 1MB
    for _ in 0..(1024 * 512) {
        data.push(0x82);
        data.push(0xff);
    }
    c.bench_function("decode_shift_jis_long_invalid_mb", |b| {
        b.iter(|| decode(&data))
    });
}

criterion_group!(
    bench_decode_shift_jis,
    decode_shift_jis_simple,
    decode_shift_jis_long_ascii,
    decode_shift_jis_long_mb,
    decode_shift_jis_long_invalid_mb,
);

criterion_main!(bench_decode_shift_jis);
