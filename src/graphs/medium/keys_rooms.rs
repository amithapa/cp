/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 841. Keys and Rooms
Link: https://leetcode.com/problems/keys-and-rooms/
 */

struct Solution;

impl Solution {
    fn dfs(rooms: &[Vec<i32>], visited: &mut Vec<bool>, source: usize) {
        visited[source] = true;
        for &dest in rooms[source].iter() {
            if !visited[dest as usize] {
                Self::dfs(rooms, visited, dest as usize);
            }
        }
    }

    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visited: Vec<bool> = vec![false; n];
        Self::dfs(&rooms, &mut visited, 0);
        visited.iter().all(|&x| x)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_i() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];

        assert_eq!(Solution::can_visit_all_rooms(rooms), true);
    }

    #[test]
    fn test_case_ii() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];

        assert_eq!(Solution::can_visit_all_rooms(rooms), false);
    }
}
