struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut right = nums.len() - 1;
        let mut left = nums.len() - 2;

        loop {
            // 当左边更小的的时候，就可以找这个数右边比这个数稍大一点的了
            if nums[left] < nums[left + 1] {
                while nums[left] >= nums[right] {
                    right -= 1;
                }
                // 交换两个值
                nums.swap(left, right);
                // 此时left后还是保持最大的数，需要变成最小的数, 放在循环外面写
                left += 1;
                break;
            } else {
                if left == 0 {
                    break;
                }
                left -= 1;
            }
        }
        // 开始颠倒
        right = nums.len() - 1;
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut a = vec![1, 2, 3];
        Solution::next_permutation(&mut a);
        assert_eq!(a, vec![1, 3, 2]);
    }
}
