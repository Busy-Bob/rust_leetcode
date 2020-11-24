/*
    2020-11-25
*/

struct Solution;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut count = vec![0; 26];
        // 先把个数存到数组里面
        s.as_bytes().iter().for_each(|&x|  count[x as usize - 'a' as usize] += 1 );
        let mut res = Vec::<u8>::new();

        while res.len() < s.len() {
            // 先上升
            for i in 0..count.len() {
                if count[i] > 0 {
                    res.push(i as u8 + 'a' as u8);
                    count[i] -= 1;
                }
            }
            // 再下降
            for i in (0..count.len()).rev() {
                if count[i] > 0 {
                    res.push(i as u8 + 'a' as u8);
                    count[i] -= 1;
                }
            }
        }

        res.into_iter().map(|x| x as char).collect::<String>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::sort_string("aaaabbbbcccc".to_string()), "abccbaabccba".to_string());
        assert_eq!(Solution::sort_string("rat".to_string()), "art".to_string());
        assert_eq!(Solution::sort_string("leetcode".to_string()), "cdelotee".to_string());
        assert_eq!(Solution::sort_string("ggggggg".to_string()), "ggggggg".to_string());
        assert_eq!(Solution::sort_string("spo".to_string()), "ops".to_string());
    }
}
