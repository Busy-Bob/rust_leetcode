// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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
struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut odd_list = ListNode::new(0);
        let mut even_list = ListNode::new(0);
        let mut odd = &mut odd_list;
        let mut even = &mut even_list;
        let mut i = 0;

        while let Some(mut x) = head {
            head = x.next.take();
            match i % 2 == 0 {
                true => {
                    even.next = Some(x);
                    even = even.next.as_deref_mut().unwrap();
                }
                false => {
                    odd.next = Some(x);
                    odd = odd.next.as_deref_mut().unwrap();
                }
            }
            i += 1;
        }

        even.next = odd_list.next.take();
        even_list.next.take()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
