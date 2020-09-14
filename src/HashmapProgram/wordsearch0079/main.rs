mod wordsearch;
pub use wordsearch::Solution;



// 给定一个二维网格和一个单词，找出该单词是否存在于网格中。

// 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。
//深度搜索
fn test1() {
    let mut board:Vec<Vec<char>>=Vec::new();
    let v1=vec!['A','B','C','E'];
    let v2=vec!['S','F','C','S'];
    let v3=vec!['A','D','E','E'];
    board.push(v1);
    board.push(v2);
    board.push(v3);

    let  w1= String::from("ABCCED");
    let ret=Solution::exist(board,w1);
    println!("{:?}",ret);


}
fn test2() {
    let mut board:Vec<Vec<char>>=Vec::new();
    let v1=vec!['A','B','C','E'];
    let v2=vec!['S','F','C','S'];
    let v3=vec!['A','D','E','E'];
    board.push(v1);
    board.push(v2);
    board.push(v3);

    let w2= String::from("SEE");

    let ret=Solution::exist(board,w2);
    println!("{:?}",ret);

}
fn test3() {
    let mut board:Vec<Vec<char>>=Vec::new();
    let v1=vec!['A','B','C','E'];
    let v2=vec!['S','F','C','S'];
    let v3=vec!['A','D','E','E'];
    board.push(v1);
    board.push(v2);
    board.push(v3);

    let w3= String::from("SEF");
    let ret=Solution::exist(board,w3);
    println!("{:?}",ret);

}


fn main() {
    test1();
    test2();
    test3();
    // println!("Hello, world!");
}
