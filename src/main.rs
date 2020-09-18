// mod twosum;
// pub use twosum::twosum_model::Solutions;
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

    let  mut strtemp=String::from("Hello world");
    // println!("{:?}",strtemp);
    // let strstr=strtemp.as_mut_str();

    if true{
        strtemp=String::from("SDASDASDsa");
    }
    
    let strvec=unsafe{strtemp.as_mut_vec()};
    println!("{:?} {:?}",strvec,strvec);
    // for i in (0..5).reverse(){
    //     println!("{:?}",i);
    // }
    let path=env!("PATH");
    println!("{:?}",path);
    let path=env!("LANG");
    println!("{:?}",path);
    println!("Hello, world!");
}
