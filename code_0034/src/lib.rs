/*
    2020-12-1
*/

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut mid = (left + right) / 2;
        // 直到找到
        while left < right {
            if nums[mid] == target {
                break;
            } else if nums[mid] > target {
                right = if mid == 0 { 0 } else { mid - 1 };
            } else {
                left = mid + 1;
            }
            mid = (left + right) / 2;
        }

        // 如果没有找到
        if nums[mid] != target {
            return vec![-1, -1];
        }

        let mut res = Vec::new();
        // 找到了一个需要找左右两边的边
        // 左边
        let mut left_part_left = left;
        let mut left_part_mid = (mid + left) / 2;
        let mut left_part_right = mid;
        // 现在左边部分的最右边==target，所有值小于等于target
        while left_part_mid != left_part_right {
            if nums[left_part_mid] == target {
                left_part_right = left_part_mid;
            }
            // 小于target的时候
            else {
                left_part_left = left_part_mid + 1;
            }
            left_part_mid = (left_part_right + left_part_left) / 2;
        }
        res.push(left_part_right as i32);

        // 右边
        let mut right_part_left = mid;
        let mut right_part_mid = (mid + right + 1) / 2;
        let mut right_part_right = right;
        // 现在右边部分的最左边==target，所有值大于等于target
        while right_part_left != right_part_mid {
            if nums[right_part_mid] == target {
                right_part_left = right_part_mid;
            }
            // 大于target的时候
            else {
                right_part_right = right_part_mid - 1;
            }
            right_part_mid = (right_part_right + right_part_left + 1) / 2;
        }
        res.push(right_part_left as i32);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::search_range(vec![2, 2], 2), vec![0, 1]);
        assert_eq!(Solution::search_range(vec![2, 2], 1), vec![-1, -1]);
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(Solution::search_range(vec![5], 5), vec![0, 0]);
    }
}
