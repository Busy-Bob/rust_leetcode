struct Solution;
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![];
        } else if nums.len() == 1 {
            return vec![0];
        }

        let mut index = (0..nums.len()).collect();
        let mut count = vec![0; nums.len()];
        let mut nums = nums;
        let right = nums.len() - 1;
        Self::split_and_cal(&mut nums, 0, right, &mut count, &mut index);
        count
    }

    // reverse_nums 返回逆序对个数
    pub fn split_and_cal(
        nums: &mut Vec<i32>,
        left: usize,
        right: usize,
        count: &mut Vec<i32>,
        index: &mut Vec<usize>,
    ) {
        if left == right {
            return;
        }
        let mid = (left + right) / 2;
        Self::split_and_cal(nums, left, mid, count, index);
        Self::split_and_cal(nums, mid + 1, right, count, index);
        // 计算两个有序数组的合并
        Self::merge(nums, left, mid, right, count, index);
    }

    //  nums[left..mid] 和 nums[mid+1..right]是有序的
    pub fn merge(
        nums: &mut Vec<i32>,
        left: usize,
        mid: usize,
        right: usize,
        count: &mut Vec<i32>,
        index: &mut Vec<usize>,
    ) {
        // 需要辅助数组
        let mut temp = Vec::with_capacity(right - left + 1);
        let mut temp_index = Vec::with_capacity(right - left + 1);
        let mut pos1 = left;
        let mut pos2 = mid + 1;
        while pos1 <= mid || pos2 <= right {
            if pos1 > mid {
                temp.push(nums[pos2]);
                temp_index.push(index[pos2]);
                pos2 += 1;
            } else if pos2 > right {
                temp.push(nums[pos1]);
                temp_index.push(index[pos1]);
                pos1 += 1;
            } else if nums[pos1] <= nums[pos2] {
                temp.push(nums[pos1]);
                temp_index.push(index[pos1]);
                pos1 += 1;
            }
            // 只有当第二个数小的时候才会有逆序对数量, 左侧剩余每个加1
            else {
                temp.push(nums[pos2]);
                temp_index.push(index[pos2]);
                pos2 += 1;
                // 增加
                (pos1..=mid).for_each(|i| count[index[i]] += 1);
            }
        }

        nums.splice(left..=right, temp.into_iter());
        index.splice(left..=right, temp_index.into_iter());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
