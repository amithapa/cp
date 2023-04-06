/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 419. Battleships in a Board
Link: https://leetcode.com/problems/battleships-in-a-board/description/
*/

struct Solution;

impl Solution {
    fn dfs(board: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, r: i32, c: i32, m: i32, n: i32) {
        visited[r as usize][c as usize] = true;

        let delta_r = [0, 0, 1, -1];
        let delta_c = [1, -1, 0, 0];

        for (dr, dc) in delta_r.iter().zip(delta_c) {
            let nr = r + dr;
            let nc = c + dc;

            if nr >= 0 && nr < n && nc >= 0 && nc < m && !visited[nr as usize][nc as usize] {
                if board[nr as usize][nc as usize] == 'X' {
                    Self::dfs(board, visited, nr, nc, m, n);
                }
            }
        }
    }

    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let n = board.len();
        let m = board[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];

        let mut battleships = 0;
        for i in 0..n {
            for j in 0..m {
                if board[i][j] == 'X' && !visited[i][j] {
                    battleships += 1;
                    Self::dfs(&board, &mut visited, i as i32, j as i32, m as i32, n as i32);
                }
            }
        }

        battleships
    }
}
