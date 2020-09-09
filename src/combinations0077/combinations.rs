// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }


// Definition for a binary tree node.

// use std::rc::Rc;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::collections::BinaryHeap;
#[derive(Debug)]
pub struct Solution {}
impl Solution {
    // 1 6 1 
    pub fn combineindex(index:i32,n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ret:Vec<Vec<i32>> = Vec::new();
        if index >n{
            return ret;
        }
        if k==0{
            return ret;
        }
        // 如3到5 取 3个数字 不可能  取2 可能
        if k>(n-index){
            return ret;
        }
        for i in index..n{
            let mut rettemp:Vec<i32> = Vec::new();
            rettemp.push(i);
            // println!("{}",i);
            //从2..n的获取k-1次数字
            if k==1{
                ret.push(rettemp);

            }else{
                let retcom=Solution::combineindex(i+1,n,k-1);
                for vecdata in retcom.iter() {
                    let mut rett1:Vec<Vec<i32>> = Vec::new();
                    rett1.push(vecdata.to_vec());
                    rett1.push(vec![i]);
                    let t1=rett1.concat();
                    // println!("{:?}",t1);
                    ret.push(t1);
                }
            }
            
        }
        ret
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ret:Vec<Vec<i32>> = Vec::new();
        if k==0{
            return ret;
        }
        ret=Solution::combineindex(1,n+1,k);
        ret
    }

    // // 比第k个值大就好了
    // pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //     let mut  ret:Vec<i32>=Vec::new();
    //     let mut map: HashMap<i32, i32> = HashMap::new();
    //     for num in nums.iter() {
    //         // println!("nums{:?}",num);
    //         let data=map.get(&num);
    //         if data.is_none() {
    //             map.insert(num.clone(),1);
    //         }else{
    //             let temp=data.unwrap()+1;
    //             map.insert(num.clone(),temp);
    //         }
    //         // map.
    //         // 不存在插入 num,1
    //         // 存在 数量加一
    //     }
    //     let mut jobs = BinaryHeap::new();
    //     for data in map.iter() {
    //         jobs.push((data.1.clone(),data.0.clone()));
    //     }
    //     for _i in 0..k {
    //         if let Some(data)=jobs.pop(){
    //             ret.push(data.1);
    //         }
            
    //     }
    //     ret
    // }
   
}
