mod top_k_frequent;
pub use top_k_frequent::Solution;
// use std::rc::Rc;
// use std::cell::RefCell;

fn test1() {
    let nums=vec![1,1,1,2,2,2,3];
    let k=2;
    let ret=Solution::top_k_frequent(nums,k);
    println!("{:?}",ret);

}
fn test2() {
    let nums=vec![];
    let k=1;
    let ret=Solution::top_k_frequent(nums,k);
    println!("{:?}",ret);

}
fn test3() {
    let nums=vec![0,1,1,12,2,5,1,1,1,3,5];
    let k=10;
    let ret=Solution::top_k_frequent(nums,k);
    println!("{:?}",ret);
}
fn main() {
    test1();
    test2();
    test3();
    // println!("Hello, world!");
}
