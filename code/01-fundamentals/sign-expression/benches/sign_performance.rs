use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

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

fn benchmark_sign(c: &mut Criterion) {
    let mut group = c.benchmark_group("sign return type");

    group.bench_function("borrowed &'static str", |bencher| {
        let mut n = -1;

        bencher.iter(|| {
            n = if n == 1 { -1 } else { n + 1 };
            black_box(sign_borrowed(black_box(n)))
        });
    });

    group.bench_function("owned String", |bencher| {
        let mut n = -1;

        bencher.iter(|| {
            n = if n == 1 { -1 } else { n + 1 };
            black_box(sign_owned(black_box(n)))
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_sign);
criterion_main!(benches);
