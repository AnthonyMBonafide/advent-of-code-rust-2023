use criterion::{black_box, criterion_group, criterion_main, Criterion};

use adventofcode2023::advent2023::day1;
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| day1::get_nums_from_line(black_box("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".to_string()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);