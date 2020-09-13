mod wordsearch;
pub use wordsearch::Solution;
//变量绑定 let a1 = 5;  类似c++ 常量
//可变绑定 let mut=10 ;

fn main() {
    // let mut contacts = HashMap<i32,i32>::new();
    let v = vec![2, 7, 11, 15];
    let tartget = 9;
    // let mut a = 1;
    // a = 2;
    // println!("{}", a);
    let sol = Solutions::two_sum(v, tartget);
    println!("{:?}", sol);
    // let sol = Solution::new();
    // let ret = sol.two_sum(&v, 100);

    // println!("{:?}", v);
    // println!("{:?}", ret);
    // for elem in &v {
    //     println!(" {}", elem);
    // }
    // Solution
    println!("Hello, world!");
}
