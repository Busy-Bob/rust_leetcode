/*
    1. BTreeMap.range用法以及std::ops::Bound::Excluded, Included, Unbounded等
    2、(key和val的引用也会导致前面的不可变借用时间变长)。 The error is caused by the fact the key and the value are borrowed. The answer is to make a copy of them before calling remove():
    https://stackoverflow.com/questions/55678292/how-can-i-remove-the-first-element-of-a-btreemap-or-hashmap-that-meets-a-conditi

*/

struct Solution;

// 由于可以重复，只能用BTreeMap 加上索引值。
use std::collections::BTreeMap;
use std::ops::Bound::Excluded;
use std::ops::Bound::Unbounded;

impl Solution {
    pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut bMap = BTreeMap::new();
        // 构建b树
        a.into_iter().for_each(|x| {
            let temp = bMap.entry(x).or_insert(0);
            *temp += 1;
        });
        // 查询b中元素对应的最小在a里面的数字。
        let mut res = Vec::with_capacity(b.len());
        for num in b {
            match bMap.range((Excluded(num), Unbounded)).next() {
                Some((&key, &val)) => {
                    res.push(key);
                    if val == 1 {
                        bMap.remove(&key);
                    } else {
                        let temp = bMap.entry(key).or_default();
                        *temp -= 1;
                    }
                }
                // 不存在比他大的就找一个最小的放在这儿。
                None => {
                    let (&key, &val) = bMap.iter().next().unwrap();
                    res.push(key);
                    if val == 1 {
                        bMap.remove(&key);
                    } else {
                        let temp = bMap.entry(key).or_default();
                        *temp -= 1;
                    }
                }
            }
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
