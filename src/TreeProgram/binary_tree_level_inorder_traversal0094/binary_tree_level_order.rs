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

    // pub fn inorder_traversal_deepth(rootnode: &Option<Rc<RefCell<TreeNode>>>,rettemp:& mut Vec<i32>){
    //     if rootnode.is_none(){
    //         return ;
    //     }
    //     let node=& rootnode.as_ref().unwrap().borrow();
        
    //    Solution::inorder_traversal_deepth(&node.left, rettemp);
    //    rettemp.push(node.val);
    //    Solution::inorder_traversal_deepth(&node.right, rettemp);
    // }
    // pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut  ret:Vec<i32> =Vec::new();
    //     Solution::inorder_traversal_deepth(&root, &mut ret);
    //     ret

    // }

    //尝试用迭代实现 左节点 根节点 右节点
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
        let mut  ret:Vec<i32> =Vec::new();
        if root.is_none(){
            return ret;
        }
        let mut index=&root; //

        let  mut temp:VecDeque<Rc<RefCell<TreeNode>>>=VecDeque::new();
        let  mut node:Rc<RefCell<TreeNode>> ;
        // temp.push_back(index.unwrap());
        while temp.is_empty()||!index.is_none(){
            // index=index.left;
            //先将所有左节点加入队列
            while index.is_some(){
                temp.push_back(*(index.as_ref().unwrap()));
                index=&(index.as_ref().unwrap().borrow().left);
            }
            if !temp.is_empty() {
                node=temp.pop_back().unwrap();
                ret.push(node.borrow().val);
                index=&(node.borrow().right);  
            }
        }
        ret

    }
}
