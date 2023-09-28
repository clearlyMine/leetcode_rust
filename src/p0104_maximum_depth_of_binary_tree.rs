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
// 0ms  2.58MB (100%  87.78%)
pub fn max_depth_fastest(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    type TreeLink = Option<Rc<RefCell<TreeNode>>>;
    struct State {
        tree: TreeLink,
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
            let mut x = node.borrow_mut();
            let y = &mut *x;
            if y.left.is_some() {
                stack.push(State {
                    tree: y.left.clone(),
                    depth: depth + 1,
                });
            }
            if y.right.is_some() {
                stack.push(State {
                    tree: y.right.clone(),
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
