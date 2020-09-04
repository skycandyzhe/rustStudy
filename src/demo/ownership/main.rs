
fn main() {
    let reference_to_nothing = dangle();
    println!("{}",reference_to_nothing);
}
//垂悬引用  不可用
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
fn dangle() -> String {
    let s = String::from("hello");

    s
}