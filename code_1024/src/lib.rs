struct Solution;

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        let mut clips = clips;
        clips.sort_unstable_by(|x, y| y[1].cmp(&x[1]));

        let mut now_end = t;
        let mut index_start: usize = 0;
        let mut result = 0;

        // 最后一个没有考虑到，[[0,2], [1,9]]
        for (i, clip) in clips.iter().enumerate() {
            // 当clip结束时间 小于 当前待填充的结束时间
            if clip[1] < now_end {
                let temp = &clips[index_start..i];
                let choose_clip = temp.iter().max_by_key(|x| now_end - x[0]);
                match choose_clip {
                    Some(choose_clip) => {
                        // 给新的头
                        index_start = i;
                        result += 1;
                        now_end = choose_clip[0];
                        // 已经走完
                        if now_end == 0 {
                            return result;
                        }
                        // 现在所处的位置仍然比选择的位置要小
                        if clip[1] < now_end {
                            return -1;
                        }
                    }
                    None => return -1,
                }
            }
        }
        // 最后一次单独考虑
        let temp = &clips[index_start..];
        let choose_clip = temp.iter().max_by_key(|x| now_end - x[0]);
        match choose_clip {
            Some(choose_clip) => {
                // 给新的头
                now_end = choose_clip[0];
                result += 1;
                // 现在所处的位置仍然比选择的位置要小
                if now_end != 0 {
                    return -1;
                }
            }
            None => return -1,
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
