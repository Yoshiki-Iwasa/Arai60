// 問題
// https://leetcode.com/problems/course-schedule/description/?envType=problem-list-v2&envId=topological-sort
// https://leetcode.com/problems/course-schedule-ii/?envType=problem-list-v2&envId=topological-sort

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    // Topological sort by bfs
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = HashMap::<i32, (usize, Vec<i32>)>::with_capacity(num_courses as usize);

        (0..num_courses).for_each(|n| {
            graph.insert(n, (0, vec![]));
        });

        prerequisites.into_iter().for_each(|prerequisite| {
            let (_, dest_nodes) = graph.entry(prerequisite[0]).or_insert((0, vec![]));
            dest_nodes.push(prerequisite[1]);

            let (in_degree, _) = graph.entry(prerequisite[1]).or_insert((0, vec![]));
            *in_degree += 1;
        });

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        graph.iter().for_each(|(node, (in_degree, _))| {
            if *in_degree == 0 {
                queue.push_back(*node)
            }
        });

        while let Some(node) = queue.pop_front() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);

            let Some((_, dest_nodes)) = graph.get(&node).cloned() else {
                continue;
            };
            dest_nodes.iter().for_each(|dest_node| {
                let Some((in_degree, _)) = graph.get_mut(dest_node) else {
                    return;
                };
                *in_degree -= 1;
                if *in_degree == 0 {
                    queue.push_back(*dest_node)
                }
            });
        }
        visited.len() == graph.len()
    }

    // return topological order
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = HashMap::<i32, (usize, Vec<i32>)>::with_capacity(num_courses as usize);

        (0..num_courses).for_each(|n| {
            graph.insert(n, (0, vec![]));
        });

        prerequisites.into_iter().for_each(|prerequisite| {
            let (_, dest_nodes) = graph.entry(prerequisite[0]).or_insert((0, vec![]));
            dest_nodes.push(prerequisite[1]);
            let (in_degree, _) = graph.entry(prerequisite[1]).or_insert((0, vec![]));
            *in_degree += 1;
        });

        let mut queue = VecDeque::new();
        graph.iter().for_each(|(node, (in_degree, _))| {
            if *in_degree == 0 {
                queue.push_back(*node);
            }
        });

        let mut result = vec![];
        let mut visited = HashSet::new();
        while let Some(node) = queue.pop_front() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            result.push(node);
            let (_, dest_nodes) = graph.get(&node).unwrap_or(&(0, vec![])).clone();
            for dest_node in dest_nodes {
                let (in_degree, _) = graph.get_mut(&dest_node).unwrap();
                *in_degree -= 1;
                if *in_degree == 0 {
                    queue.push_back(dest_node);
                }
            }
        }

        match result.len() == graph.len() {
            true => {
                result.reverse();
                result
            }
            false => vec![],
        }
    }
}
