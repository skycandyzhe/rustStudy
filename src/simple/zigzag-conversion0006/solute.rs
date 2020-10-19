// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

// use std::rc::Rc;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::collections::BinaryHeap;
#[derive(Debug)]
pub struct Solution {}


impl Solution {

    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows<=1{
            return s;
        }
        let mut origin_vec:Vec<Vec<u8>> = vec![vec![]; num_rows as usize];
        let rowcount=num_rows*2-2;
        for (i,data)  in s.char_indices() {
            // println!("{} {:?}",i,data);
            let mut index=(i as i32) %rowcount;
            if index>=num_rows{
                index=rowcount-index ;
            }
            origin_vec[index as usize].push(data as u8);
        }
        let one_vec= origin_vec.concat();
        String::from_utf8(one_vec).unwrap()
    }

}
