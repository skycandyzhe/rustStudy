mod solute;
pub use solute::Solution;
// 将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。
// 比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下
 
fn test1() {
    let t1 =String::from("LEETCODEISHIRING");
    let ret=Solution::convert(t1,3);
     println!("{:?}  ",ret );  //3
     assert_eq!(ret,String::from("LCIRETOESIIGEDHN"));
}
fn test2() {
    let t1 =String::from("0123456789");
    let ret=Solution::convert(t1,4);
     println!("{:?}  ",ret );  //3
     
}
fn test3() {
    let t1 =String::from("0123456789");
    let ret=Solution::convert(t1,2);
     println!("{:?}  ",ret );  //3
}
fn test4() {
    let t1 =String::from("0123456789");
    let ret=Solution::convert(t1,1);
     println!("{:?}  ",ret );  //3
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    // test5();
    // println!("Hello, world!");
}
