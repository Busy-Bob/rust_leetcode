struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        arr.iter().for_each(|&x| {
            let count = map.entry(x).or_insert(0);
            *count += 1;
        });
        // 将个数也存在一个hash_map里面
        let mut num = HashMap::new();
        map.iter().for_each(|(_, &val)| {
            let count = num.entry(val).or_insert(0);
            *count += 1;
        });
        // 判断个数hash_map里面是不是全部为1
        num.iter().all(|(_, &val)| val == 1)
    }
}
