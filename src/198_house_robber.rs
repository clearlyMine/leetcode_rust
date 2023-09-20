fn main() {
    println!("{}", rob(vec![]));
}

fn rob(nums: Vec<i32>) -> i32 {
    let (mut a, mut b) = (0, 0);

    for n in nums {
        let temp = b.max(n + a);
        a = b;
        b = temp;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        let io = [
            (vec![1, 2, 3, 1], 4),
            (vec![2, 7, 9, 3, 1], 12),
            (vec![2, 1, 1, 2], 4),
            (vec![1, 3, 1], 3),
            (vec![1, 1, 1], 2),
            (vec![6, 3, 10, 8, 2, 10, 3, 5, 10, 5, 3], 39),
            (vec![6, 6, 4, 8, 4, 3, 3, 10], 27),
            (vec![1, 1, 3, 6, 7, 10, 7, 1, 8, 5, 9, 1, 4, 4, 3], 42),
        ];
        io.into_iter().for_each(|(input, output)| {
            assert_eq!(rob(input), output);
        });
    }
}
