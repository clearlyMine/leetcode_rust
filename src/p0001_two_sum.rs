use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if &nums[i] + &nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![0, 0]
}

#[allow(dead_code)]
pub fn two_sum_with_hash_map(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_map: HashMap<i32, Vec<usize>> = HashMap::new();
    for i in 0..nums.len() {
        let entry = nums_map.entry(nums[i]).or_insert(vec![i]);
        entry.push(i);
    }
    for i in 0..nums.len() {
        let complement = target - nums[i];
        let complement_index = nums_map.get(&complement);
        match complement_index {
            Some(_) => {
                for j in complement_index.unwrap().iter() {
                    if j != &i {
                        return vec![i as i32, *j as i32];
                    }
                }
            }
            None => continue,
        }
    }
    vec![0, 0]
}

#[allow(dead_code)]
pub fn two_sum_copied(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_to_idx: HashMap<i32, i32> = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        match num_to_idx.get(&(target - *num)) {
            Some(&idx2) => return vec![idx as i32, idx2],
            None => num_to_idx.insert(*num, idx as i32),
        };
    }

    // unreachable!(
    //     "No solution for nums = {:?} and target = {:?}",
    //     nums, target
    // );
    vec![0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let mut nums: Vec<i32> = vec![2, 7, 11, 15];
        let mut target: i32 = 9;
        let mut output: Vec<i32> = vec![1, 0];
        assert!(contains_all(&two_sum(&nums, target), &output));
        assert!(contains_all(&two_sum_with_hash_map(&nums, target), &output));
        assert!(contains_all(&two_sum_copied(&nums, target), &output));

        nums = vec![3, 2, 4];
        target = 6;
        output = vec![1, 2];
        assert!(contains_all(&two_sum(&nums, target), &output));
        assert!(contains_all(&two_sum_with_hash_map(&nums, target), &output));
        assert!(contains_all(&two_sum_copied(&nums, target), &output));

        nums = vec![3, 3];
        target = 6;
        output = vec![0, 1];
        assert!(contains_all(&two_sum(&nums, target), &output));
        assert!(contains_all(&two_sum_with_hash_map(&nums, target), &output));
        assert!(contains_all(&two_sum_copied(&nums, target), &output));
    }

    fn contains_all(first: &Vec<i32>, second: &Vec<i32>) -> bool {
        if first.len() != second.len() {
            return false;
        }
        let (len, mut first, mut second) = (first.len(), first.clone(), second.clone());

        first.sort();
        second.sort();

        for i in 0..len {
            if first[i] != second[i] {
                return false;
            }
        }
        true
    }
}
