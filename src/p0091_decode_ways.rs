pub fn num_decodings(s: String) -> i32 {
    let len = s.len();

    let mut chars = s.chars();

    let first = chars.next().unwrap();
    if first == '0' {
        //starts with zero
        return 0;
    }
    if len == 1 {
        return 1;
    }

    let mut p1 = 0;
    let mut p2 = 0;
    let mut tens = first;
    let ones = chars.next().unwrap();
    if ones != '0' {
        p1 = 1;
        p2 = 1;
    }
    if tens == '1' || (tens == '2' && ones.to_digit(10).unwrap() < 7) {
        p2 += 1;
    }
    if len == 2 {
        return p2;
    }
    tens = ones;
    while let Some(ones) = chars.next() {
        let mut x = 0;
        if ones != '0' {
            x = p2;
        }
        if tens == '1' || (tens == '2' && ones.to_digit(10).unwrap() < 7) {
            x += p1;
        }
        p1 = p2;
        p2 = x;
        tens = ones;
    }
    p2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_decodings() {
        assert_eq!(num_decodings("12".to_string()), 2);
        assert_eq!(num_decodings("226".to_string()), 3);
        assert_eq!(num_decodings("06".to_string()), 0);
        assert_eq!(num_decodings("27".to_string()), 1);
        assert_eq!(num_decodings("2101".to_string()), 1);
        assert_eq!(num_decodings("2611055971756562".to_string()), 4);
    }
}
