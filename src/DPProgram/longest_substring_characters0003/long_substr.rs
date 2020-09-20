// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

// use std::collections::HashMap;
// 调试宏  之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体
use std::cmp;
pub struct Solution{}
impl Solution{
    pub fn length_of_longest_substring(s: String) -> i32 {

        let mut  char_vec:[i32; 128] = [-1; 128]; //上一次字母出现位置 默认为-1
        // let slen=s.len();
        let mut chars = s.chars();
        let mut index=0;
        let mut begin=0; //从该位置 到最后没有重复字符
        let mut contents=chars.next();
        let mut ret=0;
        while contents.is_some(){
            let charnum:usize=contents.unwrap() as usize;
            // if char_vec[charnum]!=-1{//前方存在相同字符
                begin=cmp::max(begin,char_vec[charnum]+1);//将开始坐标移动到相同字符的右边
            // }
            contents=chars.next();
            ret=cmp::max(ret,index-begin+1);
            char_vec[charnum]=index; //下一次出现改字符  长度 为index-char_vec[charnum]   abcdb 5-2
            index+=1;
        }
        ret
    }
}
