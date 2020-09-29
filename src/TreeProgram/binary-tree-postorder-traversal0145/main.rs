mod binary_tree_level_order;
pub use binary_tree_level_order::Solution;
pub use binary_tree_level_order::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

// 给定一个二叉树，返回它的 后序 遍历。

// 示例:

// 输入: [1,null,2,3]  
//    1
//     \
//      2
//     /
//    3 

// 输出: [3,2,1]
// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？

#[warn(non_snake_case)]
fn test1() {
    let mut  root=TreeNode::new(3);
    let mut  l1=TreeNode::new(9);
    let   l2=TreeNode::new(15);
    let mut r1=TreeNode::new(7);
    let  r2=TreeNode::new(20);
    l1.right=Option::Some(Rc::new(RefCell::new(l2)));
    root.left=Option::Some(Rc::new(RefCell::new(l1)));
   r1.left= Option::Some(Rc::new(RefCell::new(r2)));
   root.right=Option::Some(Rc::new(RefCell::new(r1)));

    let ret=Solution::postorder_traversal(Option::Some(Rc::new(RefCell::new(root))));
    println!("  {:?} ",ret);
    // [9, 15, 3, 20, 7] 
    // 输出: 1
}
fn test2() {
    let mut  root=TreeNode::new(1);
    let mut  r1=TreeNode::new(2);
    let   l1=TreeNode::new(3);
   r1.left= Option::Some(Rc::new(RefCell::new(l1)));
   root.right=Option::Some(Rc::new(RefCell::new(r1)));
    
    // println!("{:?}",root.unwrap().borrow().left);
    let ret=Solution::postorder_traversal(Option::Some(Rc::new(RefCell::new(root))));
    println!("  {:?} ",ret);
    // 132
}
fn main() {
    test1();
    test2();
    // test3();
    // println!("Hello, world!");
}
