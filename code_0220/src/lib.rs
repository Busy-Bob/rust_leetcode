
struct Solution;

// 暴力搜索超时
// impl Solution {
//     pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
//         // 存在 i,j 差小于等于t; 数值差小于等于k
//         if nums.len() < 2 {
//             return false;
//         }
//         let mut i = 0;
//         while i < nums.len() - 1 {
//             for j in (i + 1)..=(i+k as usize).min(nums.len() - 1) {
//                 // 此时会产生溢出。需要判断溢出情况
//                 let mut sub = nums[j] - nums[i];
//                 if sub >= 0 && nums[j] < 0 && nums[i] > 0 {
//                     continue;
//                 } else if sub <= 0 && nums[j] > 0 && nums[i] < 0 {
//                     continue;
//                 }
//                 // 所以用abs取绝对值的时候，-2147483648的绝对值还是-2147483648
//                 if (nums[j] - nums[i]).abs() >= 0 && (nums[j] - nums[i]).abs() <= t {
//                     return true;
//                 }
//             }
//             i += 1;
//         }

//         false
//     }
// }

use std::collections::BTreeSet;
use std::ops::Bound::Included;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        // 存在 i,j 差小于等于k; 数值差小于等于t
        if nums.len() < 2 || k <= 0{
            return false;
        }

        // 转成64位
        let nums: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let k = k as usize;
        let t = t as i64;

        //
        let mut set = BTreeSet::new();
        
        for i in 0..nums.len() {
            match set.len() > 0 {
                false => {set.insert(nums[i]);},
                true => {
                    if set.range((Included(nums[i] - t), Included(nums[i] + t))).count() > 0 {
                        return true
                    } 
                    // 如果已经满了, 重复元素在上面已经解决了
                    if set.len() >= k {
                        set.remove(&nums[i-k]);
                        set.insert(nums[i]);
                    } else {
                        set.insert(nums[i]);
                    }
                },
            }
        }

        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::contains_nearby_almost_duplicate(vec![2147483647, -1, 2147483647], 1, 2147483647), false);
    }
}
