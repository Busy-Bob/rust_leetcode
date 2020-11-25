/*
    2020-11-26
    存在N个桶里面其实会好表示一些
*/

struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let &max = nums.iter().max().unwrap();
        let &min = nums.iter().min().unwrap();
        let length = nums.len() as i32;
        // 使用N-1个桶, 桶的范围
        let min_gap = (max - min) as f64 / (length - 1) as f64;
        // [(最小值，最大值)， ]， 用i64是害怕超了。
        let mut vec = vec![vec![std::i64::MAX, std::i64::MIN]; length as usize - 1];
        //  挨个储存
        for num in nums {
            let mut index: usize = ((num - min) as f64 / min_gap).floor() as usize;
            // 对于max这个数字要额外处理，放在最后一个
            if index == length as usize - 1 {
                index -= 1;
            }
            // 取最大最小值
            if vec[index][0] > num as i64 {
                vec[index][0] = num as i64;
            }
            if vec[index][1] < num as i64 {
                vec[index][1] = num as i64;
            }
        }

        // 再挨个比较，也可以为空
        let mut res = min_gap as i64;
        let mut last = min as i64;
        for temp in vec {
            // 如果没有数字，那么就下一个
            if temp[0] == std::i64::MAX {
                continue;
            }
            // 有数字
            res = if res < temp[0] - last {
                temp[0] - last
            } else {
                res
            };
            last = temp[1];
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
        assert_eq!(Solution::maximum_gap(vec![1,1000]), 999);
        assert_eq!(Solution::maximum_gap(vec![2,99999999]), 99999997);
    }
}
