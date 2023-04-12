/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree, DSU
Problem: 1061. Lexicographically Smallest Equivalent String
Link: https://leetcode.com/problems/lexicographically-smallest-equivalent-string/
 */

struct Solution;

#[derive(Default)]
struct DSU {
    parent: [u8; 26],
}

impl DSU {
    fn new() -> Self {
        let mut dsu = DSU::default();
        for i in 0..26 {
            dsu.parent[i as usize] = i;
        }
        dsu
    }

    fn find(&mut self, node: u8) -> u8 {
        if self.parent[node as usize] == node {
            node
        } else {
            self.parent[node as usize] = self.find(self.parent[node as usize]);
            self.parent[node as usize]
        }
    }

    fn union(&mut self, u: u8, v: u8) {
        let u_root = self.find(u);
        let v_root = self.find(v);

        if u_root == v_root {
            return;
        }

        if u_root < v_root {
            self.parent[v_root as usize] = u_root;
        } else {
            self.parent[u_root as usize] = v_root;
        }
    }
}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut dsu = DSU::new();

        for (&ch1, &ch2) in s1.as_bytes().iter().zip(s2.as_bytes()) {
            dsu.union(ch1 - b'a', ch2 - b'a');
        }

        let mut result = String::new();

        for ch in base_str.bytes() {
            let smallest_ch = dsu.find(ch - b'a') + b'a';
            result.push(smallest_ch as char);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_case_i() {
        let s1 = "parker".to_string();
        let s2 = "morris".to_string();
        let base_str = "parser".to_string();

        assert_eq!(
            Solution::smallest_equivalent_string(s1, s2, base_str),
            "makkek".to_string()
        )
    }

    #[test]
    fn test_case_ii() {}
}
