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
    //递归实现 比较简单
    pub fn postorder_traversal_deepth(rootnode: &Option<Rc<RefCell<TreeNode>>>,rettemp:& mut Vec<i32>){
        if rootnode.is_none(){
            return ;
        }
        let node=& rootnode.as_ref().unwrap().borrow();
        
       Solution::postorder_traversal_deepth(&node.left, rettemp);
       Solution::postorder_traversal_deepth(&node.right, rettemp);
       rettemp.push(node.val);
    }
    pub fn postorder_traversal1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut  ret:Vec<i32> =Vec::new();
        Solution::postorder_traversal_deepth(&root, &mut ret);
        ret

    }

    //实现根右左的逆运算    
    pub fn postorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
        let mut vals = vec![];
        let mut stack = std::collections::VecDeque::new();
        let mut cur_node = root;
        while cur_node.is_some() || !stack.is_empty() {
            while let Some(node) = cur_node.clone() {
                stack.push_back(node.clone());
                let node = node.borrow();
                vals.push(node.val);
                cur_node = node.right.clone();
            }
            cur_node = stack.pop_back().unwrap().borrow().left.clone();
        }
        vals.reverse();
        vals

    }  
        pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
            let mut vals = vec![];
            let mut stack = std::collections::VecDeque::new();
            let mut cur_node = root;
            while cur_node.is_some() || !stack.is_empty() {
                while let Some(node) = cur_node.clone() {
                    stack.push_back(node.clone());
                    let node = node.borrow();
                    vals.push(node.val);
                    cur_node = node.right.clone();
                }
                cur_node = stack.pop_back().unwrap().borrow().left.clone();
            }
            vals.reverse();
            vals
    
        }
}
