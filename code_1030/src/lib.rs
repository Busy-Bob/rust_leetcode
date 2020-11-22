struct Solution;
impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut layer = Vec::new();
        // 访问的标志
        let mut flag = vec![vec![false; c as usize]; r as usize];

        layer.push(vec![r0, c0]);
        flag[r0 as usize][c0 as usize] = true;

        while layer.len() > 0 {
            // append是具有改变性的
            res.append(&mut layer.clone());
            // 继续向里面增加
            let mut temp = Vec::new();
            while let Some(coor) = layer.pop() {
                // 需要打标志
                // 上
                if coor[0] - 1 >= 0 && !flag[(coor[0] - 1) as usize][coor[1] as usize] {
                    temp.push(vec![coor[0] - 1, coor[1]]);
                    flag[(coor[0] - 1) as usize][coor[1] as usize] = true;
                }
                // 下
                if coor[0] + 1 < r && !flag[(coor[0] + 1) as usize][coor[1] as usize] {
                    temp.push(vec![coor[0] + 1, coor[1]]);
                    flag[(coor[0] + 1) as usize][coor[1] as usize] = true;
                }
                // 左
                if coor[1] - 1 >= 0 && !flag[coor[0] as usize][(coor[1] - 1) as usize] {
                    temp.push(vec![coor[0], coor[1] - 1]);
                    flag[coor[0] as usize][(coor[1] - 1) as usize] = true;
                }
                // 右
                if coor[1] + 1 < c && !flag[coor[0] as usize][(coor[1] + 1) as usize] {
                    temp.push(vec![coor[0], coor[1] + 1]);
                    flag[coor[0] as usize][(coor[1] + 1) as usize] = true;
                }
            }
            layer = temp;
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
