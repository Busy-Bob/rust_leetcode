struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut result = 0;
        Self::backtrace(&s, HashSet::new(), &mut result);

        result
    }

    pub fn backtrace<'a>(s: &'a str, hash_split: HashSet<&'a str>, res: &mut i32) {
        // 进行剪枝!! 80ms -> 4ms
        if hash_split.len() + s.chars().count() <= (*res) as usize {
            return;
        }
        let length = s.chars().count();
        for i in 0..length {
            // 没有重复，则加入并继续迭代
            if !hash_split.contains(&s[0..=i]) {
                let mut hash_split_temp = hash_split.clone();
                hash_split_temp.insert(&s[0..=i]);
                // 还有元素则迭代更新result, 没有元素来就返回当前数量
                if i + 1 != length {
                    Self::backtrace(&s[i + 1..], hash_split_temp, res)
                }
                // 已经迭代完所有了，这是最长的长度。
                else {
                    *res = (*res).max(hash_split_temp.len() as i32);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
