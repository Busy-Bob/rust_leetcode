struct Solution;

// 超时
// 用hashmap来存结果。
// use std::collections::HashMap;
// impl Solution {
//     pub fn number_of_sets(n: i32, k: i32) -> i32 {
//         let mut map: HashMap<(i32, i32), i64> = HashMap::new();

//         Self::traceback(n, k, &mut map) as i32
//     }

//     pub fn traceback(n: i32, k: i32, map: &mut HashMap<(i32, i32), i64>) -> i64 {
//         if n - 1 == k || k == 0 {
//             map.insert((n, k), 1);
//             return 1;
//         } else if k == 1 {
//             map.insert((n, k), ((n * n - n) / 2) as i64);
//             return ((n * n - n) / 2) as i64;
//         }
//         // 第一段放在的位置, 放在在i段(区间[0,i])里面, 还需要固定尾部，头部一共就有i种可能性
//         let mut res = 0;
//         for i in 1..=(n - k) as usize {
//             if map.contains_key(&(n - i as i32, k - 1)) {
//                 res += (i as i64) * map[&(n - i as i32, k - 1)];
//             } else {
//                 res += (i as i64) * Self::traceback(n - i as i32, k - 1, map);
//             }
//         }
//         res = res % (10_i64.pow(9) + 7);
//         map.insert((n, k), res);
//         res
//     }
// }

// C_{n+k-1}^{2*k}
impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let mut res = 1_i64;
        ((2 * k + 1)..=(n + k - 1)).for_each(|x| res = (res * x as i64) % 1_000_000_007_i64);

        return res as i32
    }


    // // Lucas定理，求组合数模 C_m^n % _mod
    // pub fn lucas(n: i32, m: i32, _mod: i32) -> i32{
    //     if m == 0 {
    //         1
    //     } else {
    //         Self::lucas(n / _mod, m / _mod, _mod) * 
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::number_of_sets(751, 201), 963678472);
    }
}
