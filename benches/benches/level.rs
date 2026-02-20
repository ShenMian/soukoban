use std::fs;

use criterion::{Criterion, criterion_group};
use soukoban::{Forward, Level};

fn load_from_str(c: &mut Criterion) {
    c.bench_function("Level::load_from_str", |b| {
        let mut buf = String::new();
        for entry in fs::read_dir("assets/").unwrap() {
            let path = entry.unwrap().path();
            buf += &(fs::read_to_string(path).unwrap() + "\n\n");
        }
        b.iter(|| Level::<Forward>::load_from_str(&buf).count())
    });
}

fn load_nth_from_str(c: &mut Criterion) {
    c.bench_function("Level::load_nth_from_str", |b| {
        let mut buf = String::new();
        for entry in fs::read_dir("assets/").unwrap() {
            let path = entry.unwrap().path();
            buf += &(fs::read_to_string(path).unwrap() + "\n\n");
        }
        b.iter(|| Level::<Forward>::load_nth_from_str(&buf, 3371).unwrap())
    });
}

criterion_group!(benches, load_from_str, load_nth_from_str);
