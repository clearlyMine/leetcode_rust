#[allow(dead_code)]
pub fn count_substrings(s: String) -> i32 {
    fn is_palindrome(p: &[char]) -> bool {
        let len = p.len();
        if len == 1 {
            return true;
        }
        let (mut i, mut j) = (0, len - 1);
        while i < j {
            if p[i] != p[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }

    let len = s.len();
    if len == 1 {
        return 1;
    }
    let chars: Vec<char> = s.chars().collect();
    let mut out = 0;
    for i in 0..chars.len() {
        let rest = chars[i..].to_vec();
        for j in (0..rest.len()).rev() {
            let cur = &rest[..=j];
            if is_palindrome(&cur) {
                out += 1;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let io = [
            // ("babad", "aba"),
            // ("cbbd", "bb"),
            ("aaa", 6),
            ("abc", 3),
        ];
        io.into_iter().for_each(|(input, output)| {
            assert_eq!(count_substrings(input.to_string()), output);
        });
    }
}
