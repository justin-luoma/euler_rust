use chrono::Local;
use euler::problem_69;
use std::time::Instant;

fn main() {
    println!("{}", Local::now().to_rfc2822());
    let start = Instant::now();

    let answer = problem_69::solve(1_000_000);

    let elapased = start.elapsed();

    println!(
        "problem 69: {}\nTook {} milliseconds ({} seconds)",
        answer,
        elapased.as_millis(),
        elapased.as_secs()
    );
}
