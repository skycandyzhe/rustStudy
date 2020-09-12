mod combinations;
pub use combinations::Solution;
// use std::rc::Rc;
// use std::cell::RefCell;

fn test1() {
    let nums=4;
    let k=2;
    let ret=Solution::combine(nums,k);
    println!("{:?}",ret);

}
fn test2() {
    let nums=5;
    let k=1;
    let ret=Solution::combine(nums,k);
    println!("{:?}",ret);

}
fn test3() {
    let nums=6;
    let k=3;
    let ret=Solution::combine(nums,k);
    println!("{:?} len:{}",ret,ret.len());
}
fn test4() {
    let nums=6;
    let k=7;
    let ret=Solution::combine(nums,k);
    println!("{:?} len:{}",ret,ret.len());
}
fn test5() {
    let nums=6;
    let k=5;
    let ret=Solution::combine(nums,k);
    println!("{:?} len:{}",ret,ret.len());
}
fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    // println!("Hello, world!");
}
