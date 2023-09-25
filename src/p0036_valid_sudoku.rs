use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut cur_row: HashSet<char>;
    for row in &board {
        cur_row = HashSet::new();
        for elem in row {
            if elem == &'.' {
                continue;
            }
            if cur_row.contains(elem) {
                return false;
            }
            cur_row.insert(*elem);
        }
    }
    let mut cur_col: HashSet<char>;
    for j in 0..board[0].len() {
        cur_col = HashSet::new();
        for i in 0..board.len() {
            let elem = board[i][j];
            if elem == '.' {
                continue;
            }
            if cur_col.contains(&elem) {
                return false;
            }
            cur_col.insert(elem);
        }
    }

    let mut cur_section: HashSet<char> = HashSet::new();
    for set1 in vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]] {
        for set2 in vec![[0, 1, 2], [3, 4, 5], [6, 7, 8]] {
            for (i, j) in get_all_combinations(set1, set2) {
                let elem = board[i][j];
                if elem == '.' {
                    continue;
                }
                if cur_section.contains(&elem) {
                    return false;
                }
                cur_section.insert(elem);
            }
            cur_section = HashSet::new();
        }
    }

    true
}

fn get_all_combinations(set1: [usize; 3], set2: [usize; 3]) -> [(usize, usize); 9] {
    let mut out = [(0, 0); 9];
    let mut num = 0;
    for i in set1 {
        for j in set2 {
            out[num] = (i, j);
            num += 1;
        }
    }
    out
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
    fn test_get_all_combinations() {
        let set = [0, 1, 2];
        let out = [
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        assert_eq!(get_all_combinations(set, set), out);
    }
}
