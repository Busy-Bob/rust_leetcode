struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() == 0 {
            return;
        }
        // 记录非零的位置最左边
        let mut none_zero_index = 0;
        let mut index = 0;
        // 开始交换
        'outer: while none_zero_index < nums.len() && index < nums.len() {
            if nums[index] == 0 {
                // 找到下一个非零
                loop {
                    none_zero_index = none_zero_index.max(index) + 1;
                    if none_zero_index >= nums.len() {
                        break 'outer;
                    } else if nums[none_zero_index] != 0 {
                        break;
                    }
                }
                // 交换
                nums.swap(index, none_zero_index);
            }
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
