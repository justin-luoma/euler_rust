use chrono::Local;
use euler::problem_62;
use std::time::Instant;

fn main() {
    println!("{}", Local::now().to_rfc2822());
    let start = Instant::now();

    let answer = problem_62::solve(5);

    let elapased = start.elapsed();

    println!(
        "problem 62 for 5: {}\nTook {} milliseconds ({} seconds)",
        answer,
        elapased.as_millis(),
        elapased.as_secs()
    );
}
