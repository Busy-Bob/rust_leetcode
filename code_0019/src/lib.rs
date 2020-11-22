
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head_clone = head.clone();
        let mut fast_p = &head;
        let mut slow_p = &head_clone;

        for i in 0..n {
            fast_p = &fast_p.as_ref().unwrap().next;
        }

        while let Some(fast_node) = fast_p.unwrap().next {
            fast_p = &fast_p.as_ref().unwrap().next;
            slow_p = &slow_p.as_ref().unwrap().next;
        }

        slow_p.unwrap().next = slow_p.unwrap().next.unwrap().next.take();

        head

    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
