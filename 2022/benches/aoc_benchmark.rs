use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(aoc2022::day01::run));
    c.bench_function("day02", |b| b.iter(aoc2022::day02::run));
    c.bench_function("day03", |b| b.iter(aoc2022::day03::run));
    c.bench_function("day04", |b| b.iter(aoc2022::day04::run));
    c.bench_function("day05", |b| b.iter(aoc2022::day05::run));
    c.bench_function("day06", |b| b.iter(aoc2022::day06::run));
    c.bench_function("day07", |b| b.iter(aoc2022::day07::run));
    c.bench_function("day08", |b| b.iter(aoc2022::day08::run));
    c.bench_function("day09", |b| b.iter(aoc2022::day09::run));
    // c.bench_function("day10", |b| b.iter(aoc2022::day10::run));
    // c.bench_function("day11", |b| b.iter(aoc2022::day11::run));
    // c.bench_function("day12", |b| b.iter(aoc2022::day12::run));
    // c.bench_function("day13", |b| b.iter(aoc2022::day13::run));
    // c.bench_function("day14", |b| b.iter(aoc2022::day14::run));
    // c.bench_function("day15", |b| b.iter(aoc2022::day15::run));
    // c.bench_function("day16", |b| b.iter(aoc2022::day16::run));
    // c.bench_function("day17", |b| b.iter(aoc2022::day17::run));
    // c.bench_function("day18", |b| b.iter(aoc2022::day18::run));
    // c.bench_function("day19", |b| b.iter(aoc2022::day19::run));
    // c.bench_function("day20", |b| b.iter(aoc2022::day20::run));
    // c.bench_function("day21", |b| b.iter(aoc2022::day21::run));
    // c.bench_function("day22", |b| b.iter(aoc2022::day22::run));
    // c.bench_function("day23", |b| b.iter(aoc2022::day23::run));
    // c.bench_function("day24", |b| b.iter(aoc2022::day24::run));
    // c.bench_function("day25", |b| b.iter(aoc2022::day25::run));
}

criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(std::time::Duration::from_secs(10));
  targets = criterion_benchmark
}

criterion_main!(benches);
