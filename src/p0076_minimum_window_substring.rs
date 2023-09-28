use std::collections::HashMap;

//2ms 2.26MB ( 91.67%  75.00%)
pub fn min_window_cur(s: String, t: String) -> String {
    // Early exit for empty string
    if t == "" {
        return "".to_string();
    }
    if s == t {
        return t;
    }

    // Array for storing counts from upper-case A to lower-case z in ASCII code order
    let mut sw_counts: [usize; 58] = [0; 58];
    let mut t_counts: [usize; 58] = [0; 58];

    // Convert to Vec<char> for repeated iteration
    let (s, t) = (s.as_bytes(), t.as_bytes());

    // Get t counts
    t.into_iter()
        .for_each(|c| t_counts[(c - b'A') as usize] += 1);

    // How many unique characters need to be present in the substring
    let need = t_counts.into_iter().filter(|&x| x != 0).count();

    // l and r indices corresponding to minimum window. Initialize to invalid r for special casing substring not present case
    let (mut final_left, mut final_right) = (0, s.len() + 1);

    (0..s.len()).fold((0, 0), |(mut have, mut left), right| {
        // If we satisfy required char count by visiting the new right character, increment have count
        let index_right = (s[right] - b'A') as usize;
        sw_counts[index_right] += 1;
        if t_counts[index_right] > 0 && sw_counts[index_right] == t_counts[index_right] {
            have += 1;
        }
        // Since we have found a window with substring try to trim from left until we no longer can
        while have == need {
            let index_left = (s[left] - b'A') as usize;

            // Found a valid window with new min length
            if right - left < final_right - final_left {
                final_left = left;
                final_right = right;
            }

            // Start pruning from left
            sw_counts[index_left] -= 1;
            if t_counts[index_left] > 0 && sw_counts[index_left] < t_counts[index_left] {
                have -= 1;
            }
            left += 1;
        }
        (have, left)
    });
    // What if you never found the substring
    if final_right <= s.len() {
        String::from_utf8(s[final_left..=final_right].to_vec()).unwrap()
    } else {
        "".to_string()
    }
}

//TLE
pub fn min_window_fastest_yet(s: String, t: String) -> String {
    let (l1, l2) = (s.len(), t.len());
    if l1 < l2 {
        return "".to_string();
    }

    if s == t {
        return t;
    }

    let s_bytes = s.as_bytes();

    let mut frequency_map: HashMap<u8, isize> = t.bytes().fold(HashMap::new(), |mut map, c| {
        map.entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        map
    });

    let (mut left, mut right) = (0, 0);
    let (mut final_left, mut final_right) = (0, s_bytes.len() + 1);
    let mut missing = frequency_map.len() as i32;
    loop {
        if right < s_bytes.len() && missing > 0 {
            if let Some(freq) = frequency_map.get_mut(&s_bytes[right]) {
                *freq -= 1;
                if *freq == 0 {
                    missing -= 1;
                }
            }
            right += 1;
        } else if left < s_bytes.len() && missing <= 0 {
            if right - left < final_right - final_left {
                final_right = right;
                final_left = left;
            }
            if let Some(freq) = frequency_map.get_mut(&s_bytes[left]) {
                if *freq == 0 {
                    missing += 1
                };
                *freq += 1;
            }
            left += 1;
        } else {
            break;
        }
    }

    if final_right <= s_bytes.len() {
        s[final_left..final_right].to_string()
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window_cur() {
        let io = [
            (("ADOBECODEBANC", "ABC"), "BANC"),
            (("a", "a"), "a"),
            (("a", "aa"), ""),
            (("aa", "a"), "a"),
            (("ab", "ba"), "ab"),
            (("bbaa", "aba"), "baa"),
            (("acbbaca", "aba"), "baca"),
        ];
        io.into_iter().for_each(|((s, t), o)| {
            assert_eq!(min_window_cur(s.to_string(), t.to_string()), o);
        });
        let file_io: Vec<&str> = include_str!("../inputs/p0076.txt")
            .split_whitespace()
            .collect();
        let mut i = 0;
        while i < file_io.len() {
            assert_eq!(
                min_window_cur(file_io[i].to_string(), file_io[i + 1].to_string()),
                file_io[i + 2].to_string()
            );
            i += 3;
        }
    }
}
