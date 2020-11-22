struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        let mut UP = false;
        let mut res = 0;
        let mut lastPrice = 0;
        for i in 1..prices.len() {
            match (prices[i] >= prices[i - 1], UP) {
                (true, true) => (),
                (true, false) => {
                    lastPrice = prices[i - 1];
                    UP = true;
                }
                (false, true) => {
                    res += prices[i - 1] - lastPrice;
                    UP = false;
                }
                (false, false) => (),
            }
        }
        // 还要加上最后一次
        if UP {
            res += prices.last().unwrap() - lastPrice;
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
