struct Solution;
impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let mut odd_pos = 1;
        let mut even_pos = 0;
        while odd_pos < a.len() && even_pos < a.len() {
            // 先动偶数再动奇数
            match (a[even_pos] % 2, a[odd_pos] % 2) {
                (0, 0) => {
                    even_pos += 2;
                }
                (0, 1) => {
                    even_pos += 2;
                    odd_pos += 2;
                }
                (1, 1) => {
                    odd_pos += 2;
                }
                (1, 0) => {
                    a.swap(even_pos, odd_pos);
                    even_pos += 2;
                    odd_pos += 2;
                }
                _ => panic!(),
            }
        }

        a
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
