#[allow(dead_code)]
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }

    if nums.len() == 1 {
        return nums;
    }

    let mut result: Vec<i32> = vec![1; nums.len()];
    let mut product_prefix = vec![1; nums.len()];
    let mut product_suffix = vec![1; nums.len()];

    for i in 1..nums.len() {
        product_prefix[i] = product_prefix[i - 1] * nums[i - 1];
    }

    for i in (0..(nums.len() - 1)).rev() {
        product_suffix[i] = product_suffix[i + 1] * nums[i + 1];
    }

    for i in 0..nums.len() {
        result[i] = product_prefix[i] * product_suffix[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        let input = vec![1, 2, 3, 4];
        let expected: Vec<i32> = vec![24, 12, 8, 6];
        let result = product_except_self(input);
        assert_eq!(result, expected);
    }
}
