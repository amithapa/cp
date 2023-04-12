/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 695. Max Area of Island
Link: https://leetcode.com/problems/max-area-of-island/
 */

struct Solution;
impl Solution {
    fn is_valid(r: i32, c: i32, m: i32, n: i32) -> bool {
        r >= 0 && r < n && c >= 0 && c < m
    }

    fn dfs(
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        r: i32,
        c: i32,
        m: i32,
        n: i32,
    ) -> i32 {
        visited[r as usize][c as usize] = true;

        let delta_row = [0, 0, 1, -1];
        let delta_col = [1, -1, 0, 0];

        let mut total_islands = 1;
        for (dr, dc) in delta_row.iter().zip(delta_col) {
            let (nr, nc) = (r + dr, c + dc);

            if Self::is_valid(nr, nc, m, n)
                && grid[nr as usize][nc as usize] == 1
                && !visited[nr as usize][nc as usize]
            {
                total_islands += Self::dfs(grid, visited, nr, nc, m, n);
            }
        }

        total_islands
    }

    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let n = grid.len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];

        let mut max_islands = 0;
        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == 1 && !visited[r][c] {
                    let total_islands =
                        Self::dfs(&grid, &mut visited, r as i32, c as i32, m as i32, n as i32);
                    max_islands = max_islands.max(total_islands);
                }
            }
        }
        max_islands
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_case_i() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];

        assert_eq!(Solution::max_area_of_island(grid), 6);
    }

    #[test]
    fn test_case_ii() {
        let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(Solution::max_area_of_island(grid), 0);
    }
}
