struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        // 用hashmap存每一个字符的索引
        let mut map: HashMap<u8, Vec<usize>> = HashMap::new();
        ring.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(idx, x)| map.entry(*x).or_default().push(idx));

        // 用一个闭包来从idx找最优的路径到终点 
        let best_path = |idx: usize, next_pos: &Vec<usize>, next_cost: &Vec<i32>| -> i32 {
            let mut minCost = i32::MAX;
            for i in 0..next_pos.len() {
                // 顺时针还是逆时针
                let mut step = if next_pos[i] < idx {
                    idx - next_pos[i]
                } else {
                    next_pos[i] - idx
                };
                // 取最近的一个转动
                step = step.min(ring.len() - step);
                // 取最小
                minCost = minCost.min(step as i32 + next_cost[i] + 1);
            }
            minCost
        };

        let mut right = key.len() - 1;
        let mut cost = vec![0; map[&(key.as_bytes()[right])].len()];
        while right > 0 {
            cost = map[&(key.as_bytes()[right - 1])]
                .iter()
                .map(|&idx| best_path(idx, &map[&(key.as_bytes()[right])], &cost))
                .collect();
            right -= 1;
        }
        
        // 要从起点开始找到第一个字母的最优。
        best_path(0, &map[&(key.as_bytes()[0])], &cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
