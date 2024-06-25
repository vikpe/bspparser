use std::fs;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

fn get_demo_data() -> Vec<u8> {
    fs::read("tests/files/dm3_gpl.bsp").expect("unable to read map")
}

fn lib_benchmark(c: &mut Criterion) {
    let data = get_demo_data();
    let mut group = c.benchmark_group("lib");
    group.throughput(Throughput::Bytes(data.len() as u64));

    group.bench_function("entities::get_entities", |b| {
        b.iter(|| bspparser::entities::get_entities(&data))
    });

    group.bench_function("worldspawn::get_worldspawn_message", |b| {
        b.iter(|| bspparser::worldspawn::get_worldspawn_message(&data))
    });

    group.finish();
}

criterion_group!(benches, lib_benchmark);
criterion_main!(benches);
