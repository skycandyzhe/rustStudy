// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }


// Definition for a binary tree node.

// use std::rc::Rc;
// use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::BinaryHeap;
#[derive(Debug)]
pub struct Solution {}
impl Solution {
    // 比第k个值大就好了
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut  ret:Vec<i32>=Vec::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in nums.iter() {
            // println!("nums{:?}",num);
            let data=map.get(&num);
            if data.is_none() {
                map.insert(num.clone(),1);
            }else{
                let temp=data.unwrap()+1;
                map.insert(num.clone(),temp);
            }
            // map.
            // 不存在插入 num,1
            // 存在 数量加一
        }
        let mut jobs = BinaryHeap::new();
        for data in map.iter() {
            jobs.push((data.1.clone(),data.0.clone()));
        }
        for _i in 0..k {
            if let Some(data)=jobs.pop(){
                ret.push(data.1);
            }
            
        }
        ret
    }
   
}
