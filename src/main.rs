use chrono::Local;
use euler::problem_61;
use std::time::Instant;

fn main() {
    println!("{}", Local::now().to_rfc2822());
    let start = Instant::now();

    let answer = problem_61::solve();

    let elapased = start.elapsed();

    println!(
        "problem 61: {}\nTook {} milliseconds ({} seconds)",
        answer,
        elapased.as_millis(),
        elapased.as_secs()
    );
}
