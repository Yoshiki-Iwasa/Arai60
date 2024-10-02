// Step4

// https://github.com/Yoshiki-Iwasa/Arai60/pull/66#discussion_r1777943015
// Rc::cloneが散在してるのを修正する

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
        match (root1, root2) {
            (None, None) => None,
            (None, Some(node1)) => Some(node1),
            (Some(node2), None) => Some(node2),
            (Some(node1), Some(node2)) => {
                let new_root = Rc::new(RefCell::new(TreeNode::new(
                    node1.borrow().val + node2.borrow().val,
                )));

                let mut stack = vec![];
                stack.push((node1, node2, Rc::clone(&new_root)));

                while let Some((node1, node2, root)) = stack.pop() {
                    let mut node1_ref_mut = node1.borrow_mut();
                    let mut node2_ref_mut = node2.borrow_mut();
                    let mut root_ref_mut = root.borrow_mut();
                    root_ref_mut.left = match (node1_ref_mut.left.take(), node2_ref_mut.left.take())
                    {
                        (None, None) => None,
                        (None, Some(l2)) => Some(l2),
                        (Some(l1), None) => Some(l1),
                        (Some(l1), Some(l2)) => {
                            let left = Rc::new(RefCell::new(TreeNode::new(
                                l1.borrow().val + l2.borrow().val,
                            )));
                            stack.push((l1, l2, Rc::clone(&left)));
                            Some(left)
                        }
                    };

                    root_ref_mut.right =
                        match (node1_ref_mut.right.take(), node2_ref_mut.right.take()) {
                            (None, None) => None,
                            (None, Some(r2)) => Some(r2),
                            (Some(r1), None) => Some(r1),
                            (Some(r1), Some(r2)) => {
                                let right = Rc::new(RefCell::new(TreeNode::new(
                                    r1.borrow().val + r2.borrow().val,
                                )));
                                stack.push((r1, r2, Rc::clone(&right)));
                                Some(right)
                            }
                        }
                }

                Some(new_root)
            }
        }
    }
}
