mod twosum;
pub use twosum::twosum_model::Solutions;
//变量绑定 let a1 = 5;  类似c++ 常量
//可变绑定 let mut=10 ;

//内置类型
// use std::collections::HashMap;
// // impl Solution {
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut map: HashMap<i32, i32> = HashMap::new();
//     for (i, v) in nums.iter().enumerate() {
//         match map.get(v) {
//             Some(&index) => return vec![index, i as i32],
//             _ => {
//                 map.insert(target - v, i as i32);
//             }
//         }
//     }
//     vec![]
// }
// // }
fn main() {
    // let mut contacts = HashMap<i32,i32>::new();
    let v = vec![2, 7, 11, 15];
    let tartget = 9;
    // let mut a = 1;
    // a = 2;
    // println!("{}", a);
    let sol = Solutions{};
    println!("{:?}",sol);
    let ret = sol.two_sum(v, tartget);
    println!("{:?}", &ret);
    // let sol = Solution::new();
    // let ret = sol.two_sum(&v, 100);

    // println!("{:?}", v);
    // println!("{:?}", ret);
    // for elem in &v {
    //     println!(" {}", elem);
    // }
    // Solution
    println!("Hello, world!");
}
