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
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    match head.clone() {
        Some(node) => {
            if node.next.is_none() {
                return true;
            }
        }
        None => return true,
    }

    let mut previous: Option<Box<ListNode>> = None;
    let mut slow_runner = head.clone();
    let mut fast_runner = head.clone();

    loop {
        match fast_runner.clone() {
            None => break,
            Some(fast_node) => {
                if fast_node.next.is_none() {
                    break;
                }
                fast_runner = fast_node.next.unwrap().next;
                let mut slow_node = slow_runner.unwrap();
                let next = slow_node.next.unwrap();
                slow_node.next = previous;
                previous = Some(slow_node);
                slow_runner = Some(next);
            }
        }
    }

    if !fast_runner.is_none() {
        slow_runner = slow_runner.unwrap().next;
    }

    while let Some(previous_node) = previous.clone() {
        let slow_node = slow_runner.unwrap();
        if previous_node.val != slow_node.val {
            return false;
        }
        previous = previous_node.next;
        slow_runner = slow_node.next;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;
    use super::ListNode;

    #[test]
    fn basic_1() {
        let mut root = ListNode::new(1);
        root.next = Some(Box::new(ListNode::new(2)));
        assert_eq!(is_palindrome(Some(Box::new(root))), false);
    }

    #[test]
    fn basic_2() {
        let last = Some(Box::new(ListNode::new(1)));
        let mut second_to_last = ListNode::new(2);
        second_to_last.next = last;
        let mut second = ListNode::new(2);
        second.next = Some(Box::new(second_to_last));
        let mut root = ListNode::new(1);
        root.next = Some(Box::new(second));
        assert_eq!(is_palindrome(Some(Box::new(root))), true);
    }
}
