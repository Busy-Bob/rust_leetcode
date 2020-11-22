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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_numbers(root, 0) 
    }

    pub fn get_numbers(root: Option<Rc<RefCell<TreeNode>>>, mut previous: i32) -> i32 {
        if let Some(node) = root.clone() {
            previous = previous * 10 + node.borrow().val;
            match (node.borrow().left.is_some(), node.borrow().right.is_some()) {
                (true, true) => {
                    Self::get_numbers(node.borrow().left.clone(), previous)
                        + Self::get_numbers(node.borrow().right.clone(), previous)
                }
                (false, true) => Self::get_numbers(node.borrow().right.clone(), previous),
                (true, false) => Self::get_numbers(node.borrow().left.clone(), previous),
                (false, false) => previous,
            }
        }
        else {
            previous // 该情况不会发生
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
