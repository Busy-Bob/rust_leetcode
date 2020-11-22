struct Solution;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let builder = |x: String| -> String {
            let mut stack = String::new();
            for c in x.chars() {
                match c {
                    '#' => {
                        stack.pop();
                    }
                    _ => stack.push(c),
                }
            }
            stack
        };

        if builder(s) == builder(t) {
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
