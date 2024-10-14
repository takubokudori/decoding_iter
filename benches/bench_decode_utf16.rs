#![cfg(feature = "utf-16")]

use criterion::{criterion_group, criterion_main, Criterion};

fn decode(data: &[u8]) {
    // let (_s, _) = encoding_rs::UTF_16LE.decode_without_bom_handling(data);
    // let mut _s = String::with_capacity(data.len() * 2);
    // unsafe {
    //     for x in core::char::decode_utf16(data.chunks_exact(2).map(|x| {
    //         ((*x.get_unchecked(0) as u16) << 8) + (*x.get_unchecked(1) as u16)
    //     })) {
    //         _s.push(x.unwrap_or(REPLACEMENT_CHARACTER));
    //     }
    // }
    let _s = decoding_iter::utf_16::decode_from_utf_16_lossy(data, false);
}

fn decode_utf16_simple(c: &mut Criterion) {
    let data = &[
        0x53, 0x30, 0x93, 0x30, 0x6e, 0x00, 0x69, 0x00, 0x61, 0x30, 0x00, 0xdb,
        0x68, 0x00, 0x00, 0xdb, 0x61, 0x00, 0x3c, 0xd8, 0x63, 0xdf,
    ];
    c.bench_function("decode_utf16_simple", |b| b.iter(|| decode(data)));
}

fn decode_utf16_long_ascii(c: &mut Criterion) {
    let mut data = Vec::with_capacity(1024 * 1024); // 1MB
    for _ in 0..(1024 * 512) {
        data.push(0x61);
        data.push(0x00);
    }
    c.bench_function("decode_utf16_long_ascii", |b| b.iter(|| decode(&data)));
}

fn decode_utf16_long_2b(c: &mut Criterion) {
    let mut data = Vec::with_capacity(1024 * 1024); // 1MB
    for _ in 0..(1024 * 512) {
        data.push(0x53);
        data.push(0x30);
    }
    c.bench_function("decode_utf16_long_2b", |b| b.iter(|| decode(&data)));
}

fn decode_utf16_long_invalid_2b(c: &mut Criterion) {
    let mut data = Vec::with_capacity(1024 * 1024); // 1MB
    for _ in 0..(1024 * 512) {
        data.push(0x00);
        data.push(0xdb);
    }
    c.bench_function("decode_utf16_long_invalid_2b", |b| {
        b.iter(|| decode(&data))
    });
}

criterion_group!(
    bench_decode_utf16,
    decode_utf16_simple,
    decode_utf16_long_ascii,
    decode_utf16_long_2b,
    decode_utf16_long_invalid_2b,
);

criterion_main!(bench_decode_utf16);
