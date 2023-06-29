fn main() {}

fn max_product(nums: Vec<i32>) -> i32 {
    fn sub(nums: &[i32]) -> i32 {
        if nums.len() == 0 {
            return i32::MIN;
        }
        let product = nums.iter().product();
        if product > 0 {
            return product;
        }

        if let Some(pos) = nums.iter().position(|&x| x == 0) {
            return std::cmp::max(sub(&nums[..pos]), sub(&nums[pos + 1..])).max(0);
        }

        let mut res = product;
        let pos = nums.iter().position(|&x| x < 0).unwrap();
        res = res.max(std::cmp::max(sub(&nums[..pos]), sub(&nums[pos + 1..])));
        let pos = nums.iter().rposition(|&x| x < 0).unwrap();
        res = res.max(std::cmp::max(sub(&nums[..pos]), sub(&nums[pos + 1..])));
        res
    }
    return sub(&nums[..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product() {
        let input = vec![1, 2, 3, 4, 0];
        let expected: i32 = 1 * 2 * 3 * 4;
        let result = max_product(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_max_product_with_negative_values() {
        let mut input = vec![1, 2, 3, 4, -1];
        let mut expected: i32 = 1 * 2 * 3 * 4;
        let mut result = max_product(input);
        assert_eq!(result, expected);

        input = vec![-1, 2, 3, 4];
        expected = 2 * 3 * 4;
        result = max_product(input);
        assert_eq!(result, expected);
    }
    // #[test]
    // fn test_group_anagrams_empty_input() {
    //     let input = vec![];
    //     let expected = vec![vec![String::from("")]];
    //     assert_eq!(group_anagrams(input), expected);
    // }
    //
    // #[test]
    // fn test_group_anagrams_single_input() {
    //     let input = vec!["foo".to_string()];
    //     let expected = vec![vec!["foo".to_string()]];
    //     assert_eq!(group_anagrams(input), expected);
    // }
}
