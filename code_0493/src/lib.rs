/*
    2020-11-28
*/

struct Solution;

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let mut nums = nums;
        let right = nums.len() - 1;
        Self::reverse_nums(&mut nums, 0, right)
    }

    // reverse_nums 返回逆序对个数
    pub fn reverse_nums(nums: &mut Vec<i32>, left: usize, right: usize) -> i32 {
        if left == right {
            return 0;
        }

        let mid = (left + right) / 2;
        let left_val = Self::reverse_nums(nums, left, mid);
        let right_val = Self::reverse_nums(nums, mid + 1, right);

        // 计算两个有序数组的合并
        let merge_val = Self::merge(nums, left, mid, right);
        return left_val + right_val + merge_val;
    }

    // 归并时候返回逆序对数量 nums[left..mid] 和 nums[mid+1..right]是有序的
    pub fn merge(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) -> i32 {
        // 先提前计算一下重要对个数
        let mut res = 0;
        let mut pos1 = left;
        let mut pos2 = mid + 1;
        while pos2 <= right {
            while pos1 <= mid {
                if nums[pos1] as i64 > 2 * nums[pos2] as i64 {
                    res += mid - pos1 + 1;
                    break;
                } 
                pos1 += 1;
            }
            pos2 += 1;
        }

        // 需要辅助数组，进行交换
        let mut temp = Vec::with_capacity(right - left + 1);
        let mut pos1 = left;
        let mut pos2 = mid + 1;
        
        while pos1 <= mid || pos2 <= right {
            if pos1 > mid {
                temp.push(nums[pos2]);
                pos2 += 1;
            } else if pos2 > right {
                temp.push(nums[pos1]);
                pos1 += 1;
            } 
            else if nums[pos1] <= nums[pos2] {
                temp.push(nums[pos1]);
                pos1 += 1;

            }
            else {
                temp.push(nums[pos2]);
                pos2 += 1;
            }
        }

        nums.splice(left..=right, temp.into_iter());

        res as i32
    }
}




#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_pairs(vec![1,3,2,3,1]), 2);
        assert_eq!(Solution::reverse_pairs(vec![2,4,3,5,1]), 3);
    }
}
