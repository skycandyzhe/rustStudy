mod number_of_islands;
pub use number_of_islands::Solutions;

fn num_islands(grid: &Vec<Vec<char>>) -> i32 {
    // let mut map = HashMap::new();
    let maprow=grid.len();
    let mapcolumn=grid[0].len();
    let mut vecdata:Vec<Vec<bool>>=Vec::new();
    vecdata.reserve(maprow);
    // vecdata.resize(maprow,vecrow);
    for _i in 0..maprow {
        let mut vecrow:Vec<bool>=Vec::new();
        vecrow.resize(mapcolumn,false);
        // vecrow.resize_default(mapcolumn);
        vecdata.push(vecrow);
    }
    println!("{:?}",vecdata);
    println!("{:?} {:?}",maprow,mapcolumn);
    0
}
fn test1(){
    let mut  v=Vec::new();
    
    let v1=vec!['1','1','1','1','0'];
    v.push(v1);
    let v1=vec!['1','1','0','1','0'];
    v.push(v1);
    let v1=vec!['1','1','0','0','0'];
    v.push(v1);
    let v1=vec!['0','0','0','0','0'];
    v.push(v1);
    num_islands(&v);
    println!("{:?}",v);
// 输出: 1

}
fn main() {
    test1();

    // // let mut contacts = HashMap<i32,i32>::new();
    // let v = vec![2, 7, 11, 15];
    // let tartget = 9;
    // // let mut a = 1;
    // // a = 2;
    // // println!("{}", a);
    // let sol = Solutions{};
    // println!("{:?}",sol);
    // let ret = sol.two_sum(v, tartget);
    // println!("{:?}", &ret);
    // // let sol = Solution::new();
    // // let ret = sol.two_sum(&v, 100);

    // // println!("{:?}", v);
    // // println!("{:?}", ret);
    // // for elem in &v {
    // //     println!(" {}", elem);
    // // }
    // // Solution
    println!("Hello, world!");
}
