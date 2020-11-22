struct Solution;
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let bits = |mut x: i32| {
            let mut res = 0;
            while x > 0 {
                res += x % 2;
                x = x >> 1;
            }
            res
        };

        let mut arr = arr;
        arr.sort_unstable_by(|&x, &y| {
            if bits(x) == bits(y) {
                x.cmp(&y)
            } else {
                bits(x).cmp(&bits(y))
            }
        });

        arr
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
