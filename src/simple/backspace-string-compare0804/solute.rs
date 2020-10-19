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
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut rs=String::new();
        let mut rt=String::new();
        for i in s.chars() {
            if i=='#'{
                rs.pop();
            }else{
                rs.push(i);
            }
        }
        for j in t.chars() {
            if j=='#'{
                rt.pop();
            }else{
                rt.push(j);
            }
        }
        // println!("{:?} {:?}",rs,rt);
        return rs==rt;
    }

}
