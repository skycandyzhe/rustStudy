// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

// use std::rc::Rc;
// use std::cell::RefCell;
// use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::BinaryHeap;
#[derive(Debug)]
pub struct Solution {}


impl Solution {

    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // let mut map:HashSet<i32>=HashSet::new();
        // for  i in nums1{
        //     map.insert(i);
        // }
        // let mut ret:Vec<i32>=Vec::new();
        // for i in nums2{
        //     if map.get(&i).is_some(){
        //         ret.push(i);
        //         map.remove(&i);
        //     }
        // }
        // ret

        use std::collections::HashSet;
        let mut set1: HashSet<i32> = nums1.into_iter().collect();
        let mut set2: HashSet<i32> = nums2.into_iter().collect();
        // println!("{:?}",set1.intersection(&set2));
        return set1.intersection(&set2).cloned().into_iter().collect::<Vec<i32>>();
    }

}
