use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode_problems::p0001_two_sum::{two_sum, two_sum_copied, two_sum_with_hash_map};

// Define your benchmark function.
fn benchmark_two_sums(c: &mut Criterion) {
    let io = [(vec![2, 7, 11, 15], 9), (vec![3, 2, 4], 6), (vec![3, 3], 6)];
    let mut group = c.benchmark_group("Two Sums");
    for (i, input) in io.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("two_sum", i), &input, |b, input| {
            b.iter(|| two_sum(&input.0, input.1))
        });
        group.bench_with_input(BenchmarkId::new("two_sum_copied", i), &input, |b, input| {
            b.iter(|| two_sum_copied(&input.0, input.1))
        });
        group.bench_with_input(
            BenchmarkId::new("two_sum_with_hash_map", i),
            &input,
            |b, input| b.iter(|| two_sum_with_hash_map(&input.0, input.1)),
        );
    }
    group.finish();
}

criterion_group!(benches, benchmark_two_sums);
criterion_main!(benches);
