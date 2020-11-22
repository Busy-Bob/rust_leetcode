struct Solution;
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        let mut new_interval = new_interval;
        if intervals.len() == 0 {
            intervals.push(new_interval);
            return intervals;
        }

        let mut i = 0;
        loop {
            // 在前面不相交
            if intervals[i][0] > new_interval[1] {
                intervals.insert(i, new_interval);
                break;
            }
            // 在后面不相交
            else if intervals[i][1] < new_interval[0] {
                if i == intervals.len() - 1 {
                    intervals.push(new_interval);
                    break;
                }
                i += 1;
            }
            // 新增在前面或者里面，相交
            else if intervals[i][1] >= new_interval[1] {
                new_interval[0] = new_interval[0].min(intervals[i][0]);
                new_interval[1] = new_interval[1].max(intervals[i][1]);
                intervals[i] = new_interval;
                break;
            }
            // 新增的后部分在外面，相交
            else {
                let begin = i;
                let mut end = i;
                for j in ((i + 1)..intervals.len()).into_iter() {
                    // 新增在前面
                    if intervals[j][0] > new_interval[1] {
                        end = j - 1;
                        break;
                    }
                    // 新增已经被包括完了
                    else if intervals[j][1] >= new_interval[1] {
                        end = j;
                        break;
                    }
                    // 新增还没有被包括完毕，继续下一次
                }
                new_interval[0] = new_interval[0].min(intervals[begin][0]);
                new_interval[1] = new_interval[1].max(intervals[end][1]);

                intervals.drain(begin..=end);
                intervals.insert(begin, new_interval);
                break;
            }
        }

        intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::insert(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]], vec![4, 8]), vec![vec![1,2],vec![3,10],vec![12,16]]);
    }
}
