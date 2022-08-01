use euler::problem_50;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let answer = problem_50::solve_v2(1_000_000);

    let elapased = start.elapsed();

    println!(
        "problem 50 for 1,000,000: {}\nTook {} milliseconds ({} seconds)",
        answer,
        elapased.as_millis(),
        elapased.as_secs()
    );
}
