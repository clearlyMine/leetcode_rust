// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 1ms  2.27MB ( 85.03%  99.73%)
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut next = head;
    while let Some(mut node) = next {
        next = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        let mut head = ListNode::new(1);
        head.next = Some(Box::new(ListNode::new(2)));
        let mut new_head = ListNode::new(2);
        new_head.next = Some(Box::new(ListNode::new(1)));
        assert_eq!(reverse_list(Some(Box::new(head))), Some(Box::new(new_head)));
    }
}
