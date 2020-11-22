struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let k: usize = k as usize;
        if s.as_bytes().len() < k {
            return s;
        }

        let mut chars = s.as_bytes().to_vec();
        let mut times = vec![0; 26];
        let mut left = 0;
        let mut right = 0;

        while right < chars.len() {
            if right - left >= k {
                let c = chars[left] as usize - 'a' as usize;
                times[c] -= 1;
                left += 1;
            }

            let c = chars[right] as usize - 'a' as usize;
            times[c] += 1;
            // 超过了k个
            if times[c] >= k {
                chars.drain(left..=right);
                // 复原
                times[c] = 0;
                left = if left + 1 >= k { left + 1  - k } else { 0 };
                right = left;
            }
            // 没有匹配到
            else {
                right += 1;
            }
        }

        std::str::from_utf8(&chars).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::remove_duplicates("deeedbbcccbdaa".to_string(), 3), "aa".to_string());
    }
}
