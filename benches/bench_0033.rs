use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode_problems::p0033_search_in_rotated_sorted_array::{search, search_copied};

// Define your benchmark function.
fn benchmark_is_valid_sudoku(c: &mut Criterion) {
    let io = [
        ([4, 5, 6, 7, 0, 1, 2].to_vec(), 0),
        ([4, 5, 6, 7, 0, 1, 2].to_vec(), 3),
        ([1].to_vec(), 0),
    ];
    let mut group = c.benchmark_group("search in rotated sorted array");
    for (i, input) in io.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("search", i), &input, |b, input| {
            b.iter(|| search(input.0.clone(), input.1))
        });
        group.bench_with_input(BenchmarkId::new("search_copied", i), &input, |b, input| {
            b.iter(|| search_copied(input.0.clone(), input.1))
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_is_valid_sudoku);
criterion_main!(benches);
