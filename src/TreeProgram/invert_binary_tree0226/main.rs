mod binary_tree_level_order;
pub use binary_tree_level_order::Solution;
pub use binary_tree_level_order::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

// 翻转一棵二叉树。

// 示例：

// 输入：

//      4
//    /   \
//   2     7
//  / \   / \
// 1   3 6   9
// 输出：

//      4
//    /   \
//   7     2
//  / \   / \
// 9   6 3   1
// //题目很简单 但要考虑rust可变问题
// //递归 看非空 交换节点即可
 
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

    let ret=Solution::invert_tree(Option::Some(Rc::new(RefCell::new(root))));
    println!(" {:?} ",ret);
    // [9, 15, 3, 20, 7] 
    // 输出: 1
}
fn test2() {
    1
        2
     3   


    let mut  root=TreeNode::new(1);
    let mut  r1=TreeNode::new(2);
    let   l1=TreeNode::new(3);
   r1.left= Option::Some(Rc::new(RefCell::new(l1)));
   root.right=Option::Some(Rc::new(RefCell::new(r1)));

    // println!("{:?}",root.unwrap().borrow().left);
    let ret=Solution::invert_tree(Option::Some(Rc::new(RefCell::new(root))));
    println!("   {:?} ",ret);
    // 132
}
fn main() {
    test1();
    test2();
    // test3();
    // println!("Hello, world!");
}
