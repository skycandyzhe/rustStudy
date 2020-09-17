// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

// use std::collections::HashMap;
// 调试宏  之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体

// pub use std::intrinsics;
#[derive(Debug)]
pub struct Solution {
    // pub temp: i32,
}   
// pub struct Point{
//     pub x:i32,
//     pub y:i32,
// }
// use std::collections::HashMap;

impl Solution {
  
 
    pub fn set_has_node(row:usize,column:usize,value:u8,hasnode:&mut Vec<Vec<Vec<bool>>>)->bool{
        let index3:usize =(row/3)*3+column/3;
        // println!("index {}",index3);
        //只要已经有元素表示异常
        if hasnode[0 ][row as usize][value as usize]==true||hasnode[1][column][value  as usize]==true ||hasnode[2][index3][value  as usize]==true{
            return false;
        }
        hasnode[0][row][value as usize]=true;
        hasnode[1][column][value as usize]=true;
        hasnode[2][index3][value as usize]=true;
        return true;
    }
    pub fn un_set_has_node(row:usize,column:usize,value:u8,hasnode:&mut Vec<Vec<Vec<bool>>>)->bool{
        let index3:usize =(row/3)*3+column/3;
        hasnode[0][row][value as usize]=false;
        hasnode[1][column][value as usize]=false;
        hasnode[2][index3][value as usize]=false;
        true
    }
    pub fn solve_sudoku_depth(board: &mut Vec<Vec<char>>,hasnode:&mut Vec<Vec<Vec<bool>>>)->bool
    {
        for _i in 0..9  {
            for _j in 0..9 {
                if board[_i][_j]=='.'{
                    for value in 0..9 {
                        //判断是否可以设置元素
                        if Solution::set_has_node(_i,_j,value,hasnode) {
                            //设置元素
                            board[_i][_j]=(('1' as u8)+value)  as char;
                            let ret=Solution::solve_sudoku_depth(board,hasnode);
                            if ret == true {
                                return true;
                            }
                            //退格 取消元素设置
                            Solution::un_set_has_node(_i,_j,value,hasnode);
                            board[_i][_j]='.';
                        
                        }
                    }
                    return false

                }
            }
        }
        return true;
    }
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut hasnode:Vec<Vec<Vec<bool>>> = Vec::new();
        let mut  t1:Vec<Vec<bool>>=Vec::new();
        let t2:Vec<bool>=vec![false;9];
        t1.resize(9, t2);
        hasnode.resize(3,t1);
        //初始化
        for _i in 0..9  {
            for _j in 0..9 {
                if board[_i][_j]!='.'{
                    let value=(board[_i][_j] as u8)-('1' as u8);
                    let ret=Solution::set_has_node(_i,_j,value,&mut hasnode);    
                    if ret == false {
                        panic!("error");
                        
                    }
                }
            }
        }
        let ret=Solution::solve_sudoku_depth(board ,&mut hasnode );
    }
   
}
