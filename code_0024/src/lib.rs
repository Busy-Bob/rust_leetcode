


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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        // 结果新建一个链表表示
        let mut result = ListNode::new(0);
        let mut tail = &mut result;
        while let Some(mut node1) = head {
            head = node1.next.take();
            if let Some(mut node2) = head {
                head = node2.next.take();
                node2.next = Some(node1);
                tail.next = Some(node2);
                tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
            } else { // 没有第二个节点, 
                tail.next = Some(node1);
                tail.next.as_mut().unwrap();
            }
        }
        result.next
    }
}









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
