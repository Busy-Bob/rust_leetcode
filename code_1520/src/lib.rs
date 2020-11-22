struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let mut map: HashMap<char, (usize, usize)> = HashMap::new();
        // 先给定初值, start 为最大， end 为最小
        s.chars().for_each(|c| {
            map.insert(c, (usize::MAX, usize::MIN));
        });
        // 确定字符出现的范围
        for (index, c) in s.chars().enumerate() {
            // 开始
            *map.get_mut(&c).unwrap() = (map[&c].0.min(index), map[&c].1.max(index));
        }
        // 之后需要确定内部字符的最大的范围
        let char_set: HashSet<char> = HashSet::from_iter(s.chars());
        let mut map_length: HashMap<char, (usize, usize)> = HashMap::new();

        for c in char_set {
            let mut start = map[&c].0;
            let mut end = map[&c].1;
            let mut length = end - start + 1;
            let mut set = HashSet::new(); // 记录其中有啥字母了
            set.insert(c);
            map_length.entry(c).or_insert(map[&c]);
            loop {
                let temp_s: String = s.chars().skip(start).take(length).collect();
                let temp_set: HashSet<char> = HashSet::from_iter(temp_s.chars());
                match temp_set == set {
                    true => break, // 已经完成了字母
                    false => {
                        // 找到最远的加进去
                        end = temp_set
                            .difference(&set)
                            .map(|x| map[x].1)
                            .max()
                            .unwrap()
                            .max(end);
                        start = temp_set
                            .difference(&set)
                            .map(|x| map[x].0)
                            .min()
                            .unwrap()
                            .min(start);
                        *map_length.get_mut(&c).unwrap() = (start, end);
                        length = end - start + 1;
                        set = temp_set;
                    }
                }
            }
        }
        // 将所有端点进行排序。
        let mut temp_vec = map_length.iter().map(|(_, &v)| v).collect::<Vec<_>>();
        temp_vec.sort_unstable_by(|a, b| match a.1 != b.1 {
            true => a.1.cmp(&b.1),
            false => (a.1 - a.0).cmp(&(b.1 - b.0)),
        });
        // println!("{:?}",map_length);
        // println!("{:?}",temp_vec);
        // 贪婪算法给出输出
        let mut now_end = 0; // 记录后一个位置
        let mut result = Vec::new();
        for pos in temp_vec {
            match pos.0 >= now_end {
                true => {
                    result.push(s.chars().skip(pos.0).take(pos.1 - pos.0 + 1).collect());
                    now_end = pos.1 + 1;
                }
                false => continue,
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
