// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

// use std::collections::HashMap;

pub struct Solutions {
    temp: i32,
}

use std::collections::HashMap;
impl Solutions {
    pub fn new() -> Self {
        // That means the user needs to use new to create a Billy. The user can only input times_to_print
        Self { temp: 3 }
    }
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match map.get(v) {
                Some(&index) => return vec![index, i as i32],
                _ => {
                    map.insert(target - v, i as i32);
                }
            }
        }
        vec![]
    }
}
