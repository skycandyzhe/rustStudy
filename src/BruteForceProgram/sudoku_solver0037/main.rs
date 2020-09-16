mod wordsearch;
pub use wordsearch::Solution;


// 编写一个程序，通过已填充的空格来解决数独问题。

// 一个数独的解法需遵循如下规则：

// 数字 1-9 在每一行只能出现一次。
// 数字 1-9 在每一列只能出现一次。
// 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
// 空白格用 '.' 表示。
// 答案被标成红色。
//暴力破解 

fn test1() {
    let mut board:Vec<Vec<char>>=Vec::new();
    let v1=vec!['5','3','.','.','7','.','.','.','.'];
    board.push(v1);
    let v1=vec!['6','.','.','1','9','5','.','.','.'];
    board.push(v1);
    let v1=vec!['.','9','8','.','.','.','.','6','.'];
    board.push(v1);
    let v1=vec!['8','.','.','.','6','.','.','.','3'];
    board.push(v1);
    let v1=vec!['4','.','.','8','.','3','.','.','1'];
    board.push(v1);
    let v1=vec!['7','.','.','.','2','.','.','.','6'];
    board.push(v1);
    let v1=vec!['.','6','.','.','.','.','2','8','.'];
    board.push(v1);
    let v1=vec!['.','.','.','4','1','9','.','.','5'];
    board.push(v1);
    let v1=vec!['.','.','.','.','8','.','.','7','9'];
    board.push(v1);

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
