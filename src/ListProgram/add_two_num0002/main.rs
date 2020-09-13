mod add_twonum;
pub use add_twonum::Solution;
pub use add_twonum::ListNode;

// 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。

// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。

// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。

// 示例：

// 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
// 输出：7 -> 0 -> 8
// 原因：342 + 465 = 807

//方案 入栈 然后栈顶元素计算
fn test1() {
    let f1=ListNode::new1(8,None);
    let f2=ListNode::new1(4,Option::Some(Box::new(f1)));
    let first: Option<Box<ListNode>>=Option::Some(Box::new(ListNode::new1(2,Option::Some(Box::new(f2)))));
    

    let s1=ListNode::new1(3,None);
    let s2=ListNode::new1(4,Option::Some(Box::new(s1)));
    let second: Option<Box<ListNode>>=Option::Some(Box::new(ListNode::new1(2,Option::Some(Box::new(s2)))));
    // let candidates =vec![10,1,2,7,6,1,5];
    // let k=8;
    let ret=Solution::add_two_numbers(first,second);

    println!("{:?}  ",ret);
}
fn test2() {
    // 1586+85
    let mut  f1=ListNode::new(5);
    let mut f2=ListNode::new(6);
    let  f3=ListNode::new(4);
    f2.next= Some(Box::new(f3));
    f1.next= Some(Box::new(f2));
    
    // let first: Option<Box<ListNode>>=Option::Some(Box::new(ListNode::new1(2,Option::Some(Box::new(f2)))));
    
    // 5558+558=6116
    let mut s1=ListNode::new(2);
    let mut s2=ListNode::new(4);
    let s3=ListNode::new(3);
    s2.next= Some(Box::new(s3));
    s1.next= Some(Box::new(s2));
    // let candidates =vec![10,1,2,7,6,1,5];
    // let k=8;
    let ret=Solution::add_two_numbers(Some(Box::new(f1)),Some(Box::new(s1)));

    println!("{:?}  ",ret);
}

fn main() {
    test1();
    test2();
    // test3();
    // test4();
    // test5();
    // println!("Hello, world!");
}
