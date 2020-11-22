/*
    2020年11月23日
 */
struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 0 {
            return 0;
        }
        // 按照end排序
        let mut points = points;
        points.sort_unstable_by_key(|x| x[1]);
        //  当前点的start如果比now_end大,说明还没刺破，反之，则破了
        let mut now_end = points[0][1];
        let mut res = 1; // 先初始化一个箭
        for point in points {
            if point[0] > now_end {
                now_end = point[1];
                res += 1;
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
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![10,16],vec![2,8],vec![1,6],vec![7,12]]), 2);
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![1,2],vec![3,4],vec![5,6],vec![7,8]]), 4);
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5]]), 2);
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![1,2]]), 1);
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![2,3],vec![2,3]]), 1);
        assert_eq!(Solution::find_min_arrow_shots(vec![vec![-2147483648,2147483647]]), 1);
    }
}
