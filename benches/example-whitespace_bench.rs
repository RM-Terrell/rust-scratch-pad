use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

// ! example code and its benchmark
// The "Bad" Way (String reallocation inside loop)
fn remove_whitespace_thrasher(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if !c.is_whitespace() {
            result.push(c); // Reallocates string buffer repeatedly
        }
    }
    result
}

// The "Good" Idiomatic Way (Pre-allocated capacity)
fn remove_whitespace_builder(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        if !c.is_whitespace() {
            result.push(c);
        }
    }
    result
}

fn benchmarks(c: &mut Criterion) {
    let test_string = " \tThis is a string \n with \r many \t spaces and \n newlines.  ";

    c.bench_function("RemoveWhitespaceThrasher", |b| {
        b.iter(|| remove_whitespace_thrasher(black_box(test_string)))
    });

    c.bench_function("RemoveWhitespaceBuilder", |b| {
        b.iter(|| remove_whitespace_builder(black_box(test_string)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
