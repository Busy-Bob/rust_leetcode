// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut curr = root.clone();
        let mut stack = Vec::new();

        while let Some(curr_node) = curr {
            // 将右边的节点放在栈里，将左边的节点放在右边
            if curr_node.borrow().right.is_some() {
                stack.push(curr_node.borrow_mut().right.take());
            }
            curr = curr_node.borrow_mut().left.take();
            // 从栈取出相应的元素
            match (curr.is_some(), stack.len() > 0) {
                (false, true) => curr = stack.pop().unwrap(),
                (false, false) => break,
                (true, _) => (),
            }
            // 将当前决策部分放在右边
            curr_node.borrow_mut().right = curr.clone();
        }
    }
}

// use std::cell::RefCell;
// use std::rc::Rc;
// impl Solution {
//     pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
//         if root.is_none() {
//             return;
//         }
//         let mut v = vec![];
//         let mut stack = vec![];
//         let mut root_clone = root.clone();
//         while stack.len() != 0 || !root_clone.is_none() {
//             while let Some(node) = root_clone {
//                 // 一直添加左子树直到为空
//                 v.push(node.clone());
//                 root_clone = node.borrow_mut().left.take();
//                 stack.push(node);
//             }
//             // 从栈中弹出，取节点的右子树
//             root_clone = stack.pop();
//             root_clone = root_clone.unwrap().borrow_mut().right.take();
//         }
//         for i in 1..v.len() {
//             let mut pre = &v[i - 1];
//             let mut curr = &v[i];
//             pre.as_ref().borrow_mut().left = None;
//             pre.as_ref().borrow_mut().right = Some(curr.clone());
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
