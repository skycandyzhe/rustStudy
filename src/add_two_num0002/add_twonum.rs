// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
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
        
    }
}

impl Solution {
    pub fn add_two_numbers_1(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dump_head = ListNode::new(0);
        let mut current = &mut dump_head;
        let mut carry = 0;
        let mut p = l1.as_ref();
        let mut q = l2.as_ref();
        while p.is_some() || q.is_some() {
            let sum = match (&p, &q) {
                (Some(l1), Some(l2)) => l1.val + l2.val + carry,
                (Some(l1), None) => l1.val + carry,
                (None, Some(l2)) => l2.val + carry,
                (None, None) => 0 + carry,
            };
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
            
            if p.is_some() && p.unwrap().next.is_some() {
                p = p.unwrap().next.as_ref()
            }else{
                p = None;
            }
            if q.is_some() && q.unwrap().next.is_some() {
                println!("cccc");
                q = q.unwrap().next.as_ref();
            }else{
                q = None;
            }
        }
        if (carry > 0) {
            current.next = Some(Box::new(ListNode::new(carry)));
        }
        dump_head.next
    }

// 闭包解决
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    )->Option<Box<ListNode>>{
        // 声明变量
        let mut head = ListNode::new(0);
        let mut cur = &mut head.next;
        let (mut x,mut y) = (l1,l2);
        let mut carry = 0;
        let node_val = |node:&Option<Box<ListNode>>| node.as_ref().map_or(0, |x| x.val);
        let node_next = |node:Option<Box<ListNode>>| node.map_or(None, |x| x.next);
        // 创建闭包
        while x.is_some() || y.is_some() || carry!=0 {
            let sum = node_val(&x)+node_val(&y)+carry;
            *cur = Some(Box::new(ListNode::new(sum % 10)));
            cur = &mut cur.as_mut().unwrap().next;
            carry = sum  / 10;
            x = node_next(x);
            y = node_next(y);
        }
        // while 循环比较
        
        head.next
    }
}


