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
        let mut ret:String=String::new();
        let mut rowVec:Vec<String> =Vec::new();
        for i in 0..num_rows {
            let temp:String= String::new();
            rowVec.push(temp);
        }
        let rowcount=num_rows*2-2;
        for (i,data)  in s.char_indices() {
            // println!("{} {:?}",i,data);
            let mut index=(i as i32) %rowcount;
            if index>=num_rows{
                index=rowcount-index ;
            }
            rowVec[index as usize].push(data)
        }
        for i in rowVec{
            ret.push_str(&i);
        }
        ret
    }
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
