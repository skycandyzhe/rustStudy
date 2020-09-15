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
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {

    }
}

impl Solution {
  
    //去获取边上节点的元素 是否和下一个元素相符合 ，如果下一个元素已经不在，返回true
    pub fn exist_deepth(board: &Vec<Vec<char>>,word:&String,wordindex:usize,row:i32,column:i32,has_vis:&mut Vec<Vec<bool>>,maxrow:i32,maxcolumn:i32)->bool{
        //语法不是很熟悉 获取到下一个元素 考虑有什么方法直接将chars传入 现在好像很耗时 
        let needchars=word.chars().nth(wordindex);
        let chardata=if let Some(data) = needchars {
            data
        }else{
            return true;
        };
        let actions: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, 1], [0, -1]];
        for item_action in actions.iter() {
            let _rowindex: i32 = item_action[0] + row as i32;
            let _columnindex: i32 = item_action[1] + column as i32;
            if _rowindex <0||_columnindex<0 ||has_vis[_rowindex as usize][_columnindex as usize]==true{
                continue;
            }
            let tempwordvec=board.get(_rowindex  as usize);
            if tempwordvec.is_none(){
                continue;
            }
            let tempwordvec=tempwordvec.unwrap();
            let tempword=tempwordvec.get(_columnindex as usize);
            if tempword.is_none(){
                continue;
            }
            let tempword=tempword.unwrap();
            if *tempword==chardata{
                has_vis[_rowindex as usize][_columnindex as usize]=true;
                if Solution::exist_deepth(
                    board,
                    word,
                    wordindex+1,
                    _rowindex  ,
                    _columnindex  ,
                    has_vis,
                    maxrow,
                    maxcolumn
                )==true{
                    return true;
                }
                has_vis[_rowindex as usize][_columnindex as usize]=false;
            }

           
        }

        false

    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        //获取行列
        let datarow=board.len();
        let mut datacolumn=0;
        if datarow!= 0 {
            datacolumn=board[0].len();
        }
        //快速二维数组初始化
        let mut map: Vec<Vec<bool>> = Vec::new();
        let mut vectemp:Vec<bool>=Vec::new();
        vectemp.resize(datacolumn+1, false);
        map.resize(datarow+1, vectemp);

        // let mut wordchars=word.as_mut_str();
        // let wordlen=word.len();
        let wordindex:usize=0;
        // print!("{:?}v{:?}",word.chars().nth(2),wordlen);
        let needchars=word.chars().nth(0);
        let chardata=if let Some(data) = needchars {
            data
        }else{
            return true;
        };

        //遍历元素相同的 
        for (i,v) in board.iter().enumerate(){
            // println!("{} {:?}",i,v);
            for (j,v2) in v.iter().enumerate() {
                //判断值是否相同
                if *v2==chardata{
                    //递归遍历下一个节点
                    map[i][j]=true;
                    if Solution::exist_deepth(&board, &word,wordindex+1,i as i32,j as i32, &mut map, datarow as i32,datacolumn as i32)==true{
                        return true;
                    }
                    map[i][j]=false;
                }
                
            }
        }
        false
        
    }
   
}
