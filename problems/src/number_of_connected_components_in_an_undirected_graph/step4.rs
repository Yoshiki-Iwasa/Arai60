// Step4
// 目的: 指摘対応

pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut adjacent_list = HashMap::<usize, Vec<usize>>::new();
        let mut seen = vec![false; n as usize];

        for (index, node_pair) in edges.iter().enumerate() {
            assert!(
                node_pair.len() == 2,
                "Invalid input. edges[{}]: {:?}",
                index,
                node_pair
            );
            adjacent_list
                .entry(node_pair[0] as usize)
                .or_insert(vec![])
                .push(node_pair[1] as usize);
            adjacent_list
                .entry(node_pair[1] as usize)
                .or_insert(vec![])
                .push(node_pair[0] as usize);
        }

        let mut components_count = 0;

        for node in 0..n as usize {
            if !seen[node] {
                components_count += 1;
                Self::visit_connected_node(&adjacent_list, &mut seen, node);
            }
        }

        components_count
    }

    fn visit_connected_node(
        adjacent_list: &HashMap<usize, Vec<usize>>,
        seen: &mut [bool],
        node: usize,
    ) {
        if seen[node] {
            return;
        }

        seen[node] = true;

        if let Some(next_nodes) = adjacent_list.get(&node) {
            for next_node in next_nodes {
                Self::visit_connected_node(adjacent_list, seen, *next_node)
            }
        };
    }
}

// UnionFindをつかった解法
pub struct UnionFind {
    roots: Vec<usize>,
    ranks: Vec<usize>,
    root_count: u32,
}

impl UnionFind {
    pub fn new(num_of_nodes: usize) -> Self {
        let mut roots = vec![];
        let mut ranks = vec![];
        let mut root_count = 0;
        for node in 0..num_of_nodes {
            roots.push(node);
            ranks.push(0);
            root_count += 1;
        }

        Self {
            roots,
            ranks,
            root_count,
        }
    }

    pub fn root(&mut self, node: usize) -> usize {
        if self.roots[node] == node {
            return node;
        }

        self.roots[node] = self.root(self.roots[node]);
        self.roots[node]
    }

    pub fn unite(&mut self, a: usize, b: usize) {
        let root_a = self.root(a);
        let root_b = self.root(b);

        if root_a == root_b {
            return;
        }

        if self.ranks[root_a] < self.ranks[root_b] {
            self.roots[root_a] = root_b
        } else if self.ranks[root_a] > self.ranks[root_b] {
            self.roots[root_b] = root_a
        } else {
            self.roots[root_b] = root_a;
            self.ranks[root_a] += 1;
        }
        self.root_count -= 1;
    }

    pub fn get_root_count(&self) -> u32 {
        self.root_count
    }
}

impl Solution {
    pub fn count_component_union_find(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut union_find = UnionFind::new(n as usize);

        for (index, node_pair) in edges.iter().enumerate() {
            assert!(
                node_pair.len() == 2,
                "Invalid input. edges[{}]: {:?}",
                index,
                node_pair
            );
            union_find.unite(node_pair[0] as usize, node_pair[1] as usize);
        }

        union_find.get_root_count() as i32
    }
}
