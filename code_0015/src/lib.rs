struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return Vec::new();
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut result: Vec<Vec<i32>> = Vec::new();

        // 最后三个数就不迭代了
        for (index, &value) in nums.iter().take(nums.len() - 2).enumerate() {
            // 最小数字大于等于0了，不可能出现三数之和=0
            if value > 0 {
                break;
            }
            // 第一个数去重
            if index >= 1 && value == nums[index - 1] {
                continue;
            }
            let mut left = index + 1;
            let mut right = nums.len() - 1;
            'order: while left < right {
                if nums[right] < 0 {
                    break;
                }
                let temp = nums[left] + nums[right] + value;
                if temp == 0 {
                    result.push(vec![value, nums[left], nums[right]]);
                    // 第二个数去重
                    loop {
                        left += 1;
                        if left >= right {
                            break 'order;
                        }
                        if nums[left] != nums[left - 1] {
                            break;
                        }
                    }
                    // 第三个数去重
                    loop {
                        right -= 1;
                        if left >= right {
                            break 'order;
                        }
                        if nums[right] != nums[right + 1] {
                            break;
                        }
                    }
                } else if temp < 0 {
                    // 第二个数去重
                    loop {
                        left += 1;
                        if left >= right {
                            break 'order;
                        }
                        if nums[left] != nums[left - 1] {
                            break;
                        }
                    }
                } else if temp > 0 {
                    // 第三个数去重
                    loop {
                        right -= 1;
                        if left >= right {
                            break 'order;
                        }
                        if nums[right] != nums[right + 1] {
                            break;
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    
    fn it_works() {
        assert_eq!(Some(Solution::three_sum(vec![34,55,79,28,46,33,2,48,31,-3,84,71,52,-3,93,15,21,-43,57,-6,86,56,94,74,83,-14,28,-66,46,-49,62,-11,43,65,77,12,47,61,26,1,13,29,55,-82,76,26,15,-29,36,-29,10,-70,69,17,49])), None);
    }
}
