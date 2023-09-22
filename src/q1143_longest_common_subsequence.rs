#[allow(dead_code)]
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let (len1, len2) = (text1.len(), text2.len());
    let (text1, text2, len2) = if len1 > len2 {
        (text1, text2, len2)
    } else {
        (text2, text1, len1)
    };
    let mut memo: Vec<i32> = vec![0; len2 + 1];
    for c in text1.chars() {
        let mut pr = 0;
        let mut prc;
        for (j, d) in text2.chars().enumerate() {
            prc = pr;
            pr = memo[j + 1];
            memo[j + 1] = if c == d { 1 + prc } else { pr.max(memo[j]) };
        }
    }
    *memo.last().unwrap_or(&0)
}

#[allow(dead_code)]
pub fn longest_common_subsequence_my_dp(text1: String, text2: String) -> i32 {
    let (len1, len2) = (text1.len(), text2.len());
    let (text1, text2, len2) = if len1 > len2 {
        (text1, text2, len2)
    } else {
        (text2, text1, len1)
    };
    let mut memo: Vec<i32> = vec![0; len2 + 1];
    for c in text1.bytes() {
        let mut prc;
        let mut pr = 0;
        for (j, d) in text2.bytes().enumerate() {
            prc = pr;
            pr = memo[j + 1];
            memo[j + 1] = if c == d { 1 + prc } else { memo[j].max(pr) };
        }
    }
    *memo.last().unwrap_or(&0)
}

#[allow(dead_code)]
pub fn longest_common_subsequence_basic(text1: String, text2: String) -> i32 {
    let (len1, len2) = (text1.len(), text2.len());
    let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
    let mut memo: Vec<Vec<i32>> = vec![vec![0; len2 + 1]; len1 + 1];
    for i in 1..=len1 {
        for j in 1..=len2 {
            memo[i][j] = if text1[i - 1] == text2[j - 1] {
                1 + memo[i - 1][j - 1]
            } else {
                memo[i - 1][j].max(memo[i][j - 1])
            };
        }
    }
    memo[len1][len2]
}

#[allow(dead_code)]
pub fn longest_common_subsequence_basic_2(text1: String, text2: String) -> i32 {
    let (len1, len2) = (text1.len(), text2.len());
    let mut memo: Vec<Vec<i32>> = vec![vec![0; len2 + 1]; len1 + 1];
    for i in 1..=len1 {
        for j in 1..=len2 {
            memo[i][j] = if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
                1 + memo[i - 1][j - 1]
            } else {
                memo[i - 1][j].max(memo[i][j - 1])
            };
        }
    }
    memo[len1][len2]
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
            (("ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc","bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"), 0),
            (("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), 210),
        ];
        io.into_iter().for_each(|(input, output)| {
            assert_eq!(
                longest_common_subsequence(input.0.to_string(), input.1.to_string()),
                output
            );
            assert_eq!(
                longest_common_subsequence_my_dp(input.0.to_string(), input.1.to_string()),
                output
            );
            assert_eq!(
                longest_common_subsequence_basic(input.0.to_string(), input.1.to_string()),
                output
            );
            assert_eq!(
                longest_common_subsequence_basic_2(input.0.to_string(), input.1.to_string()),
                output
            );
        });
    }
}
