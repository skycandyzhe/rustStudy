mod solution;
pub use solution::Solution;
pub use solution::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

// 「好节点」X 定义为：从根到该节点 X 所经过的节点中，没有任何节点的值大于 X 的值。

//  

// 示例 1：



// 输入：root = [3,1,4,3,null,1,5]
// 输出：4
// 解释：图中蓝色节点为好节点。
// 根节点 (3) 永远是个好节点。
// 节点 4 -> (3,4) 是路径中的最大值。
// 节点 5 -> (3,4,5) 是路径中的最大值。
// 节点 3 -> (3,1,3) 是路径中的最大值。



//     3
//    /\
//    9 7
//    /\ /\
//     1515
#[warn(non_snake_case)]
fn test1() {
    let mut  root=TreeNode::new(3);
    let mut  l1=TreeNode::new(9);
    let   l2=TreeNode::new(15);
    let mut r1=TreeNode::new(2);
    let  r2=TreeNode::new(20);
    l1.right=Option::Some(Rc::new(RefCell::new(l2)));
    root.left=Option::Some(Rc::new(RefCell::new(l1)));
   r1.left= Option::Some(Rc::new(RefCell::new(r2)));
   root.right=Option::Some(Rc::new(RefCell::new(r1)));

    let ret=Solution::good_nodes(Option::Some(Rc::new(RefCell::new(root))));
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
    let ret=Solution::good_nodes(Option::Some(Rc::new(RefCell::new(root))));
    println!("  {:?} ",ret);
    // 132
}
fn test3() {
    let mut  root=TreeNode::new(3);
    let mut  l1=TreeNode::new(1);
    let   l2=TreeNode::new(3);
    let mut r1=TreeNode::new(4);
    let  r2=TreeNode::new(1);
    let  r3=TreeNode::new(5);
    l1.right=Option::Some(Rc::new(RefCell::new(l2)));
    root.left=Option::Some(Rc::new(RefCell::new(l1)));
   r1.left= Option::Some(Rc::new(RefCell::new(r2)));
   r1.right= Option::Some(Rc::new(RefCell::new(r3)));
   root.right=Option::Some(Rc::new(RefCell::new(r1)));

    let ret=Solution::good_nodes(Option::Some(Rc::new(RefCell::new(root))));
    println!("  {:?} ",ret);
    // [9, 15, 3, 20, 7] 
    // 输出: 1
}
fn main() {
    test1();
    test2();
    test3();
    // println!("Hello, world!");
}
