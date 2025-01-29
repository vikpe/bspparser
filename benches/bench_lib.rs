use std::fs;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

fn lib_benchmark(c: &mut Criterion) {
    let path = "tests/files/dm3_gpl.bsp";
    let file = &mut fs::File::open(path).unwrap();
    let filesize = fs::metadata(path).unwrap().len();
    let mut g = c.benchmark_group("lib");
    g.throughput(Throughput::Bytes(filesize));
    g.bench_function("parse", |b| b.iter(|| bspparser::bsp::BspFile::parse(file)));
    g.finish();
}

criterion_group!(benches, lib_benchmark);
criterion_main!(benches);
