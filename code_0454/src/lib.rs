/*
   2020-11-27
   48 ms, 8.8 MB
*/
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut map = HashMap::new();
        // 先存起来
        for &i in a.iter() {
            for &j in b.iter() {
                let temp = map.entry(i + j).or_insert(0);
                *temp += 1;
            }
        }

        // 再检测有没有相反的
        for &i in c.iter() {
            for &j in d.iter() {
                if map.contains_key(&(-i - j)) {
                    res += map[&(-i - j)];
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]), 2);
    }
}
