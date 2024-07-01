// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  -

  何を考えて解いていたか
  - もろUnionFindのケースな気がする
  - 下処理してgraphに直してからDFSでもいいかもしれない
  - 0..n をnodeそのものとしてあ

  想定ユースケース
  - おもいつかず...

  正解してから気づいたこと
  - graphにはメモリをケチって単独の島を入れないようにしているが、いれてあげた方が可読性があがるかも
*/

pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = HashMap::<usize, Vec<usize>>::new();
        let mut seen = vec![false; n as usize];

        for row in &edges {
            graph
                .entry(row[0] as usize)
                .or_insert(vec![])
                .push(row[1] as usize);
            graph
                .entry(row[1] as usize)
                .or_insert(vec![])
                .push(row[0] as usize);
        }

        let mut components_count = 0;

        for node in 0..n as usize {
            if !seen[node] {
                components_count += 1;
                Self::visit_connected_node(&graph, &mut seen, node);
            }
        }

        components_count
    }

    fn visit_connected_node(graph: &HashMap<usize, Vec<usize>>, seen: &mut [bool], node: usize) {
        if seen[node] {
            return;
        }

        seen[node] = true;

        if let Some(next_nodes) = graph.get(&node) {
            for next in next_nodes {
                Self::visit_connected_node(graph, seen, *next)
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

        for nodes in edges {
            union_find.unite(nodes[0] as usize, nodes[1] as usize);
        }

        union_find.get_root_count() as i32
    }
}
