
struct Solution;

use std::cmp::max;

impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        if a.len() < 3 {
            return 0;
        }
        let mut last_num: i32 = a[0];
        let mut increase_flag: bool = false;
        let mut decrease_flag: bool = false;
        let mut mountain_start: usize = 0;
        let mut max_length: u32 = 0;
        //
        for (index, &now_num) in a.iter().enumerate() {
            // 上升
            if now_num > last_num {
                match (increase_flag, decrease_flag) {
                    (_, true) => {
                        //开始是下降，后面变成了上升,
                        max_length = max(max_length, (index - mountain_start) as u32);
                        mountain_start = index - 1;
                        decrease_flag = false;
                        increase_flag = true;
                    }
                    (false, false) => {
                        mountain_start = index - 1;
                        increase_flag = true;
                    }
                    _ => {}
                }
            }
            // 下降
            else if now_num < last_num {
                if increase_flag {
                    decrease_flag = true;
                    increase_flag = false;
                } 
            }
            // 平
            else {
                if decrease_flag {
                    max_length = max(max_length, (index - mountain_start) as u32);
                    decrease_flag = false;
                } else if increase_flag {
                    increase_flag = false;
                }
            }
            //
            last_num = now_num;
        }
        //
        if decrease_flag {
            max_length = max(max_length, (a.len() - mountain_start) as u32)
        }

        max_length as i32
    }
}

