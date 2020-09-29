// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }


// Definition for a binary tree node.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right:  Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
#[derive(Debug)]
pub struct Solution {}
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let temp=root.clone();
        if let Some(node)=temp{
            if val==node.borrow().val{
                return root.clone();
            }else if val <node.borrow().val{
                return Solution::search_bst(node.borrow().left.clone(),val);
            }else{
                 return Solution::search_bst(node.borrow().right.clone(),val);
            }
        }else{
            return None;
        }
     
    }
}
