use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use euler::{
    problem_1, problem_2, problem_3, problem_4, problem_5, problem_6, problem_60, problem_61,
    problem_62, problem_67, problem_7,
};

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

pub fn benchmark_problem_60(c: &mut Criterion) {
    c.bench_function("problem 60", |b| b.iter(|| problem_60::solve(black_box(5))));
}

pub fn benchmark_problem_61(c: &mut Criterion) {
    c.bench_function("problem 61", |b| b.iter(problem_61::solve));
}

pub fn benchmark_problem_62(c: &mut Criterion) {
    let set_of = [3, 5];

    let mut group = c.benchmark_group("problem 62");

    set_of.iter().for_each(|set| {
        group.bench_with_input(BenchmarkId::from_parameter(set), set, |b, &set| {
            b.iter(|| problem_62::solve(set))
        });
    });

    group.finish();

    // c.bench_with_input(BenchmarkId::new("problem 62", set_of), &set_of, |b, &s| {
    //     b.iter(|| problem_62::solve(s))
    // });
}

pub fn benchmark_problem_67(c: &mut Criterion) {
    c.bench_function("problem 67", |b| b.iter(problem_67::solve_67));
}

criterion_group!(
    benches,
    benchmark_problem_1,
    benchmark_problem_2,
    benchmark_problem_3,
    benchmark_problem_4,
    benchmark_problem_5,
    benchmark_problem_6,
    benchmark_problem_7,
    benchmark_problem_60,
    benchmark_problem_61,
    benchmark_problem_62,
    benchmark_problem_67,
);
criterion_main!(benches);
