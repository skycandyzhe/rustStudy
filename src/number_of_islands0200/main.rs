mod number_of_islands;
pub use number_of_islands::Solutions;

fn num_islands(grid: Vec<Vec<char>>) -> i32 {
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

fn main() {
    // let mut contacts = HashMap<i32,i32>::new();
    let v = vec![2, 7, 11, 15];
    let tartget = 9;
    // let mut a = 1;
    // a = 2;
    // println!("{}", a);
    let sol = Solutions{};
    println!("{:?}",sol);
    let ret = sol.two_sum(v, tartget);
    println!("{:?}", &ret);
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
