#[allow(dead_code)]
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut s: Vec<String> = vec!["".to_string(); n as usize];
    for i in 1..=n as usize {
        let index = i - 1;
        if i % 3 == 0 {
            s[index] = "Fizz".to_string();
        }
        if i % 5 == 0 {
            if s[index] == "" {
                s[index] = "Buzz".to_string();
            } else {
                s[index] += "Buzz";
            }
        }
        if s[index] == "" {
            s[index] = i.to_string();
        }
    }
    s
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
