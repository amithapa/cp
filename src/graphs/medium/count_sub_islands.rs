/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree, DSU
Problem: 1905. Count Sub Islands
Link: https://leetcode.com/problems/count-sub-islands/
 */

struct Solution;
impl Solution {
    fn is_valid(r: i32, c: i32, m: i32, n: i32) -> bool {
        r >= 0 && r < n && c >= 0 && c < m
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, r: i32, c: i32, m: i32, n: i32) {
        if !Self::is_valid(r, c, m, n) || grid[r as usize][c as usize] == 0 {
            return;
        }

        grid[r as usize][c as usize] = 0;

        let delta_row = [1, -1, 0, 0];
        let delta_col = [0, 0, 1, -1];

        for (dr, dc) in delta_row.into_iter().zip(delta_col) {
            let (nr, nc) = (r + dr, c + dc);
            Self::dfs(grid, nr, nc, m, n);
        }
    }

    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        let m = grid1[0].len();
        let n = grid1.len();

        // Remove islands which are not common
        for r in 0..n {
            for c in 0..m {
                if grid1[r][c] == 0 && grid2[r][c] == 1 {
                    Self::dfs(&mut grid2, r as i32, c as i32, m as i32, n as i32);
                }
            }
        }

        let mut sub_islands = 0;
        for r in 0..n {
            for c in 0..m {
                if grid2[r][c] == 1 {
                    Self::dfs(&mut grid2, r as i32, c as i32, m as i32, n as i32);
                    sub_islands += 1;
                }
            }
        }

        sub_islands
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_case_i() {
        let grid1 = vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 1, 1],
        ];
        let grid2 = vec![
            vec![1, 1, 1, 0, 0],
            vec![0, 0, 1, 1, 1],
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 1, 0],
            vec![0, 1, 0, 1, 0],
        ];

        assert_eq!(Solution::count_sub_islands(grid1, grid2), 3);
    }

    #[test]
    fn test_case_ii() {}
}
