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

   
    pub fn sum_number_depth(parent:i32,root:Option<Rc<RefCell<TreeNode>>>)->i32{
        
        
        if let Some(node)=root{
            if node.borrow().left.is_none() && node.borrow().right.is_none(){
                println!("---{:?}",parent*10+node.borrow().val);
                return parent*10+node.borrow().val
            }
            return Solution::sum_number_depth(parent*10+node.borrow().val,node.borrow().left.clone())+
            Solution::sum_number_depth(parent*10+node.borrow().val,node.borrow().right.clone());
        }
        return 0
    }
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::sum_number_depth(0, root)
    }
}
