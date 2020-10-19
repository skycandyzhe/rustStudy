mod solute;
pub use solute::Solution;
// 4. 寻找两个正序数组的中位数
// 给定两个大小为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的中位数。

// 进阶：你能设计一个时间复杂度为 O(log (m+n)) 的算法解决此问题吗？
 
fn test1() {
    let t1 =String::from("ab#c");
    let t2=String::from("ac");
    let ret=Solution::backspace_compare(t1,t2);
     println!("{:?}  ",ret );  //3
}
fn test2() {
    let t1 =String::from("ab333######c");
    let t2=String::from("ac");
    let ret=Solution::backspace_compare(t1,t2);
     println!("{:?}  ",ret );  //7.0
}
fn test3() {
    let t1 =String::from("a##c");
    let t2=String::from("#a#c");
    let ret=Solution::backspace_compare(t1,t2);
     println!("{:?}  ",ret );  //7.0
}
fn test4() {
    let t1 =String::from("ab#c");
    let t2=String::from("");
    let ret=Solution::backspace_compare(t1,t2);
     println!("{:?}  ",ret );  //5.5
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    // test5();
    // println!("Hello, world!");
}
