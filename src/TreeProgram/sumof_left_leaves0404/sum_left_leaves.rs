// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }


// Definition for a binary tree node.

use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right:  Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
  pub fn new(val: i32,left:Option<Rc<RefCell<TreeNode>>>,right:Option<Rc<RefCell<TreeNode>>>) -> Self {
    TreeNode {
      val,
      left: left,
      right: right
    }
  }
}
#[derive(Debug)]
pub struct Solution {}
impl Solution {

    fn sum_of_left_leaves_depth(root:& Option<Rc<RefCell<TreeNode>>>,isleft:bool,ret:&mut i32){
        if let Some(node)=root{
            if  isleft&& node.borrow().left.is_none() && node.borrow().right.is_none(){
                *ret+=node.borrow().val;
                return;
            }
            Solution::sum_of_left_leaves_depth(&(node.borrow().left),true,ret);
            Solution::sum_of_left_leaves_depth(&(node.borrow().right),false,ret);
            return ;
        }else{
            return ;
        }
    }
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) ->i32 { 
        let mut ret=0;
        Solution::sum_of_left_leaves_depth(&root, false,&mut ret);
        ret

    }
}
