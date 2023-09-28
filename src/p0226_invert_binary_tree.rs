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
use std::collections::VecDeque;
use std::rc::Rc;
//0ms 2MB
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    root.map(|node| {
        node.borrow_mut().swap_all();
        node
    })
}

//https://leetcode.com/problems/invert-binary-tree/solutions/1789032/rust-bfs-dfs-iterative/
#[allow(dead_code)]
type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[allow(dead_code)]
pub fn invert_tree_dfs(root: TreeLink) -> TreeLink {
    let mut stack: Vec<TreeLink> = vec![root.clone()];
    while let Some(ele) = stack.pop() {
        if let Some(n) = ele {
            let TreeNode { left, right, .. } = &mut *n.borrow_mut();
            std::mem::swap(right, left);
            stack.push(right.clone());
            stack.push(left.clone());
        }
    }
    root
}

#[allow(dead_code)]
pub fn invert_tree_bfs(root: TreeLink) -> TreeLink {
    let mut queue: VecDeque<TreeLink> = VecDeque::new();
    queue.push_back(root.clone());
    while let Some(ele) = queue.pop_front() {
        if let Some(n) = ele {
            // method 1.
            // 0ms 2.11MB
            let TreeNode { left, right, .. } = &mut *n.borrow_mut();
            std::mem::swap(right, left);
            queue.push_back(left.clone());
            queue.push_back(right.clone());

            // method 2.
            // 1ms 2.14MB
            let mut borrowed_node = n.borrow_mut();
            let borrowed_node = &mut *borrowed_node;
            std::mem::swap(&mut borrowed_node.right, &mut borrowed_node.left);
            queue.push_back(borrowed_node.left.clone());
            queue.push_back(borrowed_node.right.clone());
        }
    }
    root
}
