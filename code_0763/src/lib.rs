struct Solution;

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // 储存最后一个字母的位置
        let char_map: HashMap<_, _> = ('a'..='z').map(|x| (x, s.rfind(x))).collect();

        let mut now_pos = 0;
        let mut last_pos = 0; // 上一个序列的末端
        let mut result = Vec::new();
        for (pos, c) in s.chars().enumerate() {
            now_pos = max(char_map[&c].unwrap(), now_pos);
            if pos == now_pos {
                // 该部分已经完成
                result.push((now_pos - last_pos + 1) as i32);
                last_pos = now_pos + 1
            }
        }

        result
    }
}
