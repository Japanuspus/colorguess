#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate colorguess;

fn criterion_benchmark(c: &mut Criterion) {
    let all = colorguess::build_all_configs();
    c.bench_function("count all", move |b| b.iter(|| colorguess::count_outcomes(&colorguess::Pegs::new(&[1,2,3,4]), &all)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);