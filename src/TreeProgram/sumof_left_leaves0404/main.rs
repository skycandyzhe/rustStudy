mod sum_left_leaves;
pub use sum_left_leaves::Solution;
pub use sum_left_leaves::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

fn test1() {
    // let rootnote=TreeNode::new(3);
    // rootnote.left=Some(Rc::new(RefCell::new(TreeNode::new(9))));
    
    
    let l1=Option::Some(Rc::new(RefCell::new(TreeNode::new(9,None,None))));
    let l2=Option::Some(Rc::new(RefCell::new(TreeNode::new(15,None,None))));
    let r2=Option::Some(Rc::new(RefCell::new(TreeNode::new(7,None,None))));
    let r1=Option::Some(Rc::new(RefCell::new(TreeNode::new(20,l2,r2))));
    let root=Option::Some(Rc::new(RefCell::new(TreeNode::new(3,l1,r1))));
    println!("{:?}",root);
    // println!("{:?}",root.unwrap().borrow().left);
    let ret=Solution::sum_of_left_leaves(root);
    println!(" {:#?} {:?} ",ret,ret);
    // 输出: 1
}
fn test2() {
    let root=Option::Some(Rc::new(RefCell::new(TreeNode::new(3,None,None))));
    println!("{:?}",root);
    let ret=Solution::sum_of_left_leaves(root);
    println!(" {:#?} {:?} ",ret,ret);
    // 输出: 1
}
fn test3() {
    // let root=Option::Some(Rc::new(RefCell::new(TreeNode::new(3,None,None))));
    // println!("{:?}",root);
    let ret=Solution::sum_of_left_leaves(None);
    println!(" {:#?} {:?} ",ret,ret);
    // 输出: 1
}
fn main() {
    test1();
    test2();
    test3();
    // println!("Hello, world!");
}
