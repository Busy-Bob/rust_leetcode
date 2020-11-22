struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        // 放入栈中
        word_list.iter().for_each(|s| {
            for i in 0..s.chars().count() {
                let mut temp = s.chars().collect::<Vec<_>>();
                temp[i] = '*';
                map.entry(temp.into_iter().collect())
                    .or_insert(Vec::new())
                    .push(s.clone());
            }
        });

        // BFS
        // println!("{:?}", map);
        let mut front_queue: VecDeque<(String, i32)> = VecDeque::new();
        let mut back_queue: VecDeque<(String, i32)> = VecDeque::new();
        let mut front_map: HashMap<String, i32> = HashMap::new();
        let mut back_map: HashMap<String, i32> = HashMap::new();

        let temp = word_list.clone().into_iter().collect::<HashSet<String>>();
        if !temp.contains(&end_word) {
            return 0;
        }

        front_queue.push_back((begin_word.clone(), 1));
        back_queue.push_back((end_word.clone(), 1));
        front_map.insert(begin_word.clone(), 1);
        back_map.insert(end_word.clone(), 1);

        let mut res = 0;
        loop {
            // 前向
            if let Some((word, num)) = front_queue.pop_front() {
                // 如果现在反向集合包含了，说明已经反向遍历过该节点了，直接输出结果。
                if !back_map.contains_key(&word) {
                    for j in 0..word.chars().count() {
                        let mut temp = word.chars().collect::<Vec<char>>();
                        temp[j] = '*';
                        let temp2 = temp.iter().collect::<String>();
                        for s in map
                            .entry(temp2)
                            .or_default()
                            .iter()
                            .filter(|&s| !front_map.contains_key(s))
                            .collect::<Vec<_>>()
                        {
                            if back_map.contains_key(s) {
                                return back_map[s] + num;
                            }
                            front_queue.push_back((s.clone(), num + 1));
                            front_map.insert(s.clone(), num + 1);
                        }
                    }
                } else {
                    res = back_map[&word] + num - 1;
                    break;
                }
            } else {
                res = 0;
                break;
            }

            // 反向
            if let Some((word, num)) = back_queue.pop_front() {
                // 如果现在正向集合没有包含了，说明已经正向遍历过该节点了，直接输出结果。没有能够成功移除代表已经遍历过了，所以此时不做操作
                if !front_map.contains_key(&word) {
                    back_map.insert(word.clone(), num);
                    for j in 0..word.chars().count() {
                        let mut temp = word.chars().collect::<Vec<_>>();
                        temp[j] = '*';
                        for s in map[&temp.iter().collect::<String>()]
                            .iter()
                            .filter(|&s| !back_map.contains_key(s))
                            .collect::<Vec<_>>()
                        {
                            if front_map.contains_key(s) {
                                return front_map[s] + num;
                            }
                            back_queue.push_back((s.clone(), num + 1));
                            back_map.insert(s.clone(), num + 1);
                        }
                    }
                } else {
                    res = front_map[&word] + num - 1;
                    break;
                }
            } else {
                res = 0;
                break;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::ladder_length(
                "lost".to_string(),
                "cost".to_string(),
                vec!["lost".to_string(), "cost".to_string(),]
            ),
            2
        );
    }
}
