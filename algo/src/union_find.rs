use std::collections::HashMap;

// HashMapを使った実装
pub struct UnionFind {
    roots: HashMap<i32, i32>,
    ranks: HashMap<i32, i32>,
}

impl UnionFind {
    pub fn new(node: Vec<i32>) -> Self {
        let (roots, ranks) = node
            .iter()
            .map(|&node| ((node, node), (node, 0)))
            .collect::<(HashMap<_, _>, HashMap<_, _>)>();
        Self { roots, ranks }
    }

    pub fn find_root(&mut self, node: i32) -> i32 {
        let mut node = node;
        let mut parent = *self.roots.entry(node).or_insert(node);

        while node != parent {
            let parents_root = *self.roots.entry(parent).or_insert(parent);
            *self.roots.entry(node).or_insert(node) = parents_root;
            node = parent;
            parent = parents_root;
        }
        parent
    }

    pub fn find_root_mut(&mut self, node: i32) -> &mut i32 {
        let mut node = node;
        let mut parent = *self.roots.entry(node).or_insert(node);

        while node != parent {
            let parents_root = *self.roots.entry(parent).or_insert(parent);
            *self.roots.entry(node).or_insert(node) = parents_root;
            node = parent;
            parent = parents_root;
        }
        self.roots.entry(node).or_insert(node)
    }

    pub fn unite(&mut self, node1: i32, node2: i32) {
        let root1 = self.find_root(node1);
        let root2 = self.find_root(node2);

        let rank1 = *self.ranks.entry(root1).or_default();
        let rank2 = *self.ranks.entry(root2).or_default();

        match rank1.cmp(&rank2) {
            std::cmp::Ordering::Less => *self.find_root_mut(root1) = root2,
            std::cmp::Ordering::Equal => {
                *self.find_root_mut(root2) = root1;
                *self.ranks.entry(root1).or_default() = rank2 + 1;
            }
            std::cmp::Ordering::Greater => *self.find_root_mut(root2) = root1,
        }
    }
}
