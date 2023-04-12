struct Solution;
impl Solution {
    fn binary_search(nums: &[i32], target: i32, result: &mut Vec<i32>, l: i32, r: i32) {
        if l <= r {
            let mid = l + (r - l) / 2;

            if nums[mid as usize] == target {
                result.push(mid);
                Self::binary_search(nums, target, result, l, mid - 1);
                Self::binary_search(nums, target, result, mid + 1, r);
            } else if nums[mid as usize] < target {
                Self::binary_search(nums, target, result, mid + 1, r);
            } else {
                Self::binary_search(nums, target, result, l, mid - 1);
            }
        }
    }

    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort();

        let mut result: Vec<i32> = Vec::new();
        Self::binary_search(&nums, target, &mut result, 0, nums.len() as i32 - 1);
        result.sort();
        result
    }

    pub fn target_indices_old(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort();

        let mut result: Vec<i32> = Vec::new();

        let (mut l, mut r) = (0, nums.len() as i32 - 1);

        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid as usize] == target {
                result.push(mid);
            }
            if nums[mid as usize] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_i() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 2;
        assert_eq!(Solution::target_indices(nums, target), vec![1, 2]);
    }
    #[test]
    fn test_case_ii() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 3;
        assert_eq!(Solution::target_indices(nums, target), vec![3]);
    }

    #[test]
    fn test_case_iii() {
        let nums = vec![1, 2, 5, 2, 3];
        let target = 5;
        assert_eq!(Solution::target_indices(nums, target), vec![4]);
    }
}
