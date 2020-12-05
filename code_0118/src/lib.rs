/*
    2020-12-6
*/

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows < 1 {
            return Vec::new();
        }
        let mut res = Vec::new();
        res.push(vec![1]);
        // 逐渐增加
        for i in 1..num_rows as usize {
            let mut temp = Vec::with_capacity(i as usize+1);
            temp.push(1);
            for j in 1..i {
                temp.push(res[i-1][j] + res[i-1][j-1]);
            }
            temp.push(1);
            res.push(temp);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
