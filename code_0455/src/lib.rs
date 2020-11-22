struct Solution;
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort();
        s.sort();
        let mut satisfied = 0;
        let mut g_pointer = 0;
        let mut s_pointer = 0;

        while s_pointer < s.len() && g_pointer < g.len() {
            if s[s_pointer] >= g[g_pointer] {
                satisfied += 1;
                s_pointer += 1;
                g_pointer += 1;
            } else {
                s_pointer += 1;
            }
        }

        satisfied
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
