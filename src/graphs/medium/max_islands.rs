/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree, DSU
Problem: 695. Max Area of Island
Link: https://leetcode.com/problems/max-area-of-island/
 */

struct DSU {
    parent: Vec<usize>,
    size: Vec<i32>,
    max_size: i32,
}

impl DSU {
    fn new(size: usize) -> Self {
        let mut dsu = DSU {
            parent: vec![0; size],
            size: vec![1; size],
            max_size: 0,
        };

        for i in 0..size {
            dsu.parent[i] = i;
        }
        dsu
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            node
        } else {
            self.parent[node] = self.find(self.parent[node]);
            self.parent[node]
        }
    }

    fn union(&mut self, u: usize, v: usize) {
        let u_root = self.find(u);
        let v_root = self.find(v);

        if u_root == v_root {
            return;
        }

        if self.size[u_root] < self.size[v_root] {
            self.parent[u_root] = v_root;
            self.size[v_root] += self.size[u_root];
            self.max_size = self.max_size.max(self.size[v_root]);
        } else {
            self.parent[v_root] = u_root;
            self.size[u_root] += self.size[v_root];
            self.max_size = self.max_size.max(self.size[u_root]);
        }
    }
}

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

    pub fn max_area_of_island_dfs(grid: Vec<Vec<i32>>) -> i32 {
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

    // DSU implementation
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let n = grid.len();
        let mut dsu = DSU::new(m * n);
        let mut first_island = true;

        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == 1 {
                    if first_island {
                        dsu.max_size = dsu.max_size.max(1);
                        first_island = false;
                    }

                    let nc = c + 1;
                    if Self::is_valid(r as i32, nc as i32, m as i32, n as i32) && grid[r][nc] == 1 {
                        dsu.union(r * m + c, r * m + nc);
                    }

                    let nr = r + 1;
                    if Self::is_valid(nr as i32, c as i32, m as i32, n as i32) && grid[nr][c] == 1 {
                        dsu.union(r * m + c, nr * m + c);
                    }
                }
            }
        }
        dsu.max_size
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
