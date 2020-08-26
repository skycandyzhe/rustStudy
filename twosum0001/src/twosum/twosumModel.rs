// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut rets = Vec::new();
        for elem in &nums {
            println!(" {}", elem);
        }
        rets.push(10);
        rets.push(5);
        return rets;
    }
}
