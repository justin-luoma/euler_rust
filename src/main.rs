use chrono::Local;
use euler::problem_57;
use std::time::Instant;

fn main() {
    println!("{}", Local::now().to_rfc2822());
    let start = Instant::now();

    let answer = problem_57::solve(1000);

    let elapased = start.elapsed();

    println!(
        "problem 57 for 1000: {}\nTook {} milliseconds ({} seconds)",
        answer,
        elapased.as_millis(),
        elapased.as_secs()
    );
}
