/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1992. Find All Groups of Farmland
Link: https://leetcode.com/problems/find-all-groups-of-farmland/description/
 */

struct Solution;
impl Solution {
    pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();

        let n = land.len();
        let m = land[0].len();

        for i in 0..n {
            for j in 0..m {
                if land[i][j] == 1 {
                    let mut x = i;
                    let mut y = j;

                    while x < n && land[x][j] == 1 {
                        y = j;
                        while y < m && land[x][y] == 1 {
                            land[x][y] = 0;
                            y += 1;
                        }
                        x += 1;
                    }
                    results.push(vec![i as i32, j as i32, x as i32 - 1, y as i32 - 1]);
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_case_i() {
        let land = vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]];
        let results = vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]];

        assert_eq!(Solution::find_farmland(land), results);
    }
}
