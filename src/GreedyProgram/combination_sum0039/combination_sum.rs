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
    pub fn combination_recursion(
        candidates: &Vec<i32>,
        mut index: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        //     if target>candidates[index]{
        //        let rettemp=combination_recursion(candidates,index,target-candidates[index]);
        //    }
        while index >= 0 {
            //弹性 先获取最大的值 然后剩下的数据 再取获取
            let tv = candidates[index as usize];
            if target > tv {
                let rettemp = Solution::combination_recursion(candidates, index, target - tv);
                for item in rettemp.iter() {
                    let mut rett1: Vec<Vec<i32>> = Vec::new();
                    rett1.push(item.to_vec());
                    rett1.push(vec![tv]);
                    let t1 = rett1.concat();
                    ret.push(t1);
                }
            } else if (target == tv) {
                ret.push(vec![tv]);
            }
            // let rettemp=combination_recursion(candidates,index,target-candidates[index]);
            index -= 1;
        }
        ret
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // let mut ret:Vec<Vec<i32>>= Vec::new();
        let mut value: Vec<i32> = candidates.clone();
        value.sort();
        // value.sort_by(|a, b| b.cmp(a));
        // println!("candidates {:?}",value);
        let ret = Solution::combination_recursion(&value, (value.len() - 1) as i32, target);
        ret
    }
}
