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
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let mid = nums.len() / 2;
    let mut root = TreeNode::new(nums[mid]);
    root.left = sorted_array_to_bst(Vec::from(&nums[..mid]));
    root.right = sorted_array_to_bst(Vec::from(&nums[(mid + 1)..]));

    Some(Rc::new(RefCell::new(root)))
}

#[cfg(test)]
mod tests {
    use super::sorted_array_to_bst;

    #[test]
    fn basic() {
        let nums = vec![-10, -3, 0, 5, 9];
        let root = sorted_array_to_bst(nums);
        assert_eq!(root.unwrap().borrow().val, 0);
    }
}
