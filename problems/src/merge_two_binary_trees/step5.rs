#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut result: Option<Rc<RefCell<TreeNode>>> = None;
        let mut stack = vec![];
        stack.push((
            root1,
            root2,
            &mut result as *mut Option<Rc<RefCell<TreeNode>>>,
        ));

        while let Some((node1, node2, result_ptr)) = stack.pop() {
            unsafe {
                match (node1, node2) {
                    (None, None) => *result_ptr = None,
                    (None, mut node) | (mut node, None) => *result_ptr = node.take(),
                    (Some(node1), Some(node2)) => {
                        let mut node1_ref = node1.borrow_mut();
                        let mut node2_ref = node2.borrow_mut();
                        let new_node =
                            Rc::new(RefCell::new(TreeNode::new(node1_ref.val + node2_ref.val)));
                        stack.push((
                            node1_ref.left.take(),
                            node2_ref.left.take(),
                            &mut new_node.borrow_mut().left as *mut Option<Rc<RefCell<TreeNode>>>,
                        ));
                        stack.push((
                            node1_ref.right.take(),
                            node2_ref.right.take(),
                            &mut new_node.borrow_mut().right as *mut Option<Rc<RefCell<TreeNode>>>,
                        ));
                        *result_ptr = Some(new_node);
                    }
                }
            }
        }
        result
    }
}
