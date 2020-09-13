// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

use std::collections::VecDeque;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
//   #[inline]
    pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
  pub fn new1(val: i32,next:Option<Box<ListNode>>) -> Self {
    ListNode {
      next: next,
      val
    }
  }
}


// use std::rc::Rc;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::collections::BinaryHeap;
#[derive(Debug)]
pub struct Solution {}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut  deque1:VecDeque<i32>=VecDeque::new();
        let mut index1:& Option<Box<ListNode>> =&l1;
        //入栈 考虑如果有其他变种题目 换一下栈就可以实现
        while index1.is_some(){
            deque1.push_back(index1.as_ref().unwrap().val);
            index1=&(index1.as_ref().unwrap().next);
        }
        let mut deque2:VecDeque<i32>=VecDeque::new();
        let mut index2:& Option<Box<ListNode>> =&l2;
        while index2.is_some(){
            deque2.push_back(index2.as_ref().unwrap().val);
            index2=&(index2.as_ref().unwrap().next);
        }
        let mut head = ListNode::new(0);
        let mut cur = &mut head.next;
        let mut jinwei=0;
        while !deque1.is_empty() ||!deque2.is_empty() ||jinwei!=0 {
            let mut temp:i32=jinwei;
            if !deque1.is_empty() {
                temp+=deque1.pop_front().unwrap();
            }
            if !deque2.is_empty() {
                temp+=deque2.pop_front().unwrap();
            }
            jinwei=temp/10;
            // print!("{}",jinwei);
            // 插入结果  引用指向下一个节点
            *cur = Some(Box::new(ListNode::new(temp%10)));
            cur=&mut cur.as_mut().unwrap().next;
        } 
        head.next
    }
}

impl Solution {


// // 闭包解决
//     pub fn add_two_numbers(
//         l1: Option<Box<ListNode>>,
//         l2: Option<Box<ListNode>>,
//     )->Option<Box<ListNode>>{
//         // 声明变量
//         let mut head = ListNode::new(0);
//         let mut cur = &mut head.next;
//         let (mut x,mut y) = (l1,l2);
//         let mut carry = 0;
//         let node_val = |node:&Option<Box<ListNode>>| node.as_ref().map_or(0, |x| x.val);
//         let node_next = |node:Option<Box<ListNode>>| node.map_or(None, |x| x.next);
//         // 创建闭包
//         while x.is_some() || y.is_some() || carry!=0 {
//             let sum = node_val(&x)+node_val(&y)+carry;
//             *cur = Some(Box::new(ListNode::new(sum % 10)));
//             cur = &mut cur.as_mut().unwrap().next;
//             carry = sum  / 10;
//             x = node_next(x);
//             y = node_next(y);
//         }
//         // while 循环比较
        
//         head.next
//     }
}


