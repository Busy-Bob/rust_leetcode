struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        Vec::new()
    }
    // n: 已经有了几个ip了
    pub fn get_ip_addresses(remain: String, last: String, n: i32) -> Vec<String> {
        let result: Vec<String> = Vec::new();
        match (remain.chars().count(), n >= 4) {
            (0, true) => result.push(last),
            (0, false) => return Vec::new(),
            (1, _) => {
                
                result.append(Self::get_ip_addresses());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
