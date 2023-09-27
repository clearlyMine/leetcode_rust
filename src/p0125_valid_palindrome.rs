//0ms 2.06MB (100.00% 099.61%)
pub fn is_palindrome(s: String) -> bool {
    let s = s.bytes().filter_map(|c| {
        if c.is_ascii_alphanumeric() {
            Some(c.to_ascii_lowercase())
        } else {
            None
        }
    });
    s.clone().eq(s.rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let io = [
            ("aba", true),
            ("a,a", true),
            ("a.    a", true),
            ("ab", false),
            ("aba ", true),
            ("a,bcda", false),
            ("A man, a plan, a canal: Panama", true),
        ];
        io.into_iter()
            .for_each(|(input, output)| assert_eq!(is_palindrome(input.to_string()), output));
    }
}
