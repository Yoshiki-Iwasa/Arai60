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

pub struct UnionFind2 {
    node_to_root: HashMap<(i32, i32), (i32, i32)>,
    node_to_rank: HashMap<(i32, i32), usize>,
}

impl UnionFind2 {
    pub fn new(nodes: Vec<(i32, i32)>) -> Self {
        let (node_to_root, node_to_rank) = nodes
            .into_iter()
            .map(|node| ((node, node), (node, 0)))
            .collect::<(HashMap<_, _>, HashMap<_, _>)>();
        Self {
            node_to_root,
            node_to_rank,
        }
    }
    pub fn find_root(&mut self, node: (i32, i32)) -> (i32, i32) {
        let mut node = node;
        let mut root = *self.node_to_root.entry(node).or_insert(node);

        while node != root {
            let root_parent = *self.node_to_root.entry(root).or_insert(root);
            self.node_to_root.insert(node, root_parent);
            node = root;
            root = root_parent
        }

        root
    }

    pub fn unite(&mut self, node1: (i32, i32), node2: (i32, i32)) {
        let root1 = self.find_root(node1);
        let root2 = self.find_root(node2);

        let rank1 = *self.node_to_rank.entry(root1).or_default();
        let rank2 = *self.node_to_rank.entry(root2).or_default();

        match rank1.cmp(&rank2) {
            std::cmp::Ordering::Less => {
                self.node_to_root
                    .entry(root1)
                    .and_modify(|root| *root = root2);
            }
            std::cmp::Ordering::Equal => {
                self.node_to_root
                    .entry(root2)
                    .and_modify(|root| *root = root1);
                self.node_to_rank
                    .entry(root1)
                    .and_modify(|rank| *rank = rank2 + 1);
            }
            std::cmp::Ordering::Greater => {
                self.node_to_root
                    .entry(root2)
                    .and_modify(|root| *root = root1);
            }
        }
    }

    pub fn is_same_group(&mut self, node1: (i32, i32), node2: (i32, i32)) -> bool {
        let root1 = self.find_root(node1);
        let root2 = self.find_root(node2);

        root1 == root2
    }
}

pub struct Solution;
impl Solution {
    // Kruscal algorithmのほうが向いてるか？
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let points = points
            .into_iter()
            .map(|point| (point[0], point[1]))
            .collect::<Vec<_>>();
        let mut union_find_tree = UnionFind2::new(points.clone());
        //
        let mut connections = Vec::<(i32, (i32, i32), (i32, i32))>::new();
        for i in 0..points.len() {
            for j in 0..i {
                let (xi, yi) = points[i];
                let (xj, yj) = points[j];
                let cost = (xi - xj).abs() + (yi - yj).abs();
                connections.push((cost, points[i], points[j]))
            }
        }

        connections.sort_by_key(|(cost, _, _)| *cost);
        let mut total_cost = 0;
        for conn in connections {
            let (cost, point1, point2) = conn;
            if !union_find_tree.is_same_group(point1, point2) {
                union_find_tree.unite(point1, point2);
                total_cost += cost;
            }
        }
        total_cost
    }

    // Primのアルゴリズム
    pub fn min_cost_connect_points_2(points: Vec<Vec<i32>>) -> i32 {
        let mut graph = HashMap::new();

        for i in 0..points.len() {
            for j in 0..i {
                let (xi, yi) = (points[i][0], points[i][1]);
                let (xj, yj) = (points[j][0], points[j][1]);

                let cost = (xi - xj).abs() + (yi - yj).abs();
                graph
                    .entry((xi, yi))
                    .or_insert(vec![])
                    .push(((xj, yj), cost));
            }
        }

        let mut priority_queue = BinaryHeap::<Reverse<(i32, (i32, i32))>>::new();
        let start = (points[0][0], points[0][1]);
        graph
            .get(&start)
            .unwrap()
            .iter()
            .for_each(|&(next_point, cost)| priority_queue.push(Reverse((cost, next_point))));

        let mut visited = HashSet::new();
        visited.insert(start);
        let mut total_cost = 0;
        while let Some(Reverse((cost, point))) = priority_queue.pop() {
            if visited.contains(&point) {
                continue;
            }
            visited.insert(point);
            total_cost += cost;
            graph
                .get(&point)
                .unwrap()
                .iter()
                .for_each(|&(next_point, cost)| priority_queue.push(Reverse((cost, next_point))));
        }
        total_cost
    }
}
