#[allow(dead_code)]
pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .into_iter()
        .map(|i| match (i % 3 == 0, i % 5 == 0) {
            (true, true) => "FizzBuzz".to_string(),
            (true, false) => "Fizz".to_string(),
            (false, true) => "Buzz".to_string(),
            _ => i.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        let mut num: i32 = 3;
        let mut output: Vec<String> = vec!["1".to_string(), "2".to_string(), "Fizz".to_string()];
        // assert!(contains_all(&fizz_buzz(num), &output));
        assert_eq!(fizz_buzz(num), output);

        num = 5;
        output = vec![
            "1".to_string(),
            "2".to_string(),
            "Fizz".to_string(),
            "4".to_string(),
            "Buzz".to_string(),
        ];
        assert_eq!(fizz_buzz(num), output);

        num = 15;
        output = vec![
            "1".to_string(),
            "2".to_string(),
            "Fizz".to_string(),
            "4".to_string(),
            "Buzz".to_string(),
            "Fizz".to_string(),
            "7".to_string(),
            "8".to_string(),
            "Fizz".to_string(),
            "Buzz".to_string(),
            "11".to_string(),
            "Fizz".to_string(),
            "13".to_string(),
            "14".to_string(),
            "FizzBuzz".to_string(),
        ];
        assert_eq!(fizz_buzz(num), output);
    }
}
