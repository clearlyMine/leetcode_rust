pub fn rob(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    fn robber(nums: &[i32], lo: usize, hi: usize) -> i32 {
        let (mut a, mut b) = (0, 0);
        for i in lo..=hi {
            let n = nums[i];
            let temp = b.max(n + a);
            a = b;
            b = temp;
        }
        b
    }

    match len {
        1 => nums[0],
        2 => nums[0].max(nums[1]),
        _ => robber(&nums, 0, len - 2).max(robber(&nums, 1, len - 1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        let io = [
            (vec![2, 3, 2], 3),
            (vec![1, 2, 3, 1], 4),
            (vec![0], 0),
            (vec![0, 0], 0),
            (vec![0, 0, 0], 0),
            (vec![1, 2, 1, 1], 3),
            (vec![1, 1, 1, 2], 3),
        ];
        io.into_iter().for_each(|(input, output)| {
            assert_eq!(rob(input), output);
        });
    }
}
