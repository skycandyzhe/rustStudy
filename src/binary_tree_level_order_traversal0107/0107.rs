// trait Solution{
//      fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> ;
// }
#[derive(Debug)]

// Definition for a binary tree node.

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        
    }
}
pub struct Solution {}
impl Solution {
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
