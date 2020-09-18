mod permute;
pub use permute::Solution;
// use std::rc::Rc;
// use std::cell::RefCell;

// 给定一个可包含重复数字的序列，返回所有不重复的全排列。

// 示例:

// 输入: [1,1,2]
// 输出:
// [
//   [1,1,2],
//   [1,2,1],
//   [2,1,1]
// ]

 
fn test1() {
    let candidates =vec![10,1,2,7,1,5];
    let ret=Solution::permute_unique(candidates);
    println!("{:?} {:?}",ret,ret.len());
}
fn test2() {
    let candidates =vec![1,1,2];

    let ret=Solution::permute_unique(candidates);
    println!("{:?} {:?}",ret,ret.len());
}
fn test3() {
    let candidates =vec![1,3,2];
 let ret=Solution::permute_unique(candidates);
 println!("{:?} {:?}",ret,ret.len());
}
fn test4() {
    let candidates =vec![1,2,3,6,7];
    let ret=Solution::permute_unique(candidates);
    println!("{:?} {:?}",ret,ret.len());
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    // test5();
    // println!("Hello, world!");
}
