use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }
    let mut longest: HashMap<char, i32> = HashMap::new();
    let mut longest_string_yet: i32 = 1;
    let mut long_string = String::new();
    let mut i: i32 = 0;
    for c in &mut s.chars() {
        if longest.contains_key(&c) {
            let length = long_string.len() as i32;
            if longest_string_yet < length {
                longest_string_yet = length;
            };
            let v = long_string.split(c).collect::<Vec<&str>>();
            long_string = v[1].to_string();
            long_string.push(c);
            longest.clear();
            i = 0;
            for p in &mut long_string.chars() {
                longest.insert(p, i);
                i += 1;
            }
        } else {
            longest.insert(c.clone(), i);
            long_string.push(c);
            i += 1;
        }
    }
    if longest_string_yet > longest.len() as i32 {
        return longest_string_yet as i32;
    } else {
        return longest.len() as i32;
    };
}

// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/2220293/rust-simple-idiomatic-solution/?orderBy=most_votes&languageTags=rust
#[allow(dead_code)]
pub fn length_of_longest_substring_copied(s: String) -> i32 {
    let mut hm = HashMap::new();
    let mut start = 0;
    let mut res = 0;

    for (idx, ch) in s.chars().enumerate() {
        start = start.max(hm.insert(ch, idx as i32).unwrap_or(-1) + 1);
        res = res.max(idx as i32 - start + 1);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(3, length_of_longest_substring("dvdf".to_string()));
        assert_eq!(2, length_of_longest_substring("aab".to_string()));
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
        assert_eq!(1, length_of_longest_substring("bbbbb".to_string()));
        assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
        assert_eq!(3, length_of_longest_substring_copied("dvdf".to_string()));
        assert_eq!(2, length_of_longest_substring_copied("aab".to_string()));
        assert_eq!(
            3,
            length_of_longest_substring_copied("abcabcbb".to_string())
        );
        assert_eq!(1, length_of_longest_substring_copied("bbbbb".to_string()));
        assert_eq!(3, length_of_longest_substring_copied("pwwkew".to_string()));
    }
}
