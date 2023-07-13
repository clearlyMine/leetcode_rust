fn encode(strs: Vec<String>) -> String {
    let mut out: String = String::new();

    for s in &strs {
        out.push_str(s);
        out.push_str("`:`");
    }
    out.remove(out.len() - 1);
    out.remove(out.len() - 1);
    out.remove(out.len() - 1);

    out
}

fn decode(str: String) -> Vec<String> {
    let v: Vec<&str> = str.split("`:`").collect();
    let x = v.iter().map(|x| x.to_string()).collect();
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input: Vec<String> = vec![
            String::from("lint"),
            String::from("code"),
            String::from("love"),
            String::from("you"),
        ];
        let expected: String = String::from("lint`:`code`:`love`:`you");
        let result = encode(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_encode_2() {
        let input: Vec<String> = vec![
            String::from("we"),
            String::from("say"),
            String::from(":"),
            String::from("yes"),
        ];
        let expected: String = String::from("we`:`say`:`:`:`yes");
        let result = encode(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_decode() {
        let input: String = String::from("a`:`b`:`see");
        let expected: Vec<String> = vec![String::from("a"), String::from("b"), String::from("see")];
        let result = decode(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_encode_and_decode() {
        let input: Vec<String> = vec![
            String::from("we"),
            String::from("say"),
            String::from(":"),
            String::from("yes"),
        ];
        let result = decode(encode(input.clone()));
        assert_eq!(result, input);
    }
}
