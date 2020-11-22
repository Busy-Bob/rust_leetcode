struct Solution;
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut stack: Vec<i32> = Vec::new();
        let mut head = head;

        while let Some(this_node) = head {
            stack.push(this_node.val);
            head = this_node.next;
        }

        let (mut p1, mut p2) = (0, stack.len());

        while p1 < p2 {
            if stack[p1] != stack[p2 - 1] {
                return false;
            }
            p1 += 1;
            p2 -= 1;
        }

        true
    }
}
