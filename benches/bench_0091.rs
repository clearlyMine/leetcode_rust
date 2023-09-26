use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode_problems::p0091_decode_ways::{num_decodings, num_decodings_bytes};

// Define your benchmark function.
fn benchmark_is_valid_sudoku(c: &mut Criterion) {
    let io = ["12", "226", "06", "27", "2101", "2611055971756562"];
    let mut group = c.benchmark_group("num decodings");
    for (i, input) in io.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("num_decodings", i), &input, |b, input| {
            b.iter(|| num_decodings(input.to_string()))
        });

        group.bench_with_input(
            BenchmarkId::new("num_decodings_bytes", i),
            &input,
            |b, input| b.iter(|| num_decodings_bytes(input.to_string())),
        );
    }
    group.finish();
}

criterion_group!(benches, benchmark_is_valid_sudoku);
criterion_main!(benches);
