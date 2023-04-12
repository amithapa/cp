/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1020. Number of Enclaves
Link: https://leetcode.com/problems/number-of-enclaves/
 */

struct Solution;
impl Solution {
    fn is_valid(row: i32, col: i32, m: i32, n: i32) -> bool {
        row >= 0 && row < n && col >= 0 && col < m
    }

    fn dfs(
        grid: &mut Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        row: i32,
        col: i32,
        m: i32,
        n: i32,
    ) {
        if !Self::is_valid(row, col, m, n)
            || visited[row as usize][col as usize]
            || grid[row as usize][col as usize] == 0
        {
            return;
        }

        visited[row as usize][col as usize] = true;
        grid[row as usize][col as usize] = 0;
        let delta_row = [1, -1, 0, 0];
        let delta_col = [0, 0, 1, -1];

        for (dr, dc) in delta_row.into_iter().zip(delta_col) {
            let nr = row + dr;
            let nc = col + dc;
            Self::dfs(grid, visited, nr, nc, m, n);
        }
    }

    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let n = grid.len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];

        for i in 0..n {
            Self::dfs(&mut grid, &mut visited, i as i32, 0, m as i32, n as i32);
            Self::dfs(
                &mut grid,
                &mut visited,
                i as i32,
                m as i32 - 1,
                m as i32,
                n as i32,
            );
        }
        for i in 0..m {
            Self::dfs(&mut grid, &mut visited, 0, i as i32, m as i32, n as i32);
            Self::dfs(
                &mut grid,
                &mut visited,
                n as i32 - 1,
                i as i32,
                m as i32,
                n as i32,
            );
        }
        let mut enclaves = 0;
        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == 1 {
                    enclaves += 1;
                }
            }
        }

        enclaves
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_case_i() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];

        assert_eq!(Solution::num_enclaves(grid), 3);
    }

    #[test]
    fn test_case_iii() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];

        assert_eq!(Solution::num_enclaves(grid), 0);
    }

    #[test]
    fn test_case_ii() {
        let grid = vec![
            vec![0, 0, 0, 1, 1, 1, 0, 1, 0, 0],
            vec![1, 1, 0, 0, 0, 1, 0, 1, 1, 1],
            vec![0, 0, 0, 1, 1, 1, 0, 1, 0, 0],
            vec![0, 1, 1, 0, 0, 0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 0, 1],
            vec![0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            vec![0, 0, 1, 0, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 1],
        ];
        assert_eq!(Solution::num_enclaves(grid), 3);
    }
}
