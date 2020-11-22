struct Solution;

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 按照身高降序，排序升序。
        let mut people = people;
        people.sort_unstable_by(|x, y| (-x[0], x[1]).cmp(&(-y[0], y[1])));
        // 依次插入
        let mut res = Vec::new();
        for one in people {
            res.insert(one[1] as usize, one);
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
