#[allow(dead_code)]
fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    use std::collections::HashMap;
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in &mut s.chars() {
        if map.contains_key(&c) {
            let i = map.get(&c).unwrap();
            let x = i + 1;
            map.insert(c, x);
            continue;
        }
        map.insert(c, 1);
    }
    for c in &mut t.chars() {
        if !map.contains_key(&c) {
            return false;
        }
        let i = map.get(&c).unwrap();
        let x = i - 1;
        if x == 0 {
            map.remove(&c);
            continue;
        }
        map.insert(c, x);
    }

    return true;
}

#[allow(dead_code)]
fn is_anagram_2(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut a: Vec<char> = vec![];

    for cr in &mut s.chars() {
        a.push(cr);
    }
    for cr in &mut t.chars() {
        let mut found_at: usize = 1;
        let mut not_found: bool = true;
        for (i, charu) in a.iter().enumerate() {
            if charu == &cr {
                found_at = i;
                not_found = false;
                break;
            }
        }
        if not_found {
            return false;
        }
        a.remove(found_at);
    }
    return true;
}

#[allow(dead_code)]
fn is_anagram_copied(s: String, t: String) -> bool {
    let mut map = std::collections::HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
    map.into_values().all(|v| v == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_anagram() {
        assert!(is_anagram(String::from("anagram"), String::from("nagaram")));
        assert!(!is_anagram(String::from("rat"), String::from("car")));
        assert!(is_anagram_2(
            String::from("anagram"),
            String::from("nagaram")
        ));
        assert!(!is_anagram_2(String::from("rat"), String::from("car")));
        assert!(is_anagram_copied(
            String::from("anagram"),
            String::from("nagaram")
        ));
        assert!(!is_anagram_copied(String::from("rat"), String::from("car")));
    }
}
