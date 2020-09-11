// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

// 给定一个数组 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
// candidates 中的每个数字在每个组合中只能使用一次。
// 说明：
// 所有数字（包括目标数）都是正整数。
// 解集不能包含重复的组合。

//与39类似 但是注意只能用一次 且不能包含重复的组合


// use std::rc::Rc;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::collections::BinaryHeap;
#[derive(Debug)]
pub struct Solution {}
impl Solution {
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
                let rettemp = Solution::combination_recursion(candidates, index-1, target - tv);
                for item in rettemp.iter() {
                    let mut rett1: Vec<Vec<i32>> = Vec::new();
                    rett1.push(item.to_vec());
                    rett1.push(vec![tv]);
                    let t1 = rett1.concat();
                    ret.push(t1);
                }
            } else if target == tv {
                ret.push(vec![tv]);
            }
            // let rettemp=combination_recursion(candidates,index,target-candidates[index]);
            index -= 1;
        }
        ret
    }
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // let mut ret:Vec<Vec<i32>>= Vec::new();
        let mut value: Vec<i32> = candidates.clone();
        value.sort();
        // value.sort_by(|a, b| b.cmp(a));
        // println!("candidates {:?}",value);
        let mut ret = Solution::combination_recursion(&value, (value.len() - 1) as i32, target);
        ret.sort();
        ret.dedup();
        ret
    }
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let conditions=vec![1,2,3,4,5,6,7,8,9];
        let mut  Ret=Solution::combination_sum2(conditions,n);
        let mut counts=Ret.len();
        while counts>0{
            counts-=1;
            if Ret[counts].len() !=(k as usize) {
                Ret.remove(counts);
            }
        }
        Ret
    }
}
