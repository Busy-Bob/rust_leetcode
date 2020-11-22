struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut result_map = vec![HashMap::new(); a.len()];
        let mut result: Vec<String> = Vec::new();
        for (index, string) in a.iter().enumerate() {
            string.chars().for_each(|c| {
                let count = result_map[index].entry(c).or_insert(0);
                *count += 1;
            });
        }

        for c in 'a'..='z' {
            let repeat = result_map
                .iter()
                .map(|x| *x.get(&c).unwrap_or(&0))
                .min()
                .unwrap();
            for i in 0..repeat {
                result.push(c.to_string());
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
