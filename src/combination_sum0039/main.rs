mod combinations;
pub use combinations::Solution;
// use std::rc::Rc;
// use std::cell::RefCell;

// 1 <= candidates.length <= 30
// 1 <= candidates[i] <= 200
// candidate 中的每个元素都是独一无二的。
// 1 <= target <= 500
// 最大只有500 用贪心算法 

// 给定一个无重复元素的数组 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。

// candidates 中的数字可以无限制重复被选取。

// 说明：

// 所有数字（包括 target）都是正整数。
// 解集不能包含重复的组合。 
fn test1() {
    let candidates =vec![2,3,6,7];
    let k=7;
    let ret=Solution::combination_sum(nums,k);
    println!("{:?}",ret);

}
// fn test2() {
//     let nums=5;
//     let k=1;
//     let ret=Solution::combine(nums,k);
//     println!("{:?}",ret);

// }
// fn test3() {
//     let nums=6;
//     let k=3;
//     let ret=Solution::combine(nums,k);
//     println!("{:?} len:{}",ret,ret.len());
// }
// fn test4() {
//     let nums=6;
//     let k=7;
//     let ret=Solution::combine(nums,k);
//     println!("{:?} len:{}",ret,ret.len());
// }
// fn test5() {
//     let nums=6;
//     let k=5;
//     let ret=Solution::combine(nums,k);
//     println!("{:?} len:{}",ret,ret.len());
// }
fn main() {
    test1();
    // test2();
    // test3();
    // test4();
    // test5();
    // println!("Hello, world!");
}
