/*
    2020-12-5
*/

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut map = HashMap::new();
        let length = tasks.len();
        tasks.into_iter().for_each(|c| {
            let temp = map.entry(c).or_insert(0);
            *temp += 1;
        });

        // 记录最大的数字和次数
        let mut best_val = 0;
        let mut times = 0;
        for (key, val) in map {
            if val > best_val {
                times = 1;
                best_val = val;
            } else if val == best_val {
                times += 1;
            }
        }

        (length as i32).max((n + 1) * (best_val - 1) + times)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
            6
        );
    }

    #[test]
    fn it_works3() {
        assert_eq!(
            Solution::least_interval(vec!['A','A','A','A','A','A','B','C','D','E','F','G'], 2),
            16
        );
    }
}
