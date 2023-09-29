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

use std::cell::RefCell;
use std::rc::Rc;
// 1ms  2.74MB ( 81.23%  22.02%)
pub fn max_depth_fastest(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    struct State {
        tree: Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
    }
    let mut stack: Vec<State> = vec![State {
        tree: root.clone(),
        depth: 1,
    }];
    let mut max_depth = 0;
    while let Some(state) = stack.pop() {
        let n = state.tree;
        let depth = state.depth;
        if let Some(node) = n {
            max_depth = depth.max(max_depth);
            let x = node.borrow();
            if let Some(left) = &x.left {
                stack.push(State {
                    tree: Some(left.clone()),
                    depth: depth + 1,
                });
            }
            if let Some(right) = &x.right {
                stack.push(State {
                    tree: Some(right.clone()),
                    depth: depth + 1,
                });
            }
        }
    }
    max_depth as i32
}

// 1ms  2.69MB ( 82.22%  52.59%)
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut stack = vec![(root.clone(), 1)];
    let mut max_depth = 0;
    while let Some((n, current_depth)) = stack.pop() {
        if let Some(node) = n {
            max_depth = current_depth.max(max_depth);
            let y = node.borrow();
            if y.left.is_some() {
                stack.push((y.left.clone(), current_depth + 1));
            }
            if y.right.is_some() {
                stack.push((y.right.clone(), current_depth + 1));
            }
        }
    }
    max_depth
}
