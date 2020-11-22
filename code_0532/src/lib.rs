struct Solution;

// 类似于两数之和, 尤其注意k=0情况，去重可以只存大值或者小值
use std::collections::HashSet;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut set = HashSet::new();
        let mut pair = HashSet::new();
        nums.into_iter().for_each(|x| {
            if set.contains(&(x + k)) {
                pair.insert(x + k);
            }
            if set.contains(&(x - k)) {
                pair.insert(x);
            }
            set.insert(x);
        });

        pair.len() as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
