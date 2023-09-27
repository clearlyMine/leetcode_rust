use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode_problems::p0322_coin_change::{coin_change_dp, coin_change_dp_opt};

// Define your benchmark function.
fn benchmark_is_valid_sudoku(c: &mut Criterion) {
    let io = [
        (vec![1, 2, 5], 11),
        (vec![2], 3),
        (vec![1], 0),
        (vec![186, 419, 83, 408], 6249),
    ];
    let mut group = c.benchmark_group("coin change");
    for (i, input) in io.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("coin change dp", i), &input, |b, input| {
            b.iter(|| coin_change_dp(input.0.clone(), input.1))
        });

        group.bench_with_input(
            BenchmarkId::new("coin change dp opt", i),
            &input,
            |b, input| b.iter(|| coin_change_dp_opt(input.0.clone(), input.1)),
        );
    }
    group.finish();
}

criterion_group!(benches, benchmark_is_valid_sudoku);
criterion_main!(benches);
