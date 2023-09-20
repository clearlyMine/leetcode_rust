fn main() {
    println!("{}", longest_palindrome("babad".to_string()));
}

fn longest_palindrome(s: String) -> String {
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
        return s;
    }
    let chars: Vec<char> = s.chars().collect();
    let mut longest = "".to_string();
    for i in 0..chars.len() {
        // 'main: for (i, c) in chars.clone().iter().enumerate() {
        let rest = chars[i..].to_vec();
        if rest.len() + 1 < longest.len() {
            break;
        }
        for j in (0..rest.len()).rev() {
            let cur = &rest[..=j];
            if cur.len() <= longest.len() {
                break;
            }
            if is_palindrome(&cur) {
                longest = "".to_string();
                for c in cur {
                    longest.push(*c);
                }
                break;
            }
        }
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let io = [
            // ("babad", "aba"),
            // ("cbbd", "bb"),
            // ("ac", "a"),
            ("bacabab", "bacab"),
        ];
        io.into_iter().for_each(|(input, output)| {
            assert_eq!(longest_palindrome(input.to_string()), output.to_string());
        });
    }
}
