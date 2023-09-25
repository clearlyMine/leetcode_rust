use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode_problems::p0062_unique_paths::{unique_paths, unique_paths_space_optimized};

// Define your benchmark function.
fn benchmark_is_valid_sudoku(c: &mut Criterion) {
    let io = [(3, 7), (3, 2), (23, 12)];
    let mut group = c.benchmark_group("unique paths");
    for (i, input) in io.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("unique_paths_dp", i),
            &input,
            |b, input| b.iter(|| unique_paths(input.0, input.1)),
        );

        group.bench_with_input(
            BenchmarkId::new("unique_paths_dp_space_optimized", i),
            &input,
            |b, input| b.iter(|| unique_paths_space_optimized(input.0, input.1)),
        );

        // group.bench_with_input(
        //     BenchmarkId::new("unique_paths_dfs", i),
        //     &input,
        //     |b, input| b.iter(|| unique_paths_dfs(input.0, input.1)),
        // );
        //
        // group.bench_with_input(
        //     BenchmarkId::new("unique_paths_dp", i),
        //     &input,
        //     |b, input| b.iter(|| unique_paths_bfs(input.0, input.1)),
        // );
    }
    group.finish();
}

criterion_group!(benches, benchmark_is_valid_sudoku);
criterion_main!(benches);
