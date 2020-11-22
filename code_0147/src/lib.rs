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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 用一个新链来存, 先要新增一个头节点
        let mut res = ListNode::new(std::i32::MIN);
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            res = Self::insert_sort(res, node);
        }

        res.next
    }

    // 链head已经是一个有序链了，现在增加一个node
    pub fn insert_sort(mut head: ListNode, node_add: Box<ListNode>) -> ListNode {
        let mut curr = head.next.take();
        let mut curr_res = &mut head;
        while let Some(mut curr_node) = curr {
            // 如果比较小，说明还没有到插入位置
            if curr_node.val < node_add.val {
                curr = curr_node.next.take();
                // 往后增加链
                curr_res.next = Some(curr_node);
                curr_res = curr_res.next.as_deref_mut().unwrap();
            }
            // 需要在这儿插入。插入后，break
            else {
                // 待增加变量
                curr_res.next = Some(node_add);
                curr_res = curr_res.next.as_deref_mut().unwrap();
                // 后面的所有加上
                curr_res.next = Some(curr_node);

                return head;
            }
        }

        // 还要考虑【最前端：已经考虑】和最后端插入的情况，在这儿表示待加入节点是最大的。
        curr_res.next = Some(node_add);

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
