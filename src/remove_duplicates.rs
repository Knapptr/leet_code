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
    fn from_vec(vals: Vec<i32>) -> Self {
        let mut dummy_head = ListNode::new(-1);
        let mut current_val = &mut dummy_head;
        for val in vals {
            current_val.next = Some(Box::new(ListNode::new(val)));
            current_val = current_val.next.as_mut().unwrap()
        }
        *dummy_head.next.unwrap()
    }
}

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // create dummy head
    // set dummy head as predec
    let mut dummy = Box::new(ListNode {
        val: -1,
        next: head,
    });
    let mut prec = &mut dummy;
    // handle a case where the head is None
    // current pointer is one in front of prec

    // A 'loop' version
    loop {
        if prec.next.is_none() {
            break;
        }
        let current = prec.next.as_mut().unwrap();
        if current.next.is_some() && current.next.as_ref().unwrap().val == current.val {
            while current.next.is_some() && current.next.as_ref().unwrap().val == current.val {
                *current = current.next.take().unwrap();
            }
            //skip dups
            prec.next = current.next.take();
        } else {
            prec = prec.next.as_mut().unwrap();
        }
    }
    // A nice while let version
    // while let Some(mut current) = prec.next.as_mut() {
    //     // check if current has a next
    //     // check the value of the current.next and see if it is equal to current
    //     if current.next.is_some() && current.next.as_ref().unwrap().val == current.val {
    //         // move current forward until current.next is not equal to current
    //         while current.next.is_some() && current.next.as_ref().unwrap().val == current.val {
    //             current = current.next.as_mut().unwrap();
    //         }
    //         // remove elements in list by setting the preceding to the next of the current (which
    //         // has skipped all dups)
    //         prec.next = current.next.take();
    //     } else {
    //         prec = prec.next.as_mut().unwrap();
    //     }
    // }
    dummy.next
}

#[cfg(test)]
#[test]
fn example_one() {
    let nums = ListNode::from_vec(vec![1, 2, 3, 3, 4, 4, 5]);
    let expected = Box::new(ListNode::from_vec(vec![1, 2, 5]));

    assert_eq!(delete_duplicates(Some(Box::new(nums))).unwrap(), expected);
}
