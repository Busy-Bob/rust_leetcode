/*
    2020-12-8
*/
struct Solution;

impl Solution {
    pub fn split_into_fibonacci(s: String) -> Vec<i32> {
        if s.len() < 3 {
            return Vec::new();
        }
        // 每一个长度最多为一半，两个加起来最多最多占2/3
        for len1 in 1..=(s.len() / 2) {
            // 需要判断，否则会overflow
            if let Ok(a) = s[..len1].parse::<i32>() {
                for len2 in 1..=(s.len() / 2).min(s.len() * 2 / 3 - len1) {
                    if let Ok(b) = s[len1..(len1 + len2)].parse::<i32>() {
                        let mut res = Vec::<i32>::new();
                        res.push(a);
                        res.push(b);
                        if Self::backtrace(s[(len1 + len2)..].to_string(), a, b, &mut res) {
                            return res.iter().map(|&x| x as i32).collect();
                        }
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        Vec::new()
    }

    // 对于已知的两个值，确定是否是
    pub fn backtrace(s: String, a: i32, b: i32, array: &mut Vec<i32>) -> bool {
        let c = a + b;
        // 说明溢出了
        if c < 0 {
            return false
        }

        let temp = c.to_string();
        if s.len() >= temp.len() && s[0..temp.len()] == temp {
            array.push(c);
            if s.len() == temp.len() {
                return true;
            } else {
                return Self::backtrace(s[temp.len()..].to_string(), b, c, array);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::split_into_fibonacci("123456579".to_string()),
            vec![123, 456, 579]
        );
    }
    #[test]
    fn it_works2() {
        assert_eq!(
            Solution::split_into_fibonacci("11235813".to_string()),
            vec![1, 1, 2, 3, 5, 8, 13]
        );
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::split_into_fibonacci("0123".to_string()), vec![]);
    }
    #[test]
    fn it_works4() {
        assert_eq!(
            Solution::split_into_fibonacci("1101111".to_string()),
            vec![110, 1, 111]
        );
    }
}
