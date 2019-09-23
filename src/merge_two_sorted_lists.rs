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

#[allow(dead_code)]
pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                node1.next = merge_two_lists(node1.next, Some(node2));
                Some(node1)
            } else {
                node2.next = merge_two_lists(node2.next, Some(node1));
                Some(node2)
            }
        }
        (Some(node), None) => Some(node),
        (None, Some(node)) => Some(node),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::merge_two_lists;
    use super::ListNode;

    #[test]
    fn basic() {
        // Create list one
        let mut l1 = ListNode::new(1);
        let mut l1_next = ListNode::new(2);
        l1_next.next = Some(Box::new(ListNode::new(4)));
        l1.next = Some(Box::new(l1_next));

        // Create list two
        let mut l2 = ListNode::new(1);
        let mut l2_next = ListNode::new(3);
        l2_next.next = Some(Box::new(ListNode::new(4)));
        l2.next = Some(Box::new(l2_next));

        let mut merged = merge_two_lists(Some(Box::new(l1)), Some(Box::new(l2)));
        let sol = vec![1, 1, 2, 3, 4, 4];

        for i in 0..6 {
            let node = merged.unwrap();
            assert_eq!(node.val, sol[i]);
            merged = node.next;
        }
    }
}
