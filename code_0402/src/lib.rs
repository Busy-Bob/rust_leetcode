/*
1、retain 的用法
2、 drain的用法， 会删除一个范围，并返回被删除的范围。

3、实际上不理解为“删除” ，理解为“提取”几个会更好写代码
*/

struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        if num.len() <= k as usize {
            return "0".to_string();
        }
        let mut num = num;
        let mut left = 0;
        let mut right = 1;
        let mut remove_index = HashSet::with_capacity(k as usize);
        while right < num.len() {
            // 如果左边的坐标已经被取过了或者已经就跳过。
            if remove_index.contains(&left) {
                left += 1;
                continue;
            }

            // 当左边紧贴的值都已经取过了，就会使得两者一样大
            if left == right {
                right += 1;
                continue;
            }

            // 已经满了就没有必要继续下去了
            if remove_index.len() == k as usize {
                break;
            }
            // 当出现下降的时候, left 这个数据需要被remove掉
            else if num.as_bytes()[left] > num.as_bytes()[right] {
                remove_index.insert(left);
                // println!("{:?}", remove_index);
                while left > 0 {
                    left -= 1;
                    if !remove_index.contains(&left) {
                        break;
                    }
                }
            }
            // 否则就继续搜寻
            else {
                left += 1;
                right += 1;
            }
        }

        // 结束之后没有找完，就从后往前删除相应的
        num.drain((num.len() - k as usize + remove_index.len())..);

        let mut i: usize = 0;
        num.retain(|_| (!remove_index.contains(&i), i += 1).0);

        // 去除前面的0
        let mut end: usize = num.len();
        for i in 0..num.len() {
            if num.as_bytes()[i] == b'0' {
                continue;
            } else {
                end = i;
                break;
            }
        }
        num.drain(..end);

        // 如果全部是0，那么还需要输出
        if num.len() == 0 {
            "0".to_string()
        } else {
            num
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::remove_kdigits("996414363788153611534713021581934201828636847894114849949764848271145953346100425440564".to_string(), 50),
            "0".to_string()
        );
    }
}
