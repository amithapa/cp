/*
Source: LeetCode
Tags: Tree, DFS, Graphs, DSU
Problem: 959. Regions Cut By Slashes
Link: https://leetcode.com/problems/regions-cut-by-slashes/description/
 */

struct Solution;

impl Solution {
    fn inside(row: i32, col: i32, m: i32, n: i32) -> bool {
        row >= 0 && row < n && col >= 0 && col < m
    }
    fn dfs(
        grid: &Vec<Vec<char>>,
        row: i32,
        col: i32,
        m: i32,
        n: i32,
        corner: usize,
        visited: &mut Vec<Vec<Vec<bool>>>,
    ) {
        if !Self::inside(row, col, m, n) || visited[row as usize][col as usize][corner] {
            return;
        }

        visited[row as usize][col as usize][corner] = true;

        if corner == 0 {
            Self::dfs(grid, row - 1, col, m, n, 2, visited); // above me
        } else if corner == 1 {
            Self::dfs(grid, row, col + 1, m, n, 3, visited); // on the right
        } else if corner == 2 {
            Self::dfs(grid, row + 1, col, m, n, 0, visited); // below me
        } else if corner == 3 {
            Self::dfs(grid, row, col - 1, m, n, 1, visited); // on the left
        }

        if grid[row as usize][col as usize] != '/' {
            Self::dfs(grid, row, col, m, n, corner ^ 1, visited);
        }

        if grid[row as usize][col as usize] != '\\' {
            Self::dfs(grid, row, col, m, n, corner ^ 3, visited);
        }
    }

    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let mut new_grid: Vec<Vec<char>> = Vec::new();

        for g in grid.into_iter() {
            new_grid.push(g.chars().collect());
        }

        let mut regions = 0;
        let m = new_grid[0].len();
        let n = new_grid.len();
        let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; m]; n];

        for row in 0..n {
            for col in 0..m {
                for corner in 0..4 {
                    if !visited[row][col][corner] {
                        Self::dfs(
                            &new_grid,
                            row as i32,
                            col as i32,
                            m as i32,
                            n as i32,
                            corner,
                            &mut visited,
                        );
                        regions += 1;
                    }
                }
            }
        }
        regions
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_i() {
        let grid = vec![" /".to_owned(), "/ ".to_owned()];
        assert_eq!(Solution::regions_by_slashes(grid), 2);
    }

    #[test]
    fn test_case_ii() {
        let grid = vec![" /".to_owned(), "  ".to_owned()];
        assert_eq!(Solution::regions_by_slashes(grid), 1);
    }

    #[test]
    fn test_case_iii() {
        let grid = vec!["/\\".to_owned(), "\\/ ".to_owned()];
        assert_eq!(Solution::regions_by_slashes(grid), 5);
    }
}
