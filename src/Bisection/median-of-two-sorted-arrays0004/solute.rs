// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

// use std::rc::Rc;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::collections::BinaryHeap;
#[derive(Debug)]
pub struct Solution {}

// 获取到二个序列的第k个数据
fn get_k_th(nums1: &Vec<i32>, nums2: &Vec<i32>, s1: usize, s2: usize, k: usize) -> f64 {
   
    let len1 = nums1.len() - s1;
    let len2 = nums2.len() - s2;
    println!("{:?} {:?} {:?} {:?} {:?}",s1,s2,k,len1,len2);
    if len1 > len2 {
        return get_k_th(nums2, nums1, s2, s1, k)
    }
    if len1 == 0 {
        return *nums2.get(s2 + k - 1).unwrap() as f64
    }

    if k == 1 {
        let num1 = nums1.get(s1).unwrap();
        let num2 = nums2.get(s2).unwrap();
        return if num1 < num2 {
            *num1 as f64
        } else {
            *num2 as f64
        }
    }

    let mid = k / 2;
    let i = s1 + if len1 < mid {
        len1
    } else {
        mid
    } - 1;
    let j = s2 + if len2 < mid {
        len2
    } else {
        mid
    } - 1;

    let num1 = nums1.get(i).unwrap();
    let num2 = nums2.get(j).unwrap();

    if num1 > num2 {
        get_k_th(nums1, nums2, s1, j + 1, k - (j - s2 + 1))
    } else {
        get_k_th(nums1, nums2, i + 1, s2, k - (i - s1 + 1))
    }
}


impl Solution {
    pub fn find_median_sorted_arrays_best(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 复杂度o log(m+n)
        let len = nums1.len() + nums2.len();
        let left_index = (len + 1) / 2;  //中位数左边的index
        let right_index = (len + 2) / 2; //获取到中位数右边的index 
        // 使得 奇数个获取index 相同 偶数index不同

        let left = get_k_th(&nums1, &nums2,0, 0, left_index);
        let right = get_k_th(&nums1, &nums2,0, 0, right_index);

        (left + right) / 2 as f64
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {  
        // 复杂度 o(m+n)
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
