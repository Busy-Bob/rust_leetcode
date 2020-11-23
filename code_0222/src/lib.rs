/*
    2020年11月24日
    
*/

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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut root = root;
        while let Some(node) = root {
            // 比较左子树的深度和右子树的深度
            let mut left_node = node.borrow().left.clone();
            let mut right_node = node.borrow().right.clone();

            let mut left_level = 0;
            let mut right_level = 0;
            // 找左边节点的深度
            while let Some(curr) = left_node {
                left_level += 1;
                left_node = curr.borrow().left.clone();
            }
            // 找右边节点的深度
            while let Some(curr) = right_node {
                right_level += 1;
                right_node = curr.borrow().left.clone();
            }
            // 两边深度相等，左边满了
            if left_level == right_level {
                res += 2_i32.pow(left_level);
                root = node.borrow().right.clone();
            }
            // 两边深度不等，右边满了
            else {
                res += 2_i32.pow(right_level);
                root = node.borrow().left.clone();
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
