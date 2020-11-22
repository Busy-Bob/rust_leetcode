struct Solution;
impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        } else if  nums.len() == 1 {
            return (nums[0] <= upper && nums[0] >= lower) as i32;
        }

        let mut preSum = nums
            .into_iter()
            .scan(0i64, |s, t| {
                *s = *s + t as i64;
                Some(*s)
            })
            .collect::<Vec<i64>>();
        // 前缀和最前面有一个0
        preSum.insert(0, 0);
        let right = preSum.len() - 1;

        Self::count_and_sort(&mut preSum, 0, right, lower as i64, upper as i64)
    }

    // 区间为nums[left..=right]
    pub fn count_and_sort(
        preSum: &mut std::vec::Vec<i64>,
        left: usize,
        right: usize,
        lower: i64,
        upper: i64,
    ) -> i32 {
        if left == right {
            return 0;
        }
        let mid = (left + right) / 2;
        let left_count = Self::count_and_sort(preSum, left, mid, lower, upper);
        let right_count = Self::count_and_sort(preSum, mid + 1, right, lower, upper);

        // 开始合并和计算
        let mut res = left_count + right_count;
        let mut i = left;
        let mut j = mid + 1;

        // i..=j 的和
        for i in left..=mid {
            while j <= right {
                if preSum[j] - preSum[i] >= lower {
                    break;
                }
                j += 1;
            }
            for j2 in j..=right {
                if preSum[j2] - preSum[i] <= upper {
                    res += 1;
                } else {
                    break;
                }
            }
        }

        // 合并
        let mut i = left;
        let mut j = mid + 1;
        let mut temp = Vec::with_capacity(right - left + 1);
        while i <= mid || j <= right {
            if i > mid {
                temp.push(preSum[j]);
                j += 1;
            } else if j > right {
                temp.push(preSum[i]);
                i += 1;
            } else if preSum[i] <= preSum[j] {
                temp.push(preSum[i]);
                i += 1;
            } else {
                temp.push(preSum[j]);
                j += 1;
            }
        }
        preSum.splice(left..=right, temp.into_iter());

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
