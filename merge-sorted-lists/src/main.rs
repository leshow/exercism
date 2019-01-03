fn main() {}
#[derive(PartialEq, Eq, Debug)]
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

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match l1 {
        None => match l2 {
            None => None,
            Some(l) => Some(l),
        },
        Some(list1) => match l2 {
            None => Some(list1),
            Some(list2) => {
                let mut list = Box::new(ListNode::new(list1.val));
                std::mem::replace(&mut list.next, Some(Box::new(ListNode::new(list2.val))));

                if let Some(ref mut l) = list.next {
                    std::mem::replace(&mut l.next, merge_two_lists(list1.next, list2.next));
                }

                Some(list)
            }
        },
    }
}
