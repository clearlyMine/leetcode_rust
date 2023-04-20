use std::time::Instant;

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
fn main() {
    let mut l1: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let mut l2: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let mut target: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 8, next: None })),
        })),
    }));
    let mut instant = Instant::now();
    let mut result: Option<Box<ListNode>> = add_two_numbers(l1, l2);
    let mut elapsed = instant.elapsed().as_nanos();
    if result == target {
        println!("{:?} in {:?}", result, elapsed);
    } else {
        eprintln!("{:?} is not expected {:?}", result, target)
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match l1 {
        Some(list1) => match l2 {
            Some(list2) => {
                let mut number1: i32 = 0;
                let mut number2: i32 = 0;
                let mut current: Box<ListNode>  = list1;
                let mut next: Option<Box<ListNode>>;
                let mut i : i32 = 0;
                loop {
                    number1 = number1 + current.val * (10 ^ i);
                    next = current.next;
                    match next{
                        Some(p)=> {current=p;}
                        None => {break}
                    }
                    i=i+1;
                }
                current = list2;
                i = 0;
                loop {
                    number1 = number1 + current.val * (10 ^ i);
                    next = current.next;
                    match next{
                        Some(p)=> {current=p;}
                        None => {break}
                    }
                    i=i+1;
                }
                let result: i32 = number1 + number2;
                let mut result_list: Option<Box<ListNode>>;
                let returnable_list : &Option<Box<ListNode>>;
                let mut x = result;
                let mut last_digit: i32;
                if x>0{
                    last_digit = x % 10;
                    let n = Box::new(ListNode::new(last_digit));
                    result_list = Some(n);
                    // returnable_list = &mut result_list;
                    x = x/10;
                    while x>0{
                        last_digit = x%10;
                        match result_list{
                            Some(next)=>{
                                next.next = Some(Box::new(ListNode::new(last_digit)));
                            }
                            None =>{
                                unreachable!("result_list should never be None")
                            }
                        }
                        let mut node = result_list.unwrap().next;
                        node = Some(Box::new(ListNode::new(last_digit)));
                        result_list = result_list.unwrap().next;
                        x = x/10;
                    }
                }
                
                // while x > 0 {
                //     last_digit = x % 10;
                //     x = x/10;
                //     node.next = Some(Box::new(ListNode::new(last_digit)));
                //     result_list = Some();
                // }
                return *returnable_list;
            }
            None => {
                unreachable!()
            }
        },
        None => {
            unreachable!()
        }
    }
}
