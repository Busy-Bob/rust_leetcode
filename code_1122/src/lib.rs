struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        arr1.iter().for_each(|x| {
            let temp = map.entry(x).or_insert(0);
            *temp += 1;
        });

        let mut res = Vec::with_capacity(arr1.len());
        arr2.iter().for_each(|&x| {
            res.append(&mut vec![x; map[&x]]);
            map.remove(&x);
        });


        let mut remain = map.keys().collect::<Vec<_>>();
        remain.sort_unstable();
        remain.iter().for_each(|&&&x| {
            res.append(&mut vec![x; map[&x]]);
        });

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
