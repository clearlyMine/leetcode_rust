use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode_problems::p1143_longest_common_subsequence::{
    longest_common_subsequence, longest_common_subsequence_basic,
    longest_common_subsequence_basic_2, longest_common_subsequence_my_dp,
};

// Define your benchmark function.
fn benchmark_longest_common_subsequence(c: &mut Criterion) {
    let io = [
            (("abcde", "ace"), 3),
            (("abc", "abc"), 3),
            (("abc", "def"), 0),
            (("bl", "yby"), 1),
            (("hofubmnylkra", "pqhgxgdofcvmr"), 5),
            (("mhunuzqrkzsnidwbun", "szulspmhwpazoxijwbq"), 6),
            (("abcba", "abcbcba"), 5),
            (("yzebsbuxmtcfmtodclszgh", "ejevmhcvshclydqrulwbyha"), 6),
            (("ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc","bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"), 0),
            (("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), 210),
        ];
    let mut group = c.benchmark_group("longest_common_subsequence dp");
    for (i, (input, _)) in io.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("DP", i), &input, |b, input| {
            b.iter(|| longest_common_subsequence(input.0.to_string(), input.1.to_string()))
        });
        group.bench_with_input(BenchmarkId::new("My DP", i), &input, |b, input| {
            b.iter(|| longest_common_subsequence_my_dp(input.0.to_string(), input.1.to_string()))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("longest_common_subsequence basic");
    for (i, (input, _)) in io.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("Basic", i), &input, |b, input| {
            b.iter(|| longest_common_subsequence_basic(input.0.to_string(), input.1.to_string()))
        });
        group.bench_with_input(BenchmarkId::new("Basic 2", i), &input, |b, input| {
            b.iter(|| longest_common_subsequence_basic_2(input.0.to_string(), input.1.to_string()))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("longest_common_subsequence my dp vs basic");
    for (i, (input, _)) in io.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("My DP", i), &input, |b, input| {
            b.iter(|| longest_common_subsequence_my_dp(input.0.to_string(), input.1.to_string()))
        });
        group.bench_with_input(BenchmarkId::new("Basic ", i), &input, |b, input| {
            b.iter(|| longest_common_subsequence_basic(input.0.to_string(), input.1.to_string()))
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_longest_common_subsequence);
criterion_main!(benches);
