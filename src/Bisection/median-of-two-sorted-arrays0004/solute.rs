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
    
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let  l1_num=nums1.len();
        let  l2_num=nums2.len();
        let mut ret:f64=0.0;
        if l1_num+l2_num==0{
            return 0.0
        }
        let mut flag=false;
        if (l1_num+l2_num)%2==0{// 偶数表示需要二个
            flag=true;
        }
        let mut  needindex=(l1_num+l2_num+1)/2;   //4-> 2 3   5->2
        let mut l1index=0;
        let mut l2index=0;
        // let index=0;
        loop{
            // println!("needindex {:?}  {:?} {:?}",needindex,l1index,l2index);
            needindex-=1;
            if needindex==0{
                if flag==true{
                    if l1index< l1_num && l2index< l2_num{
                        if nums1[l1index] >nums2[l2index]{
                            ret+= nums2[l2index] as f64;
                            l2index+=1;
                        }else{
                            ret+=  nums1[l1index] as f64;
                            l1index+=1;
                        }
                    }else if l1index<l1_num {
                        ret+=  nums1[l1index] as f64;
                        l1index+=1;
                    }else if l2index<l2_num{
                        ret+=  nums2[l2index] as f64;
                        l2index+=1;
                    }
                    if l1index< l1_num && l2index< l2_num{
                        if nums1[l1index] >nums2[l2index]{
                            ret+= nums2[l2index] as f64;
                            l2index+=1;
                        }else{                       
                            ret+=  nums1[l1index] as f64;
                            l1index+=1;
                        }
                    }else if l1index<l1_num {                       
                        ret+=  nums1[l1index] as f64;
                        l1index+=1;
                    }else if l2index<l2_num{
                        ret+=  nums2[l2index] as f64;
                        l2index+=1;
                    }    
                    return ret/2.0;
                }else{
                    if l1index< l1_num && l2index< l2_num{
                        if nums1[l1index] >nums2[l2index]{
                            ret+= nums2[l2index] as f64 ;
    
                        }else{
                            ret+= nums1[l1index] as f64;
    
                        }
                    }else if l1index<l1_num {
                        ret+= nums1[l1index] as f64;

                    }else if l2index<l2_num{
                        ret+= nums2[l2index] as f64;

                    }
                    return ret;
                }
            }
            if l1index< l1_num && l2index< l2_num{
                if nums1[l1index] >nums2[l2index]{
                    l2index+=1;
                }else{
                    l1index+=1;
                }
            } else if l1index<l1_num {
                l1index+=1;
            }else if l2index<l2_num{
                l2index+=1;
            }
        }

    }

}
