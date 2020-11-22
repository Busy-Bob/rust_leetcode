struct Solution;
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        // 初始化访问访问矩阵
        let mut visit_flag: Vec<Vec<bool>> = Vec::new();
        grid.iter()
            .for_each(|x| visit_flag.push(vec![false; x.len()]));
        // 找到一个陆地块
        let mut row = 0;
        let mut col = 0;
        let row_max = grid.len();
        let col_max = grid[row].len();
        for i in 0..grid.len() {
            for j in 0..grid[row].len() {
                if grid[i][j] == 1 {
                    row = i;
                    col = j;
                    break;
                }
            }
        }
        // 从起始陆地开始递归
        let mut length = 0;
        Self::recursion(
            row,
            col,
            row_max,
            col_max,
            &grid,
            &mut visit_flag,
            &mut length,
        );
        length
    }

    pub fn recursion(
        row: usize,
        col: usize,
        row_max: usize,
        col_max: usize,
        grid: &Vec<Vec<i32>>,
        mut visit_flag: &mut Vec<Vec<bool>>,
        length: &mut i32,
    ) {
        // 先给自己标记访问
        visit_flag[row][col] = true;
        // 向上
        if row != 0 {
            if grid[row - 1][col] == 0 {
                // 如果是海
                *length += 1;
            } else if !visit_flag[row - 1][col] {
                // 如果还没访问, 访问
                Self::recursion(
                    row - 1,
                    col,
                    row_max,
                    col_max,
                    &grid,
                    &mut visit_flag,
                    length,
                )
            }
        } else {
            //在边上
            *length += 1;
        }
        // 向下
        if row + 1 < row_max {
            if grid[row + 1][col] == 0 {
                // 如果是海
                *length += 1;
            } else if !visit_flag[row + 1][col] {
                // 如果还没访问, 访问
                Self::recursion(
                    row + 1,
                    col,
                    row_max,
                    col_max,
                    &grid,
                    &mut visit_flag,
                    length,
                )
            }
        } else {
            //在边上
            *length += 1;
        }

        // 向左
        if col != 0 {
            if grid[row][col - 1] == 0 {
                // 如果是海
                *length += 1;
            } else if !visit_flag[row][col - 1] {
                // 如果还没访问, 访问
                Self::recursion(
                    row,
                    col - 1,
                    row_max,
                    col_max,
                    &grid,
                    &mut visit_flag,
                    length,
                )
            }
        } else {
            //在边上
            *length += 1;
        }

        // 向右
        if col + 1 < col_max {
            if grid[row][col + 1] == 0 {
                // 如果是海
                *length += 1;
            } else if !visit_flag[row][col + 1] {
                // 如果还没访问, 访问
                Self::recursion(
                    row,
                    col + 1,
                    row_max,
                    col_max,
                    &grid,
                    &mut visit_flag,
                    length,
                )
            }
        } else {
            //在边上
            *length += 1;
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
