struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        // 先测试所有字母在不在里面
        let ascii_set: HashSet<char> = word_dict.join("").chars().collect();
        for i in s.chars() {
            if !ascii_set.contains(&i) {
                return vec![];
            }
        }
        // 添加前缀
        let mut exist_prefix: HashSet<String> = HashSet::new();
        for word in word_dict.iter() {
            for i in 0..word.chars().count() {
                exist_prefix.insert(word[0..=i].to_string());
            }
        }
        // 添加单词
        let dictionary: HashSet<String> = word_dict.into_iter().collect();
        // 结果
        let mut result: Vec<String> = Vec::new();
        // 递归
        Self::backtrace(s, Vec::new(), &mut result, &exist_prefix, &dictionary);

        result
    }

    pub fn backtrace(
        s: String,
        mut now_words: Vec<String>,
        result: &mut Vec<String>,
        exist_prefix: &HashSet<String>,
        dictionary: &HashSet<String>,
    ) {
        for k in 0..s.chars().count() {
            if !exist_prefix.contains(&s[0..=k].to_string()) {
                break;
            }
            //存在前缀的情况
            else {
                // 存在单词的情况
                if dictionary.contains(&s[0..=k].to_string()) {
                    let mut temp_now_words = now_words.clone();
                    temp_now_words.push(s[0..=k].to_string());
                    // 结束了的情况
                    if k + 1 == s.chars().count() {
                        result.push(temp_now_words.join(" "));
                    }
                    // 没有结束的情况
                    else {
                        Self::backtrace(
                            s[k + 1..].to_string(),
                            temp_now_words,
                            result,
                            exist_prefix,
                            dictionary,
                        );
                    }
                }
                // 不存在单词的情况
                else {
                    continue;
                }
            }
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
