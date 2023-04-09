/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1706. Where Will the Ball Fall
Link: https://leetcode.com/problems/where-will-the-ball-fall/description/
 */

struct Solution;
impl Solution {
    fn dfs(grid: &Vec<Vec<i32>>, n: usize, m: usize, r: usize, c: usize) -> i32 {
        // println!("{r} {r} <-- {n}");
        // is stuck on left wall
        if c == 0 && grid[r][c] == -1 {
            return -1;
        }

        // is stuck on right wall
        if c == m - 1 && grid[r][c] == 1 {
            return -1;
        }

        // is stuck in v
        if grid[r][c] == -1 && grid[r][c - 1] == 1 || grid[r][c] == 1 && grid[r][c + 1] == -1 {
            return -1;
        }

        if r == n - 1 {
            if grid[r][c] == 1 {
                return (c + 1) as i32;
            } else {
                return (c - 1) as i32;
            }
        }

        if grid[r][c] == 1 {
            Self::dfs(grid, n, m, r + 1, c + 1)
        } else {
            Self::dfs(grid, n, m, r + 1, c - 1)
        }
    }

    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid[0].len();
        let n = grid.len();

        let mut result = vec![-1; m];

        for i in 0..m {
            result[i] = Self::dfs(&grid, n, m, 0, i);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_i() {
        let grid = vec![
            vec![1, 1, 1, -1, -1],
            vec![1, 1, 1, -1, -1],
            vec![-1, -1, -1, 1, 1],
            vec![1, 1, 1, 1, -1],
            vec![-1, -1, -1, -1, -1],
        ];

        assert_eq!(Solution::find_ball(grid), vec![1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_case_ii() {
        let grid = vec![vec![-1]];

        assert_eq!(Solution::find_ball(grid), vec![-1]);
    }

    #[test]
    fn test_case_iii() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1],
        ];
        assert_eq!(Solution::find_ball(grid), vec![0, 1, 2, 3, 4, -1]);
    }
}
