fn main(){
    println!(
        "{:?} should be true",
        contains_duplicate(vec![1,2,3,1])
    );
    println!(
        "{:?} should be false",
        contains_duplicate(vec![1,2,3,4])
    );
    println!(
        "{:?} should be true",
        contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2])
    );
}

fn contains_duplicate(nums: Vec<i32>) -> bool{
   use std::collections::HashSet;
   let mut all: HashSet<i32> = HashSet::new();
   !nums.into_iter().all(|n| all.insert(n))
}
