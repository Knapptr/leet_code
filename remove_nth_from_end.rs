
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
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let fast = head.clone();

    //iterate fast up by n
    for _ in 0..n{
        match fast{
            Some(f)=>fast=f.next,
            None=> panic!("Less than {n} in list");
        }
    }
}
