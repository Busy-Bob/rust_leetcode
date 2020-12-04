/*
    2020-12-4
*/

struct Solution;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        let mut last_ele = std::i32::MAX;
        // 现在长度为1的个数
        let mut dp1 = 0;
        // 现在长度为2的个数
        let mut dp2 = 0;
        // 长度为3以上的个数
        let mut dp3 = 0;

        let mut pos = 0;
        let mut cur = 1;
        for i in 0..(nums.len() - 1) {
            if nums[i] == nums[i + 1] {
                cur += 1;
            }
            // 开始做运算
            else {
                // 如果是和上一个接着的
                if nums[i] - 1 == last_ele {
                    if cur < (dp1 + dp2) {
                        return false;
                    } else {
                        if cur > dp1 + dp2 + dp3 {
                            let temp = cur - (dp1 + dp2 + dp3);
                            dp3 += dp2;
                            dp2 = dp1;
                            dp1 = temp
                        } else {
                            dp3 = cur - (dp1 + dp2);
                            dp3 += dp2;
                            dp2 = dp1;
                            dp1 = 0;
                        }
                    }
                }
                // 如果没有接着
                else {
                    if dp1 + dp2 > 0 {
                        return false;
                    } else {
                        dp1 = cur;
                        dp2 = 0;
                        dp3 = 0;
                    }
                }
                last_ele = nums[i];
                cur = 1;
            }
        }

        let &now = nums.last().unwrap();
        if last_ele != now {
                // 如果是和上一个接着的
                if now - 1 == last_ele {
                    if cur < (dp1 + dp2) {
                        return false;
                    } else {
                        if cur > dp1 + dp2 + dp3 {
                            let temp = cur - (dp1 + dp2 + dp3);
                            dp3 += dp2;
                            dp2 = dp1;
                            dp1 = temp
                        } else {
                            dp3 = cur - (dp1 + dp2);
                            dp3 += dp2;
                            dp2 = dp1;
                            dp1 = 0;
                        }
                    }
                }
                // 如果没有接着
                else {
                    if dp1 + dp2 > 0 {
                        return false;
                    } else {
                        dp1 = cur;
                        dp2 = 0;
                        dp3 = 0;
                    }
                }
        }

        // 判断是否结束
        dp1 + dp2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::is_possible(vec![1,2,3,3,4,5]), true);
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::is_possible(vec![1,2,3,3,4,4,5,5]), true);
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::is_possible(vec![1,2,3,4,4,5]), false);
    }
}
