fn main() {
    println!("Hello, world!");
}

struct Solution {
    a: i32
}



// impl Solution {
//     pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
//         if nums.len() == 0 {
//             return vec![];
//         }

//         let mut enumerate_pair = nums.iter().enumerate().collect::<Vec<_>>();
//         enumerate_pair.sort_unstable_by(|a, b| a.1.partial_cmp(b.1).unwrap());

//         let mut result = vec![0; enumerate_pair.len()];
//         let mut rank: i32 = 0;
//         result[enumerate_pair[0].0] = 0;
//         for i in 1..enumerate_pair.len() {
//             if enumerate_pair[i].1 != enumerate_pair[i-1].1 {
//                 rank = i as i32;
//             }
//             result[enumerate_pair[i].0] = rank;
//         }

//         result
//     }
// }

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        // 102 原因是0~100一共101个数字，加上错位1位=102
        let mut freq = vec![0; 102];
        nums.iter().for_each(|&x| freq[(x + 1) as usize] += 1);
        (1..freq.len()).for_each(|x| freq[x] += freq[x-1]);
        nums.iter().map(|&x| freq[x as usize]).collect::<Vec<_>>()
    }
}