// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }

// use std::collections::HashMap;
// 调试宏  之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体
#[derive(Debug)]
pub struct Solutions {}
fn dealcontent(
    grid: &Vec<Vec<char>>,
    vecdata: &mut Vec<Vec<bool>>,
    rowindex: usize,
    columnindex: usize,
    maxrows: usize,
    maxcolumns: usize,
) {
    if rowindex >= maxrows
        || columnindex >= maxcolumns
        || vecdata[rowindex][columnindex] == true
        || grid[rowindex][columnindex] == '0'
    {
        return;
    }
    vecdata[rowindex][columnindex] = true;
    let actions: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, 1], [0, -1]];
    for item_action in actions.iter() {
        let _rowindex: i32 = item_action[0] + rowindex as i32;
        let _columnindex: i32 = item_action[1] + columnindex as i32;
        if _rowindex >= 0 && _columnindex >= 0 {
            dealcontent(
                grid,
                vecdata,
                _rowindex as usize,
                _columnindex as usize,
                maxrows,
                maxcolumns,
            );
        }
    }
}
impl Solutions {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let maprow = grid.len();
        if maprow == 0 {
            return 0;
        }
        let mapcolumn = grid[0].len();
        if mapcolumn == 0 {
            return 0;
        }
        let mut vecdata: Vec<Vec<bool>> = Vec::new();
        // vecdata.reserve(maprow);
        let mut vecrow: Vec<bool> = Vec::new();
        vecrow.resize(mapcolumn, false);
        vecdata.resize(maprow, vecrow);
        //快速二维数组初始化
        let mut counts = 0;
        //访问过的节点置为true  然后
        for _i in 0..maprow {
            for _j in 0..mapcolumn {
                if vecdata[_i][_j] == false && grid[_i][_j] == '1' {
                    counts += 1;
                    dealcontent(&grid, &mut vecdata, _i, _j, maprow, mapcolumn);
                }
            }
        }
        counts
    }
}
