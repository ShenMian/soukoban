use std::fs;

use criterion::{black_box, criterion_group, Criterion};
use soukoban::Level;

fn load_from_str(c: &mut Criterion) {
    let mut buf = String::new();
    for entry in fs::read_dir("assets/").unwrap() {
        let path = entry.unwrap().path();
        buf += &(fs::read_to_string(path).unwrap() + "\n\n");
    }

    c.bench_function("Level::load_from_str", |b| {
        b.iter(|| black_box(Level::load_from_str(black_box(&buf)).count()))
    });
}

fn load_nth_from_str(c: &mut Criterion) {
    let mut buf = String::new();
    for entry in fs::read_dir("assets/").unwrap() {
        let path = entry.unwrap().path();
        buf += &(fs::read_to_string(path).unwrap() + "\n\n");
    }

    c.bench_function("Level::load_nth_from_str", |b| {
        b.iter(|| black_box(Level::load_nth_from_str(black_box(&buf), black_box(3371)).unwrap()))
    });
}

criterion_group!(benches, load_from_str, load_nth_from_str);
