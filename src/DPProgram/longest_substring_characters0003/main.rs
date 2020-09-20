mod long_substr;
pub use long_substr::Solution;

//循环一遍就可以了
fn test1() {
    let str1=String::from("abcabcbb");
    let sol = Solution::length_of_longest_substring(str1);
    println!("{}", sol);
}
fn test2() {
    let str1=String::from("a");
    let sol = Solution::length_of_longest_substring(str1);
    println!("{}", sol);
    // 输出: 1
}
fn main() {
    test1();
    test2();
    // println!("Hello, world!");
}
