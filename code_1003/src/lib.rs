struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for &c in s.as_bytes() {
            match c {
                b'a' => stack.push(c),
                b'b' => {
                    if Some(b'a') != stack.pop() {
                        return false;
                    } else {
                        stack.push(c)
                    }
                }
                b'c' => {
                    if Some(b'b') != stack.pop() {
                        return false;
                    }
                }
                _ => panic!(),
            }
        }
        if stack.len() == 0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
