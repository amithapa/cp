/*
Source: LeetCode
Tags: BackTracking, DFS, BFS, Graph
Problem: 797. All Paths From Source to Target
Link: https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/
 */

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn all_paths_source_target_bfs(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let n = graph.len();

        let mut result = Vec::new();

        queue.push_back((0, vec![]));

        while let Some((node, mut path)) = queue.pop_front() {
            println!("--> {node}");
            path.push(node as i32);

            for neighbor in graph[node as usize].iter() {
                queue.push_back((*neighbor as usize, path.clone()))
            }

            if node == n - 1 {
                result.push(path);
            }
        }

        result
    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(graph: &Vec<Vec<i32>>, node: i32, path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
            if node == (graph.len() as i32) - 1 {
                paths.push(path.clone());
                return;
            }

            for neighbor in graph[node as usize].iter() {
                path.push(*neighbor);
                dfs(graph, *neighbor, path, paths);
                path.pop();
            }
        }
        let mut result = Vec::new();

        dfs(&graph, 0, &mut vec![0], &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_i() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];

        println!("{:?}", Solution::all_paths_source_target(graph));
    }
}
