use std::collections::HashSet;

//https://leetcode.com/problems/valid-sudoku/solutions/1261934/rust-solution-using-three-1-d-arrays-with-bit-manipulation-instead-of-hashset/
pub fn is_valid_sudoku_new(board: Vec<Vec<char>>) -> bool {
    let mut rows: [u16; 9] = [0; 9];
    let mut cols: [u16; 9] = [0; 9];
    let mut boxes: [u16; 9] = [0; 9];

    for i in 0..9 {
        for j in 0..9 {
            match board[i][j] {
                '.' => continue,
                c => {
                    let b: usize = (i / 3) * 3 + (j / 3);
                    let curr = 1 << (c.to_digit(10).unwrap());

                    if rows[i] & curr != 0 || cols[j] & curr != 0 || boxes[b] & curr != 0 {
                        return false;
                    }

                    rows[i] |= curr;
                    cols[j] |= curr;
                    boxes[b] |= curr;
                }
            }
        }
    }

    true
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        let mut cur_row: HashSet<char> = HashSet::new();
        let mut cur_col: HashSet<char> = HashSet::new();
        let mut cur_section: HashSet<char> = HashSet::new();
        for j in 0..9 {
            let mut elem = board[i][j];
            if elem != '.' && !cur_row.insert(elem) {
                return false;
            }
            elem = board[j][i];

            if elem != '.' && !cur_col.insert(elem) {
                return false;
            }

            let row_index = 3 * (i / 3);
            let col_index = 3 * (i % 3);
            elem = board[row_index + (j / 3)][col_index + (j % 3)];
            if elem != '.' && !cur_section.insert(elem) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(is_valid_sudoku(board));

        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!is_valid_sudoku(board));

        let board = vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ];
        assert!(!is_valid_sudoku(board));
    }

    #[test]
    fn test_is_valid_sudoku_new() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(is_valid_sudoku_new(board));

        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!is_valid_sudoku_new(board));

        let board = vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ];
        assert!(!is_valid_sudoku_new(board));
    }
}
