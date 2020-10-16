// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

// 同一个元素不能选择多次
// 121
// 第一次 1  2   1
// 第一次    1   2 
// 第二次 2  1   1
// 第三次 1  2   1
// 第三次    1   2 
// 123
// 第一次 1  2   3
// 第一次    3   2 

// 第二次 2  1   3
// 第二次 2  3   1

// 第三次 3  2   1
// 第三次    1   2 

// use std::rc::Rc;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::collections::BinaryHeap;
#[derive(Debug)]
pub struct Solution {}
impl Solution {
    // pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
    //     let mut ret:Vec<i32>=Vec::new();
    //     for i in a{
    //         ret.push(i*i);
    //     }
    //     ret.sort();
    //     ret
    // }

    pub fn combination_recursion(candidates: Vec<i32> ) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let veclen=candidates.len();
        if veclen==1{
            ret.push(candidates);
            return ret;
        }
        for i in 0..veclen {
            //加入剪枝 同一级 同一个不用加入多次
            if i>0 &&  (candidates[i]==candidates[i-1]){
                continue;
            }
            let tv = candidates[(i) as usize];
            let mut temp=candidates.clone();
            temp.remove(i);
            let rettemp = Solution::combination_recursion(temp );
            for item in rettemp.iter() {
                let mut rett1: Vec<Vec<i32>> = Vec::new();
                rett1.push(item.to_vec());
                rett1.push(vec![tv]);
                let t1 = rett1.concat();
                ret.push(t1);
            }
        }
        ret
    }
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut temp=nums.clone();
        temp.sort();
        //还可以考虑将clone 去掉 改成vis 看元素是否被访问过 但会降低可读性 不想写了
        // let vis:Vec<bool>= Vec::new();
        // vis.resize(nums.len(),false);
        let mut ret = Solution::combination_recursion(temp);
        ret
    }

   
}
