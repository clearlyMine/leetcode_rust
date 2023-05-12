fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>>{
    if strs.len()==0{
        return vec!(vec!(String::from("")));
    }
    if strs.len()==1{
        return vec!(strs.to_vec())
    }
    let mut to_return: Vec<Vec<String>> = Vec::new();
    use std::collections::HashMap;
    let mut map:HashMap<String, Vec<usize>> = HashMap::new();

    for (i, word) in strs.iter().enumerate() {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let sorted_word: String = chars.iter().collect();
        let vec = map.entry(sorted_word).or_insert_with(Vec::new);
        vec.push(i);
    };
    for (_, vec) in map.iter(){
        to_return.push(vec.iter().map(|i| strs[*i].clone()).collect());
    };
    return to_return;
}


#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn test_group_anagrams() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let expected = vec![
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
            vec!["tan".to_string(), "nat".to_string()],
            vec!["bat".to_string()],
        ];
        let result = group_anagrams(input);
        assert_eq!(result.len(), expected.len());
        for v in expected {
            assert!(result.contains(&v));
        }
    }

    #[test]
    fn test_group_anagrams_empty_input() {
        let input = vec![];
        let expected = vec![vec![String::from("")]];
        assert_eq!(group_anagrams(input), expected);
    }

    #[test]
    fn test_group_anagrams_single_input() {
        let input = vec!["foo".to_string()];
        let expected = vec![vec!["foo".to_string()]];
        assert_eq!(group_anagrams(input), expected);
    }
}
