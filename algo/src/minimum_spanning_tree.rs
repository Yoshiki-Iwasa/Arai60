// Primのアルゴリズム
// すでにエッジが選択された頂点集合から伸びるエッジのうち最小となるエッジを選択していく
// すべてのnodeを訪れることで終了
// ダイクストラのようにBinaryHeapを使って実装

// https://leetcode.com/problems/connecting-cities-with-minimum-cost/description/?envType=problem-list-v2&envId=minimum-spanning-tree

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn minimum_cost_1(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    if connections.len() < n - 1 {
        return -1;
    }

    let mut graph = HashMap::<i32, Vec<(i32, usize)>>::new();
    for connection in &connections {
        graph
            .entry(connection[0])
            .or_default()
            .push((connection[1], connection[2] as usize));
        graph
            .entry(connection[1])
            .or_default()
            .push((connection[0], connection[2] as usize));
    }

    let mut priority_queue = BinaryHeap::<Reverse<(usize, i32)>>::new();
    let mut visited = HashSet::<i32>::with_capacity(n);

    graph
        .get(&connections[0][0])
        .unwrap()
        .iter()
        .for_each(|&(node, cost)| priority_queue.push(Reverse((cost, node))));

    visited.insert(connections[0][0]);
    let mut min_cost = 0;
    while let Some(Reverse((cost, node))) = priority_queue.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        min_cost += cost;

        graph
            .get(&node)
            .unwrap()
            .iter()
            .for_each(|&(next_node, cost)| priority_queue.push(Reverse((cost, next_node))));
    }
    min_cost as i32
}

// Kruscalのアルゴリズム
// 閉路ができないように最小のエッジを選んでいく
// 閉路であるか否かは、UnionFind Treeのis_same関数で判断していく

pub struct UnionFind {
    node_to_root: HashMap<i32, i32>,
    root_to_rank: HashMap<i32, usize>,
    root_count: usize,
}

impl UnionFind {
    pub fn new(nodes: Vec<i32>) -> Self {
        let (node_to_root, root_to_rank) = nodes
            .iter()
            .map(|&node| ((node, node), (node, 0)))
            .collect::<(HashMap<_, _>, HashMap<_, _>)>();
        Self {
            node_to_root,
            root_to_rank,
            root_count: nodes.len(),
        }
    }

    pub fn find_root(&mut self, node: i32) -> i32 {
        let mut node = node;
        let mut root = *self.node_to_root.entry(node).or_insert(node);

        while root != node {
            let root_parent = *self.node_to_root.entry(root).or_insert(root);
            *self.node_to_root.entry(node).or_insert(node) = root_parent;
            node = root;
            root = root_parent
        }
        root
    }

    fn find_rank(&mut self, root: i32) -> usize {
        *self.root_to_rank.entry(root).or_default()
    }

    pub fn unite(&mut self, node1: i32, node2: i32) {
        let root1 = self.find_root(node1);
        let root2 = self.find_root(node2);

        if root1 == root2 {
            return;
        }

        let rank1 = self.find_rank(root1);
        let rank2 = self.find_rank(root2);

        match rank1.cmp(&rank2) {
            std::cmp::Ordering::Less => {
                *self.node_to_root.entry(root1).or_insert(root1) = root2;
            }
            std::cmp::Ordering::Equal => {
                *self.node_to_root.entry(root2).or_insert(root2) = root1;
                *self.root_to_rank.entry(root1).or_default() = rank2 + 1;
            }
            std::cmp::Ordering::Greater => {
                *self.node_to_root.entry(root2).or_insert(root2) = root1;
            }
        }
        self.root_count -= 1;
    }

    pub fn is_same_group(&mut self, node1: i32, node2: i32) -> bool {
        self.find_root(node1) == self.find_root(node2)
    }

    pub fn get_root_count(&self) -> usize {
        self.root_count
    }
}

pub fn minimum_cost_2(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut union_find_tree = UnionFind::new(Vec::from_iter(0..n));

    let mut cost_acs_connections = connections.clone();
    cost_acs_connections.sort_by_key(|connection| connection[2]);

    let mut min_cost = 0;
    cost_acs_connections.into_iter().for_each(|connection| {
        let node1 = connection[0];
        let node2 = connection[1];
        let cost = connection[2];
        if !union_find_tree.is_same_group(node1, node2) {
            min_cost += cost;
            union_find_tree.unite(node1, node2);
        }
    });

    if union_find_tree.get_root_count() != 1 {
        return -1;
    }
    min_cost
}
