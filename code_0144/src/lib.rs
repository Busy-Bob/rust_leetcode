// #[derive(Debug, PartialEq, Eq)]
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
/* 迭代方法*/
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut head = root.clone();
        while head.is_some() || !stack.is_empty() {
            while let Some(root_node) = head {
                result.push(root_node.borrow().val);
                stack.push(root_node.borrow().right.clone()); // 由于Rc的clone实际上是增加引用计数，所以可以直接clone. Option的clone为 Some(x) => Some(x.clone())
                head = root_node.borrow().left.clone();
            }
            head = match stack.pop() {
                Some(a) => a,
                None => break,
            };
        }
        result
    }
}

/* 递归方法

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(root_node) = root {
            result.push(root_node.borrow().val);
            let left = root_node.borrow_mut().left.take();
            result.append(&mut Solution::preorder_traversal(left));
            let right = root_node.borrow_mut().right.take();
            result.append(&mut Solution::preorder_traversal(right));
        }
        result
    }
}

*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
