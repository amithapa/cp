/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree, DSU
Problem: 1020. Number of Enclaves
Link: https://leetcode.com/problems/number-of-enclaves/
 */

struct DSU {
    parent: Vec<usize>,
    boundary: Vec<bool>,
}

impl DSU {
    fn new(size: usize) -> Self {
        let mut dsu = DSU {
            parent: vec![0; size],
            boundary: vec![false; size],
        };

        for index in 0..size {
            dsu.parent[index] = index;
        }
        dsu
    }

    fn find(&mut self, mut node: usize) -> usize {
        while node != self.parent[node] {
            let prev_node = node;
            node = self.parent[node];
            self.parent[prev_node] = node;
        }
        node
    }

    fn union(&mut self, u: usize, v: usize) {
        let u_root = self.find(u);
        let v_root = self.find(v);

        if u_root == v_root {
            return;
        }
        if self.boundary[u_root] {
            self.parent[v_root] = u_root;
            self.boundary[v_root] = true;
        } else if self.boundary[v_root] {
            self.parent[u_root] = v_root;
            self.boundary[u_root] = true;
        } else {
            self.parent[u_root] = v_root;
        }
    }
}

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

    fn is_boundary(r: usize, c: usize, m: usize, n: usize) -> bool {
        r == 0 || r == n - 1 || c == 0 || c == m - 1
    }

    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let n = grid.len();
        let mut dsu = DSU::new(m * n);

        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == 0 {
                    continue;
                }
                let current_index = r * m + c;
                if Self::is_boundary(r, c, m, n) {
                    dsu.boundary[current_index] = true;
                }

                // left
                if Self::is_valid(r as i32 - 1, c as i32, m as i32, n as i32) && grid[r - 1][c] == 1
                {
                    let left_index = (r - 1) * m + c;
                    if Self::is_boundary(r - 1, c, m, n) {
                        dsu.boundary[left_index] = true;
                    }
                    dsu.union(current_index, left_index);
                }

                // up
                if Self::is_valid(r as i32, c as i32 - 1, m as i32, n as i32) && grid[r][c - 1] == 1
                {
                    let up_index = r * m + (c - 1);
                    if Self::is_boundary(r, c - 1, m, n) {
                        dsu.boundary[up_index] = true;
                    }
                    dsu.union(current_index, up_index);
                }
            }
        }

        let mut enclaves = 0;

        for row in 0..n {
            for col in 0..m {
                let index = row * m + col;

                if grid[row][col] == 1 {
                    let parent = dsu.find(index);
                    if !dsu.boundary[parent] {
                        enclaves += 1;
                    }
                }
            }
        }
        enclaves
    }

    pub fn num_enclaves_dfs(mut grid: Vec<Vec<i32>>) -> i32 {
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
