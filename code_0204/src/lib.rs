use std::vec;

struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }

        let mut flag = vec![true; n as usize];
        flag[0] = false;
        flag[1] = false;

        (2..n as usize).for_each(|x| {
            // 是质数再筛
            if flag[x] {
                // 比他小的都已经乘过了，所以直接从x开始
                for y in x..n as usize {
                    if y * x < n as usize {
                        flag[y * x] = false;
                    } else {
                        break;
                    }
                }
            }
        });

        flag.into_iter().filter(|&x| x).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::count_primes(10), 4);
    }
}
