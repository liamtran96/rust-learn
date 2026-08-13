use std::hint::black_box;
use std::time::{Duration, Instant};

const ITERATIONS: u32 = 10_000_000;
const ROUNDS: usize = 7;

fn sign_borrowed(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n > 0 {
        "positive"
    } else {
        "zero"
    }
}

fn sign_owned(n: i32) -> String {
    if n < 0 {
        String::from("negative")
    } else if n > 0 {
        String::from("positive")
    } else {
        String::from("zero")
    }
}

fn measure_borrowed() -> Duration {
    let start = Instant::now();

    for i in 0..ITERATIONS {
        let n = black_box((i % 3) as i32 - 1);
        black_box(sign_borrowed(n));
    }

    start.elapsed()
}

fn measure_owned() -> Duration {
    let start = Instant::now();

    for i in 0..ITERATIONS {
        let n = black_box((i % 3) as i32 - 1);
        black_box(sign_owned(n));
    }

    start.elapsed()
}

fn median(mut durations: Vec<Duration>) -> Duration {
    durations.sort_unstable();
    let middle = durations.len() / 2;

    if durations.len().is_multiple_of(2) {
        (durations[middle - 1] + durations[middle]) / 2
    } else {
        durations[middle]
    }
}

fn nanos_per_call(duration: Duration) -> f64 {
    duration.as_nanos() as f64 / f64::from(ITERATIONS)
}

fn main() {
    // Warm up both paths before measuring them.
    black_box(measure_borrowed());
    black_box(measure_owned());

    let mut borrowed_rounds = Vec::with_capacity(ROUNDS);
    let mut owned_rounds = Vec::with_capacity(ROUNDS);

    for _ in 0..ROUNDS {
        // Alternate the order to reduce bias from temperature and background work.
        borrowed_rounds.push(measure_borrowed());
        owned_rounds.push(measure_owned());
        owned_rounds.push(measure_owned());
        borrowed_rounds.push(measure_borrowed());
    }

    let borrowed = median(borrowed_rounds);
    let owned = median(owned_rounds);
    let borrowed_ns = nanos_per_call(borrowed);
    let owned_ns = nanos_per_call(owned);

    println!("Iterations per measurement: {ITERATIONS}");
    println!("Borrowed &'static str: {borrowed:>10.3?} ({borrowed_ns:.2} ns/call)");
    println!("Owned String:          {owned:>10.3?} ({owned_ns:.2} ns/call)");
    println!("String / &str ratio:   {:.2}x", owned_ns / borrowed_ns);
    println!();
    println!("Run several times: operating-system activity can change timings.");
}
