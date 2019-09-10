use std::cell::RefCell;
use std::cmp::max;
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
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                1
            } else if node.left.is_none() {
                1 + max_depth(node.right.clone())
            } else if node.right.is_none() {
                1 + max_depth(node.left.clone())
            } else {
                1 + max(max_depth(node.left.clone()), max_depth(node.right.clone()))
            }
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::max_depth;
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn basic() {
        let low_left = TreeNode::new(15);
        let low_right = TreeNode::new(7);
        let mut middle_right = TreeNode::new(20);
        middle_right.left = Some(Rc::new(RefCell::new(low_left)));
        middle_right.right = Some(Rc::new(RefCell::new(low_right)));
        let middle_left = TreeNode::new(9);
        let mut root = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(middle_left)));
        root.right = Some(Rc::new(RefCell::new(middle_right)));

        assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), 3)
    }
}
