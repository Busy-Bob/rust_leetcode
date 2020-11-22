struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn bus_rapid_transit(
        target: i32,
        inc: i32,
        dec: i32,
        jump: Vec<i32>,
        cost: Vec<i32>,
    ) -> i32 {
        let mut map = HashMap::new();
        let jump = jump.into_iter().map(|x| x as i64).collect();
        let cost = cost.into_iter().map(|x| x as i64).collect();
        (Self::dfs(
            target as i64,
            inc as i64,
            dec as i64,
            &jump,
            &cost,
            &mut map,
        ) % 1000000007) as i32
    }

    pub fn dfs(
        target: i64,
        inc: i64,
        dec: i64,
        jump: &Vec<i64>,
        cost: &Vec<i64>,
        map: &mut HashMap<i64, i64>,
    ) -> i64 {
        // 已经存在，直接返回
        if map.contains_key(&target) {
            return map[&target];
        }
        // 出发点不需要耗时间
        match target {
            0 => return 0,
            1 => return inc, //在1的时候需要之前往前走一步
            _ => (),
        }
        // 还未走过这一点，需要走路或者坐公交
        let mut minTime = target * inc;
        for i in 0..jump.len() {
            // 之前是坐公交，然后向前走或者往后走
            let incDis = target % jump[i]; // 之前是坐公交后向前走了这么多距离
            let incStart = target / jump[i]; // 之前“公交后向前走”的坐公交起点
            let decDis = jump[i] - incDis;
            let decStart = incStart + 1;
            // 之前是向前走到这儿
            if map.contains_key(&incStart) {
                minTime = minTime.min(map[&incStart] + incDis * inc + cost[i]);
            } else {
                // 储存起来
                let costTime = Self::dfs(incStart, inc, dec, jump, cost, map) + incDis * inc + cost[i]; // 之前往前走的cost + 公交cost + 上一点情况。
                minTime = minTime.min(costTime); 
            }
            // 如果之前是向后走的
            if incDis != 0 {
                if map.contains_key(&decStart) {
                    minTime = minTime.min(map[&decStart] + decDis * dec + cost[i]);
                } else {
                    // 储存起来
                    let costTime = Self::dfs(decStart, inc, dec, jump, cost, map) + decDis * dec + cost[i]; // 之前往前走的cost + 公交cost + 上一点情况。
                    minTime = minTime.min(costTime);
                }
            }
        }
        map.insert(target, minTime);
        minTime
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::bus_rapid_transit(612, 4, 5, vec![3,6,8,11,5,10,4], vec![4,7,6,3,7,6,4]), 26);
    }
}
