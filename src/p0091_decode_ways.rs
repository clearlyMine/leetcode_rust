pub fn num_decodings_dp(s: String) -> i32 {
    let mut chars = s.chars();

    let mut tens = chars.next().unwrap();
    if tens == '0' {
        //starts with zero
        return 0;
    }
    let len = s.len();
    if len == 1 {
        return 1;
    }

    let mut p1 = 1;
    let mut p2 = 1;
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

pub fn num_decodings_dp_with_bytes(s: String) -> i32 {
    let len = s.len();

    let chars = s.into_bytes();

    let first = chars[0];
    if first == b'0' {
        //starts with zero
        return 0;
    }
    if len == 1 {
        return 1;
    }

    let mut p1 = 1;
    let mut p2 = 1;
    let mut x: i32;
    let mut tens: u8;
    let mut ones: u8;
    for i in 1..len {
        tens = chars[i - 1];
        ones = chars[i];
        x = 0;
        if ones != b'0' {
            x = p2;
        }
        if tens == b'1' || (tens == b'2' && ones < b'7') {
            x += p1;
        }
        p1 = p2;
        p2 = x;
    }
    p2
}

//https://leetcode.com/problems/decode-ways/solutions/2645913/rust-0-ms-constrained-fibonacci-with-detailed-comments/
pub fn num_decodings_dp_with_windows(s: String) -> i32 {
    let bytes = s.as_bytes();
    // [1] trivial cases
    if bytes[0] == b'0' {
        return 0;
    }
    if s.len() == 1 {
        return 1;
    }

    // [2] number of decodings when adding one letter
    let mut d1 = 1;

    // [3] number of decodings when adding two letters
    let mut d2 = 1;

    // [4] iterate over the string and update decodings for the last two steps
    let mut n;
    for w in bytes.windows(2) {
        n = 0;

        // [5] if current digit is not '0', we can add decodings from the step i-1
        if w[1] != b'0' {
            n += d1;
        }

        // [6] if two digits are in range from '10' to '26', we can add decodings from the step i-2
        if (w[0] == b'1') || (w[0] == b'2' && w[1] - b'0' <= 6) {
            n += d2;
        }

        // [7] update the number of decodings to be used on the next step
        d2 = d1;
        d1 = n;
    }

    return d1;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_decodings_dp() {
        assert_eq!(num_decodings_dp("12".to_string()), 2);
        assert_eq!(num_decodings_dp("226".to_string()), 3);
        assert_eq!(num_decodings_dp("06".to_string()), 0);
        assert_eq!(num_decodings_dp("27".to_string()), 1);
        assert_eq!(num_decodings_dp("2101".to_string()), 1);
        assert_eq!(num_decodings_dp("2611055971756562".to_string()), 4);
    }

    #[test]
    fn test_num_decodings_dp_with_bytes() {
        assert_eq!(num_decodings_dp_with_bytes("12".to_string()), 2);
        assert_eq!(num_decodings_dp_with_bytes("226".to_string()), 3);
        assert_eq!(num_decodings_dp_with_bytes("06".to_string()), 0);
        assert_eq!(num_decodings_dp_with_bytes("27".to_string()), 1);
        assert_eq!(num_decodings_dp_with_bytes("2101".to_string()), 1);
        assert_eq!(
            num_decodings_dp_with_bytes("2611055971756562".to_string()),
            4
        );
    }

    #[test]
    fn test_num_decodings_with_windows() {
        assert_eq!(num_decodings_dp_with_windows("12".to_string()), 2);
        assert_eq!(num_decodings_dp_with_windows("226".to_string()), 3);
        assert_eq!(num_decodings_dp_with_windows("06".to_string()), 0);
        assert_eq!(num_decodings_dp_with_windows("27".to_string()), 1);
        assert_eq!(num_decodings_dp_with_windows("2101".to_string()), 1);
        assert_eq!(
            num_decodings_dp_with_windows("2611055971756562".to_string()),
            4
        );
    }
}
