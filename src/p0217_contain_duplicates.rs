#[allow(dead_code)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut all: HashSet<i32> = HashSet::new();
    !nums.into_iter().all(|n| all.insert(n))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_contains_duplicate() {
        assert! {contains_duplicate(vec![1,2,3,1])};
        assert! {!contains_duplicate(vec![1,2,3,4])};
        assert! {contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2])};
    }
}
