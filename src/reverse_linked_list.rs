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
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head;
    let mut previous: Option<Box<ListNode>> = None;
    let mut next: Option<Box<ListNode>>;

    while let Some(mut node) = current {
        next = node.next;
        node.next = previous;
        previous = Some(node);
        current = next;
    }
    previous
}

#[cfg(test)]
mod tests {
    use super::reverse_list;
    use super::ListNode;

    #[test]
    fn basic() {
        let mut head = ListNode::new(1);
        let mut node_1 = ListNode::new(2);
        let mut node_2 = ListNode::new(3);
        let mut node_3 = ListNode::new(4);
        let node_4 = ListNode::new(5);
        node_3.next = Some(Box::new(node_4));
        node_2.next = Some(Box::new(node_3));
        node_1.next = Some(Box::new(node_2));
        head.next = Some(Box::new(node_1));

        let mut head = reverse_list(Some(Box::new(head)));

        let mut num = 5;
        while let Some(node) = head {
            assert_eq!(node.val, num);
            num -= 1;
            head = node.next;
        }
    }
}
