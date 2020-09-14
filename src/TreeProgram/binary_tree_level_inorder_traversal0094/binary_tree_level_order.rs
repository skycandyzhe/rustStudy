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

    pub fn inorder_traversal_deepth(rootnode: &Option<Rc<RefCell<TreeNode>>>,rettemp:& mut Vec<i32>){
        if rootnode.is_none(){
            return ;
        }
        let node=& rootnode.as_ref().unwrap().borrow();
        
       Solution::inorder_traversal_deepth(&node.left, rettemp);
       rettemp.push(node.val);
       Solution::inorder_traversal_deepth(&node.right, rettemp);
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut  ret:Vec<i32> =Vec::new();
        Solution::inorder_traversal_deepth(&root, &mut ret);
        ret

    }
}
