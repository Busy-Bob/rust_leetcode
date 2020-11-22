struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        // let mut a = a;
        // 不需要排序？最大堆建立复杂度O(N),删除复杂度O(log2(N)), 最多也不会超过O(Nlog2(N))
        // a.sort_unstable();

        let mut heap = a.into_iter().collect::<BinaryHeap<_>>();

        let mut a = heap.pop().unwrap();
        let mut b = heap.pop().unwrap();

        while let Some(c) = heap.pop() {
            if a < c + b {
                return a + b + c;
            }
            a = b;
            b = c;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
