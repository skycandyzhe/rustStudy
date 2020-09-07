mod number_of_islands;
pub use number_of_islands::Solutions;

fn test1() {
    let mut v = Vec::new();

    let v1 = vec!['1', '1', '0', '1', '0'];
    v.push(v1);
    let v1 = vec!['1', '1', '0', '1', '0'];
    v.push(v1);
    let v1 = vec!['0', '0', '1', '0', '0'];
    v.push(v1);
    let v1 = vec!['0', '0', '0', '1', '1'];
    v.push(v1);
    let sol = Solutions::num_islands(v);
    println!("{}", sol);
    // 输出: 1
}
fn test2() {
    let v: Vec<Vec<char>> = Vec::new();
    let sol = Solutions::num_islands(v);
    println!("{}", sol);
    // 输出: 1
}
fn main() {
    test1();
    test2();
    // println!("Hello, world!");
}
