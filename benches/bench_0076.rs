use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode_problems::p0076_minimum_window_substring::{min_window_cur, min_window_fastest_yet};

// Define your benchmark function.
fn benchmark_is_valid_sudoku(c: &mut Criterion) {
    let io = [
        ("ADOBECODEBANC", "ABC"),
        ("a", "a"),
        ("a", "aa"),
        ("aa", "a"),
        ("ab", "ba"),
        ("bbaa", "aba"),
        ("acbbaca", "aba"),
        ("aallooo", "aloe"),
        ("aaalooooooooooo", "alllo"),
    ];
    let mut group = c.benchmark_group("min window substring");
    for (i, input) in io.iter().enumerate() {
        group.sample_size(100);
        if i == 7 {
            group.sample_size(10);
        }
        group.bench_with_input(BenchmarkId::new("min window", i), &input, |b, input| {
            b.iter(|| min_window_cur(input.0.to_string(), input.1.to_string()))
        });
        // group.bench_with_input(
        //     BenchmarkId::new("min window fastest yet", i),
        //     &input,
        //     |b, input| b.iter(|| min_window_fastest_yet(input.0.to_string(), input.1.to_string())),
        // );

        // group.bench_with_input(BenchmarkId::new("min window old", i), &input, |b, input| {
        //     b.iter(|| min_window_old(input.0.to_string(), input.1.to_string()))
        // });
    }

    group.sample_size(10);
    //very long input
    let file_io: Vec<&str> = include_str!("../inputs/p0076.txt")
        .split_whitespace()
        .collect();

    let mut i = 0;
    while i < file_io.len() {
        group.bench_with_input(
            BenchmarkId::new("min window", io.len() + i / 3),
            &file_io.clone(),
            |b, input| b.iter(|| min_window_cur(input[i].to_string(), input[i + 1].to_string())),
        );
        // group.bench_with_input(
        //     BenchmarkId::new("min window fastest yet", io.len() + i / 3),
        //     &file_io,
        //     |b, input| {
        //         b.iter(|| min_window_fastest_yet(input[i].to_string(), input[i + 1].to_string()))
        //     },
        // );
        i += 3;
    }

    group.finish();
}

criterion_group!(benches, benchmark_is_valid_sudoku);
criterion_main!(benches);
