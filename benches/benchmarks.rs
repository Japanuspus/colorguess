#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate colorguess;

fn criterion_benchmark(c: &mut Criterion) {
    let all = colorguess::build_all_configs();
    c.bench_function("count all", move |b| b.iter(|| colorguess::count_outcomes(&[1,2,3,4], &all)));

    let all = colorguess::build_all_configs();
    //c.bench_function("count color", move |b| b.iter(|| all.iter().map(|pp| colorguess::count_matches_color(pp, &[1,2,3,4])).sum::<u8>()));
    c.bench_function("count color", move |b| b.iter(|| all.iter().map(|pp| colorguess::count_matches_color_nosort(pp, &[1,2,3,4])).sum::<u8>()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);