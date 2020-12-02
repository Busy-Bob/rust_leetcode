struct Solution;

// 取两个中最大的，下一个如果更大，则替代，直到个数不够为止。
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        // 需要删掉的个数
        let mut to_delete = nums1.len() as i32 + nums2.len() as i32 - k;

        let mut res = Vec::<i32>::new();

        for k1 in 0.max(k - nums2.len() as i32)..=k.min(nums1.len() as i32) {
            let k2 = k - k1;
            let nums1_temp = Self::max_single_vector(&nums1, k1);
            let nums2_temp = Self::max_single_vector(&nums2, k2);
            let temp = Self::merge(nums1_temp, nums2_temp);

            if Self::compare(&res, &temp, 0, 0) {
                res = temp;
            }
        }

        res
    }

    // 使用单调栈求一个vector的最大排列
    pub fn max_single_vector(nums: &Vec<i32>, k: i32) -> Vec<i32> {
        let mut to_delete = nums.len() as i32 - k;
        let mut res = Vec::with_capacity(k as usize);

        for &x in nums {
            // 删除掉比当前值小的，
            while to_delete > 0 {
                match res.last() {
                    Some(&y) if y < x => {
                        res.pop();
                        to_delete -= 1;
                    }
                    _ => break,
                }
            }
            res.push(x);
        }

        // 最后不够就从最后删除
        while to_delete > 0 {
            res.pop();
            to_delete -= 1;
        }


        res
    }

    // merge 两个如果相等，还需要比较后面谁大
    pub fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1_pos = 0;
        let mut nums2_pos = 0;
        let mut res = Vec::new();
        while nums1_pos < nums1.len() || nums2_pos < nums2.len() {
            if Self::compare(&nums1, &nums2, nums1_pos, nums2_pos) {
                res.push(nums2[nums2_pos]);
                nums2_pos += 1;
            } else {
                res.push(nums1[nums1_pos]);
                nums1_pos += 1;
            }
        }

        res
    }

    // 比较谁更小
    pub fn compare(nums1: &Vec<i32>, nums2: &Vec<i32>, pos1: usize, pos2: usize) -> bool {
        if pos1 >= nums1.len() {
            return true;
        }
        if pos2 >= nums2.len() {
            return false;
        }
        if nums1[pos1] < nums2[pos2] {
            return true;
        } else if nums1[pos1] > nums2[pos2] {
            return false;
        } else {
            return Self::compare(nums1, nums2, pos1 + 1, pos2 + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
            vec![9, 8, 6, 5, 3]
        );
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            Solution::max_number(vec![6, 7], vec![6, 0, 4], 5),
            vec![6, 7, 6, 0, 4]
        );
    }
}
