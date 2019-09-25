use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(node) = root {
        let node = node.borrow();
        return is_symmetric_helper(node.left.clone(), node.right.clone());
    }

    true
}

fn is_symmetric_helper(
    t1: Option<Rc<RefCell<TreeNode>>>,
    t2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (t1, t2) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(node1), Some(node2)) => {
            let node1 = node1.borrow();
            let node2 = node2.borrow();
            node1.val == node2.val
                && is_symmetric_helper(node1.left.clone(), node2.right.clone())
                && is_symmetric_helper(node1.right.clone(), node2.left.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_symmetric;
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn basic_1() {
        let mut left = TreeNode::new(2);
        left.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        left.right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let mut right = TreeNode::new(2);
        right.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        right.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(is_symmetric(Some(Rc::new(RefCell::new(root)))), true);
    }

    #[test]
    fn basic_2() {
        let mut left = TreeNode::new(2);
        left.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let mut right = TreeNode::new(2);
        right.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(is_symmetric(Some(Rc::new(RefCell::new(root)))), false);
    }
}
