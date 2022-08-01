use criterion::{black_box, criterion_group, criterion_main, Criterion};
use euler::{problem_1, problem_2, problem_3, problem_4, problem_5, problem_6, problem_7};

pub fn benchmark_problem_1(c: &mut Criterion) {
    c.bench_function("problem 1", |b| {
        b.iter(|| problem_1::solve(black_box(1000)))
    });
}

pub fn benchmark_problem_2(c: &mut Criterion) {
    c.bench_function("problem 2", |b| b.iter(problem_2::solve));
}

pub fn benchmark_problem_3(c: &mut Criterion) {
    c.bench_function("problem 3", |b| {
        b.iter(|| problem_3::solve(black_box(600851475143)))
    });
}

pub fn benchmark_problem_4(c: &mut Criterion) {
    c.bench_function("problem 4", |b| b.iter(problem_4::solve));
}

pub fn benchmark_problem_5(c: &mut Criterion) {
    c.bench_function("problem 5", |b| {
        b.iter(|| problem_5::solve_v2(black_box(20)))
    });
}

pub fn benchmark_problem_6(c: &mut Criterion) {
    c.bench_function("problem 6", |b| b.iter(|| problem_6::solve(black_box(100))));
}

pub fn benchmark_problem_7(c: &mut Criterion) {
    c.bench_function("problem 7", |b| {
        b.iter(|| problem_7::solve(black_box(10001)))
    });
}

criterion_group!(
    benches,
    benchmark_problem_1,
    benchmark_problem_2,
    benchmark_problem_3,
    benchmark_problem_4,
    benchmark_problem_5,
    benchmark_problem_6,
    benchmark_problem_7
);
criterion_main!(benches);
