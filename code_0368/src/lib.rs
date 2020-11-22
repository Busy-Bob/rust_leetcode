struct Solution;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![];
        }

        let mut nums = nums;
        let mut eds = vec![1; nums.len()];
        nums.sort_unstable();

        // 记录在每个地方为止是否最长的长度，用于剪枝
        let mut max_flag = vec![false; nums.len()];
        let mut max_length = 1;
        for i in 0..nums.len() {
            for j in (0..i).rev() {
                if nums[i] % nums[j] == 0 {
                    eds[i] = eds[i].max(eds[j] + 1);
                    if max_flag[j] {
                        break;
                    }
                }
            }

            max_length = if eds[i] >= max_length {
                max_flag[i] = true;
                eds[i]
            } else {
                max_length
            };
        }

        let (max_i, &max_eds) = eds.iter().enumerate().max_by_key(|(_, &val)| val).unwrap();
        let mut max_eds = max_eds;
        let mut now_val = nums[max_i];
        let mut result = vec![now_val];
        for i in (0..max_i).rev() {
            if now_val % nums[i] == 0 && eds[i] == max_eds - 1 {
                now_val = nums[i];
                max_eds = eds[i];
                result.push(now_val);
            }
        }

        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::largest_divisible_subset(vec![4, 8, 10, 240]),
            vec![4, 8, 240]
        );
    }
}
