use core::hint::unreachable_unchecked;
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

//https://leetcode.com/problems/reorder-list/solutions/1487975/rust-itertaive-in-place-re-ordering-with-o-1-additional-space/
//A prime example of why linked lists are a bad idea in Rust
pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    // Find how many nodes are in the list;
    let mut count = 0;
    let mut list = &*head;

    while let Some(node) = list {
        list = &node.next;
        count += 1;
    }

    // If there are less than two nodes, then there is nothing to do
    if count <= 2 {
        return;
    }

    // Reach the middle of the list in order to split in to two lists
    let mut half = head.as_mut();
    for _ in 0..count / 2 {
        match half {
            // SAFETY: we have counted the number of nodes, so we know there are more nodes
            None => unsafe { unreachable_unchecked() },
            Some(node) => {
                half = node.next.as_mut();
            }
        }
    }

    // Reverse the second half
    let mut half = match half {
        // SAFETY: we have counted the number of nodes, so we know there are more nodes
        None => unsafe { unreachable_unchecked() },
        Some(node) => node.next.take(),
    };

    let mut reversed = ListNode::new(0);
    while let Some(mut node) = half.take() {
        half = node.next.take();
        node.next = reversed.next.take();
        reversed.next = Some(node);
    }

    // merge the two lists, tail points to the node in the first list
    // that has to be disconnected in order to put a node from the reversed
    // list in its place. Then it's reattached as "next" of the new node
    let mut tail = match head.as_mut() {
        // SAFETY: we know there is node at HEAD
        None => unsafe { unreachable_unchecked() },
        Some(node) => &mut node.next,
    };

    while tail.is_some() && reversed.next.is_some() {
        // SAFETY: We know there is a reversed.next because we already checked it
        let mut rev = reversed.next.take().unwrap();
        reversed.next = rev.next.take();

        rev.next = tail.take();
        *tail = Some(rev);
        tail = &mut tail.as_mut().unwrap().next;
        if let Some(node) = tail {
            tail = &mut node.next;
        }
    }
}
