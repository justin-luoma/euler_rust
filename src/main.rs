use euler::problem_60;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let answer = problem_60::solve(5);

    let elapased = start.elapsed();

    println!(
        "problem 60 for 5: {}\nTook {} milliseconds ({} seconds)",
        answer,
        elapased.as_millis(),
        elapased.as_secs()
    );
}
