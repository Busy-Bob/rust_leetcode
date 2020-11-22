struct Solution;
impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        // 使用二分法找0
        let mut right_pos: i32 = a.len() as i32 - 1 ;
        let mut left_pos: i32 = 0;
        // 因为可能在相邻位置
        while (right_pos - left_pos) > 1 {
            let index = (right_pos + left_pos) / 2;
            if a[index as usize] > 0 {
                right_pos = index;
            } else {
                left_pos = index;
            }
        }
        // 开始搜索的位置
        let pos = if a[right_pos as usize].abs() < a[left_pos as usize].abs() {
            right_pos
        } else {
            left_pos
        };
        left_pos = pos;
        right_pos = pos;
        result.push(a[pos as usize].pow(2));
        left_pos -= 1;
        right_pos += 1;

        // 开始搜索
        loop {
            match (left_pos < 0, right_pos >= a.len() as i32) {
                (false, false) => {
                    if a[left_pos as usize].pow(2) < a[right_pos as usize].pow(2) {
                        result.push(a[left_pos as usize].pow(2));
                        left_pos -= 1;
                    } else {
                        result.push(a[right_pos as usize].pow(2));
                        right_pos += 1;
                    }
                }
                (false, true) => {
                    result.push(a[left_pos as usize].pow(2));
                    left_pos -= 1;
                }
                (true, false) => {
                    result.push(a[right_pos as usize].pow(2));
                    right_pos += 1;
                }
                (true, true) => break,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::sorted_squares(vec![-4,-1,0,3,10]), vec![0,1,9,16,100]);
    }
}
