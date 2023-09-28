//1ms 1.98MB ( 73.53% 100.00%)
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 1 {
        return if nums[0] == target { 0 } else { -1 };
    }
    fn binary_search(nums: &Vec<i32>, low: usize, high: usize, target: i32) -> i32 {
        let mut low = low as i32;
        let mut high = high as i32;
        while low <= high {
            let mid = (high - low) / 2 + low;
            let val = nums[mid as usize];
            if val == target {
                return mid as i32;
            }
            if val > target {
                high = mid - 1;
            } else if val < target {
                low = mid + 1;
            }
        }
        -1
    }
    if nums.first() < nums.last() {
        //non-rotated array
        //binary-search for the target
        return binary_search(&nums, 0, nums.len() - 1, target);
    } else {
        let (mut start, mut end) = (1, 0);
        while nums[start] > nums[end] {
            start += 1;
            end += 1;
        }
        let mut x = binary_search(&nums, start, nums.len() - 1, target);
        if x != -1 {
            return x;
        }
        x = binary_search(&nums, 0, end, target);
        return x;
    }
}

//https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/3879263/100-binary-search-easy-video-o-log-n-optimal-solution/
//1ms 2.36MB ( 73.57%  16.80%)
pub fn search_copied(nums: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = nums.len() as i32 - 1;

    while low <= high {
        let mid = (low + high) / 2;

        if nums[mid as usize] == target {
            return mid;
        }

        if nums[low as usize] <= nums[mid as usize] {
            if nums[low as usize] <= target && target < nums[mid as usize] {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        } else {
            if nums[mid as usize] < target && target <= nums[high as usize] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_rotated_sorted_array() {
        let io = [
            ((vec![4, 5, 6, 7, 0, 1, 2], 0), 4),
            ((vec![4, 5, 6, 7, 0, 1, 2], 3), -1),
            ((vec![1], 0), -1),
            ((vec![1, 3], 0), -1),
            ((vec![1, 3], 3), 1),
        ];
        io.into_iter().for_each(|((num, target), out)| {
            assert_eq!(search(num, target), out);
        });
    }

    #[test]
    fn test_search_rotated_sorted_array_copied() {
        let io = [
            ((vec![4, 5, 6, 7, 0, 1, 2], 0), 4),
            ((vec![4, 5, 6, 7, 0, 1, 2], 3), -1),
            ((vec![1], 0), -1),
            ((vec![1, 3], 0), -1),
            ((vec![1, 3], 3), 1),
            ((vec![7, 8, 0, 1, 2, 3, 4, 5, 6], 0), 2),
            ((vec![7, 8, 0, 1, 2, 3, 4, 5, 6], 3), 5),
            ((vec![4, 5, 6, 7, 0, 1, 2], 2), 6),
            ((vec![6, 7, 1, 2, 3, 4, 5], 7), 1),
        ];
        io.into_iter().for_each(|((num, target), out)| {
            assert_eq!(search_copied(num, target), out);
        });
    }
}
