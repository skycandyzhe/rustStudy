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

    pub fn Breadthforeach(lastlevel:Vec<Rc<RefCell<TreeNode>>>,rettemp:& mut Vec<f64>){
        let mut vecdata:f64=0.0;
        let mut levelcount=0;
        if lastlevel.len()==0{
            return
        }
        let mut  levelnode: Vec<Rc<RefCell<TreeNode>>>=Vec::new();
        for node in lastlevel.iter(){
            // root.unwrap().borrow().left
            let _node=node.borrow();
            // let right=node.unwrap().borrow();
            if _node.left.is_none()==false{
                levelnode.push(_node.left.as_ref().unwrap().clone());
            }
            if _node.right.is_none()==false{
                levelnode.push(_node.right.as_ref().unwrap().clone());
            }     
            vecdata+=_node.val as f64;
            levelcount+=1;
            // 获取到每个节点的左右节点 是none抛弃 不是 加入节点表和下一层表

        }
        vecdata=vecdata/(levelcount as f64);
        rettemp.push(vecdata);
        Solution::Breadthforeach(levelnode,rettemp);


    }
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut  ret:Vec<f64> =Vec::new();
        if root.is_none(){
            return ret;
        }
        // println!("{:?}",root.unwrap().borrow());
        
        let vlevel=vec![root.unwrap().clone()];
        Solution::Breadthforeach(vlevel,&mut ret);
        ret

    }
}
