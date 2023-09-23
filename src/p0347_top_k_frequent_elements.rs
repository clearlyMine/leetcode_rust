#[allow(dead_code)]
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![];
    }
    if nums.len() == 1 && k == 1 {
        return nums;
    }
    use std::collections::HashMap;
    let mut frequency: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        match frequency.get(&num) {
            Some(f) => frequency.insert(num.clone(), f + 1),
            None => frequency.insert(num.clone(), 1),
        };
    }
    let mut most_frequent: Vec<i32> = vec![0; k as usize];
    for i in 0..k {
        let x = frequency
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k);
        match x {
            Some(v) => {
                most_frequent[i as usize] = *v;
                let p = v.clone();
                frequency.remove(&p);
            }
            None => return most_frequent,
        }
    }
    return most_frequent;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        let input = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let expected: Vec<i32> = vec![1, 2];
        let result = top_k_frequent(input, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_top_k_frequent_single_input() {
        let input = vec![1];
        let k = 1;
        let expected = vec![1];
        let result = top_k_frequent(input, k);
        assert_eq!(result, expected);
    }
}
