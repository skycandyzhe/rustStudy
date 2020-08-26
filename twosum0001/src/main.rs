mod twosum;
use twosum::twosumModel::Solution;
//变量绑定 let a1 = 5;  类似c++ 常量
//可变绑定 let mut=10 ;

//内置类型

fn main() {
    let v = vec![2, 7, 11, 15];

    let mut a = 1;
    a = 2;
    println!("{}", a);
    let sol = Solution;
    sol.twosum(v, 100);
    // let sol = Solution::new();
    // let ret = sol.two_sum(&v, 100);

    println!("{:?}", v);
    // println!("{:?}", ret);
    for elem in &v {
        println!(" {}", elem);
    }
    // Solution
    println!("Hello, world!");
}
