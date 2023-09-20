fn main() {
    println!(
        "{}",
        longest_common_subsequence("abcde".to_string(), "ace".to_string())
    );
}

//takes too long
fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    use std::collections::HashMap;
    fn recurse(
        text1: String,
        text2: String,
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        let t = memo.get(&(i, j));
        if t.is_some() {
            return *t.unwrap();
        }
        if i == text1.len() || j == text2.len() {
            return 0;
        }
        if text1.chars().nth(i) == text2.chars().nth(j) {
            let t = 1 + recurse(text1, text2, i + 1, j + 1, memo);
            memo.insert((i, j), t);
            return t;
        } else {
            let t = recurse(text1.clone(), text2.clone(), i + 1, j, memo).max(recurse(
                text1,
                text2,
                i,
                j + 1,
                memo,
            ));
            memo.insert((i, j), t);
            return t;
        }
    }
    recurse(text1, text2, 0, 0, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        let io = [
            (("abcde", "ace"), 3),
            (("abc", "abc"), 3),
            (("abc", "def"), 0),
            (("bl", "yby"), 1),
            (("hofubmnylkra", "pqhgxgdofcvmr"), 5),
            (("mhunuzqrkzsnidwbun", "szulspmhwpazoxijwbq"), 6),
            (("abcba", "abcbcba"), 5),
            (("yzebsbuxmtcfmtodclszgh", "ejevmhcvshclydqrulwbyha"), 6),
            (("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), 210),
        ];
        io.into_iter().for_each(|(input, output)| {
            assert_eq!(
                longest_common_subsequence(input.0.to_string(), input.1.to_string()),
                output
            );
        });
    }
}
