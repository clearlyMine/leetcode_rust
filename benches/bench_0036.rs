use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode_problems::p0036_valid_sudoku::{is_valid_sudoku, is_valid_sudoku_new};

// Define your benchmark function.
fn benchmark_is_valid_sudoku(c: &mut Criterion) {
    let io = [
        vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ],
        vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ],
        vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ],
    ];
    let mut group = c.benchmark_group("is valid sudoku");
    for (i, input) in io.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("is_valid_sudoku", i),
            &input,
            |b, input| b.iter(|| is_valid_sudoku(input.to_vec())),
        );
        group.bench_with_input(
            BenchmarkId::new("is_valid_sudoku_new", i),
            &input,
            |b, input| b.iter(|| is_valid_sudoku_new(input.to_vec())),
        );
    }
    group.finish();
}

criterion_group!(benches, benchmark_is_valid_sudoku);
criterion_main!(benches);
