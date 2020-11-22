struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        // 因为每一个数都不重复，考虑用hashmap
        let mut map = HashMap::new();
        pieces.iter().enumerate().for_each(|(idx, x)| {
            map.insert(x[0], idx);
        });

        let mut i = 0;
        while i < arr.len() {
            if map.contains_key(&arr[i]) {
                // 开始比较
                let idx = map[&arr[i]];
                map.remove(&arr[i]);
                for j in 1..pieces[idx].len() {
                    i += 1;
                    if i >= arr.len() || pieces[idx][j] != arr[i] {
                        return false;
                    }
                }
            }
            i += 1;
        }

        if map.len() > 0 {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::can_form_array(vec![91, 4, 64, 78], vec![vec![78], vec![4, 64], vec![91]]),
            true
        );
    }
}
