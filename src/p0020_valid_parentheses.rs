pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for char in s.chars() {
        match char {
            //This has to be done because of how rust treats {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}' | ')' | ']' if stack.pop() != Some(char) => return false,
            _ => (),
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_parentheses() {
        let io = [("()", true), ("()[]{}", true), ("(]", false)];
        io.into_iter().for_each(|(input, output)| {
            assert_eq!(is_valid(input.to_string()), output);
        })
    }
}
