use std::collections::HashSet;

fn main() {
    println!(
        "{}",
        longest_common_subsequence("abcde".to_string(), "ace".to_string())
    );
}

//takes too long
fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    fn get_all_subsequences(text: String) -> Vec<String> {
        if text.len() == 1 {
            return vec![text];
        }
        let chars = text.chars().collect::<Vec<char>>();
        let mut memo: HashSet<String> = HashSet::from([chars.last().unwrap().to_string()]);
        for i in (0..chars.len() - 1).rev() {
            let c = chars[i];
            memo.clone().into_iter().for_each(|o| {
                let mut x = o.clone();
                x.insert(0, c);
                memo.insert(x);
            });
            memo.insert(c.to_string());
        }
        memo.into_iter().collect::<Vec<_>>()
    }

    let (len1, len2) = (text1.len(), text2.len());
    if len1 == len2 {
        if text1 == text2 {
            return len1 as i32;
        }
    }
    let (main, to_check) = if len1 < len2 {
        (text2, text1)
    } else {
        (text1, text2)
    };
    let len2 = to_check.len();
    if main[..len2] == to_check {
        return len2 as i32;
    }

    let mut all_subsequences = get_all_subsequences(to_check.clone());
    all_subsequences.sort_by(|a, b| b.len().cmp(&a.len()));
    'main: for sub in all_subsequences {
        let chars: Vec<char> = sub.chars().collect();
        let mut last: isize = -1;
        let mut found = 0;
        for c in chars {
            let temp = main
                .chars()
                .skip((last + 1) as usize)
                .enumerate()
                .filter(|(_, x)| x == &c)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            let ind = temp.get(0);
            if ind == None {
                continue 'main;
            }
            last += *ind.unwrap() as isize + 1;
            found += 1;
        }
        return found;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        let io = [
            // (("abcde", "ace"), 3),
            // (("abc", "abc"), 3),
            // (("abc", "def"), 0),
            // (("bl", "yby"), 1),
            // (("hofubmnylkra", "pqhgxgdofcvmr"), 5),
            // (("mhunuzqrkzsnidwbun", "szulspmhwpazoxijwbq"), 6),
            // (("abcba", "abcbcba"), 5),
            (("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), 210),
        ];
        io.into_iter().for_each(|(input, output)| {
            assert_eq!(
                longest_common_subsequence(input.0.to_string(), input.1.to_string()),
                output
            );
        });
    }
    //
    // #[test]
    // fn test_get_all_subsequences() {
    //     let io = [
    //         ("a", vec!["a".to_string()]),
    //         (
    //             "ab",
    //             vec!["b".to_string(), "ab".to_string(), "a".to_string()],
    //         ),
    //         (
    //             "cab",
    //             vec![
    //                 "b".to_string(),
    //                 "ab".to_string(),
    //                 "a".to_string(),
    //                 "cb".to_string(),
    //                 "cab".to_string(),
    //                 "ca".to_string(),
    //                 "c".to_string(),
    //             ],
    //         ),
    //         (
    //             "dcab",
    //             vec![
    //                 "b".to_string(),
    //                 "ab".to_string(),
    //                 "a".to_string(),
    //                 "cb".to_string(),
    //                 "cab".to_string(),
    //                 "ca".to_string(),
    //                 "c".to_string(),
    //                 "db".to_string(),
    //                 "dab".to_string(),
    //                 "da".to_string(),
    //                 "dcb".to_string(),
    //                 "dcab".to_string(),
    //                 "dca".to_string(),
    //                 "dc".to_string(),
    //                 "d".to_string(),
    //             ],
    //         ),
    //     ];
    //     io.into_iter().for_each(|(input, output)| {
    //         assert_eq!(get_all_subsequences(input.to_string()), output);
    //     });
    // }
}
