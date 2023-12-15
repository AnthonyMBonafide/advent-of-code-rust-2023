use criterion::{black_box, criterion_group, criterion_main, Criterion};

use adventofcode2023::advent2023::day1;
pub fn day1_part1(c: &mut Criterion) {
    c.bench_function("day1_part1", |b| b.iter(|| day1::trebuchet_part_1(black_box("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".to_string()))));
}

pub fn day1_part2(c: &mut Criterion) {
    c.bench_function("day1_part2", |b| b.iter(|| day1::trebuchet_part_2(black_box("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".to_string()))));
}


criterion_group!(benches, day1_part1, day1_part2);
criterion_main!(benches);