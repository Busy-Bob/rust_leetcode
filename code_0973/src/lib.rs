struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        for point in points {
            let distance =  point[0].pow(2) + point[1].pow(2);
            if heap.len() < k as usize  {
                heap.push((distance, point));
            } else if distance < heap.peek().cloned().unwrap().0 {
                heap.pop();
                heap.push((distance, point));
            } 
        }
        heap.iter().map(|(_, x)| x.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
