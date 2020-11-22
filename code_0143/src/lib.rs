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
// impl Solution {
//     pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
//         let mut num = 0; 
//         let mut temp_head = head.clone();
//         // let mut start = head.clone();
//         let mut vector = Vec::new();
//         while let Some(mut temp) = temp_head {
//             num += 1;
//             temp_head = temp.next;
//             temp.next = None;
//             vector.push(temp);
//         }

        
        
        
//     }
// }
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut headc = head.clone();
        let len = ListNode::len(&headc);
        let (mut half1, mut half2) = (ListNode::new(0), ListNode::new(0));
        let (mut tail, mut cnt) = (&mut half1, 0);
        let mut half3 = None;
        
        // split
        let pivot = len >> 1;
        while let Some(mut x) = headc {
            headc = x.next.take();
            if cnt < pivot {
                tail.next = Some(x);
                tail = tail.next.as_mut().unwrap();
            } else if cnt > pivot {
                x.next = half2.next.take();
                half2.next = Some(x);
            } else if (cnt == pivot) && (len & 0x1 == 1) {
                half3 = Some(x);
            } else {
                x.next = half2.next.take();
                half2.next = Some(x);
            }
            cnt += 1;
        }
        
        // merge
        let mut res = ListNode::new(0);
        tail = &mut res;
        let (mut half1, mut half2) = (half1.next, half2.next);
        while let (Some(mut x), Some(mut y)) = (half1, half2) {
            half1 = x.next.take();
            half2 = y.next.take();
            tail.next = Some(x);
            tail.next.as_mut().unwrap().next = Some(y);
            tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
        }

        tail.next = half3;
        
        *head = res.next;

    }
}

impl ListNode {
    fn len(mut head: &Option<Box<ListNode>>) -> usize {
        let mut l = 0;
        while let Some(x) = head {
            l += 1;
            head = &x.next;
        }
        l
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
