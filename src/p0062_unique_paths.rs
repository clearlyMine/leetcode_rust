use std::collections::VecDeque;

//TLE
pub fn unique_paths_dfs(m: i32, n: i32) -> i32 {
    let (m, n) = (m - 1, n - 1);
    let mut stack: Vec<(i32, i32)> = vec![(0, 0)];
    let mut total_paths = 0;
    while let Some(next) = stack.pop() {
        if next == (m, n) {
            total_paths += 1;
            continue;
        }
        if next.0 < m {
            stack.push((next.0 + 1, next.1));
        }
        if next.1 < n {
            stack.push((next.0, next.1 + 1));
        }
    }
    total_paths
}

//MLE
pub fn unique_paths_bfs(m: i32, n: i32) -> i32 {
    let (m, n) = (m - 1, n - 1);
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back((0, 0));
    let mut total_paths = 0;
    while let Some(next) = queue.pop_front() {
        if next == (m, n) {
            total_paths += 1;
            continue;
        }
        if next.0 < m {
            queue.push_back((next.0 + 1, next.1));
        }
        if next.1 < n {
            queue.push_back((next.0, next.1 + 1));
        }
    }
    total_paths
}

//https://leetcode.com/problems/unique-paths/solutions/3994523/98-83-easy-dp-math/
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut ans: i64 = 1;
    for i in 1..m as i64 {
        ans = ans * (n as i64 - 1 + i) / i;
    }
    ans as i32
}

pub fn unique_paths_space_optimized(m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let mut pre: Vec<i32> = vec![1; n];
    for _ in 1..m {
        for j in 1..n {
            pre[j] += pre[j - 1];
        }
    }
    pre[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unique_paths() {
        let input = (3, 7);
        let output = 28;
        assert_eq!(unique_paths(input.0, input.1), output);

        let input = (3, 2);
        let output = 3;
        assert_eq!(unique_paths(input.0, input.1), output);

        let input = (23, 12);
        let output = 193536720;
        assert_eq!(unique_paths(input.0, input.1), output);
    }

    #[test]
    fn test_unique_paths_space_optimized() {
        let input = (3, 7);
        let output = 28;
        assert_eq!(unique_paths_space_optimized(input.0, input.1), output);

        let input = (3, 2);
        let output = 3;
        assert_eq!(unique_paths_space_optimized(input.0, input.1), output);

        let input = (23, 12);
        let output = 193536720;
        assert_eq!(unique_paths_space_optimized(input.0, input.1), output);
    }
}
