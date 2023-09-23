#[allow(dead_code)]
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    if nums.len() == 1 {
        return 1;
    }

    use std::collections::HashSet;
    let map: HashSet<i32> = nums.into_iter().collect();
    let mut count: i32 = 1;
    for num in map.iter() {
        if map.contains(&(num - 1)) {
            continue;
        }
        let mut x = num + 1;
        let mut local_count = 1;
        while map.contains(&x) {
            local_count += 1;
            x += 1;
        }
        if local_count > count {
            count = local_count;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive_1() {
        let input = vec![100, 4, 200, 1, 3, 2];
        let expected: i32 = 4;
        let result = longest_consecutive(input);
        assert_eq!(result, expected);
        let input = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let expected: i32 = 9;
        let result = longest_consecutive(input);
        assert_eq!(result, expected);
    }
}
