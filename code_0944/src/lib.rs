struct Solution;
impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        let mut res = 0;
        for col in 0..a[0].len() {
            for row in 1..a.len() {
                if a[row - 1].as_bytes()[col] > a[row].as_bytes()[col] {
                    res += 1;
                    break;
                }
            }
        }
        
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::min_deletion_size(vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()]), 1);
    }
}
