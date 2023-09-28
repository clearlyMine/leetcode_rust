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

//https://leetcode.com/problems/invert-binary-tree/solutions/1565092/Rust-iterative-and-recursive-std:mem::swap-solutions/
trait TreeNodeSwap {
    fn swap(&mut self);
    fn swap_all(&mut self);
}

impl TreeNodeSwap for TreeNode {
    fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right)
    }
    fn swap_all(&mut self) {
        self.left.as_mut().map(|node| node.borrow_mut().swap_all());
        self.right.as_mut().map(|node| node.borrow_mut().swap_all());
        self.swap();
    }
}

use std::cell::RefCell;
use std::rc::Rc;
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    root.map(|node| {
        node.borrow_mut().swap_all();
        node
    })
}
