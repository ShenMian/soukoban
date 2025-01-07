use criterion::criterion_main;

mod benches;

criterion_main!(
    benches::level::benches,
    benches::path_finding::benches,
    benches::deadlock::benches,
    benches::map::benches,
    benches::solver::benches
);
