pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut valid_flag = true;
        let mut pos = vec![0; n as usize];
        let mut stack = Vec::new();
        let mut result = 0;

        let mut row: i32 = 0;
        let mut col: i32 = 0;
        'row: loop {
            // 在第row行第col列落子
            'col: loop {
                pos[row as usize] = col;
                valid_flag = true;
                // 检测0..row前是否与该棋子冲突
                'check: for row_check in (0..row) {
                    if (pos[row as usize] == pos[row_check as usize])
                        || (pos[row as usize] - pos[row_check as usize]).abs() == (row_check - row).abs()
                    {
                        valid_flag = false;
                        break 'check;
                    }
                }

                match (valid_flag, col == (n - 1)) {
                    (true, true) => {
                        // 这一行落点没有啥问题, 第n列的子没有再探索能力了，所以不加入栈
                        row += 1;
                        col = 0;
                        break 'col;
                    }
                    (true, false) => {
                        // 这一行落点没有啥问题, 把当前的状态入栈
                        stack.push((row, col));
                        row += 1;
                        col = 0;
                        break 'col;
                    }
                    (false, false) => {
                        // 如果放在这个row, col下有问题。所以换下一个col
                        col += 1;
                    }
                    (false, true) => {
                        // 如果该行的所有col都不行，就开始回溯
                        if stack.is_empty() {
                            break 'row;
                        }
                        let temp = stack.pop().unwrap();
                        row = temp.0;
                        col = temp.1;
                        col += 1;
                    }
                }
            }

            // 对于一个位置排好的N皇后，再从回溯位置继续开始。
            if row == n {
                result += 1;
                if stack.is_empty() {
                    break 'row;
                }
                let temp = stack.pop().unwrap();
                row = temp.0;
                col = temp.1;
                col += 1;
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
        assert_eq!(Solution::total_n_queens(10), 724);
    }
}
