use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// https://www.geeksforgeeks.org/detect-cycle-in-a-graph/?ref=lbp

pub struct Graph<T> {
    v: usize,
    adj: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
    where T: Copy + Eq + Hash
{
    const EXPECTED_NODE: &'static str = "all values should have an entry";

    pub fn new(nodes: impl IntoIterator<Item=T>) -> Self {
        let adj = nodes.into_iter()
            .map(|node| (node, <HashSet<T>>::new()))
            .collect::<HashMap<_, _>>();

        Self {
            v: adj.len(),
            adj,
        }
    }

    pub fn add_edge(&mut self, src: T, dest: T) {
        let nodes: &mut HashMap<T, HashSet<T>> = self.adj.borrow_mut();
        let edges = nodes.get_mut(&src).expect(Self::EXPECTED_NODE);

        edges.insert(dest);
    }

    pub fn is_cyclical(&self) -> bool {
        let mut visited = HashSet::with_capacity(self.v);
        let mut stack = HashSet::with_capacity(self.v);

        for node in self.adj.keys().copied() {
            if self.is_cyclical_util(node, &mut visited, &mut stack) {
                return true;
            }
        }

        false
    }

    fn is_cyclical_util(&self, node: T, visited: &mut HashSet<T>, stack: &mut HashSet<T>) -> bool
        where T: Copy
    {
        if stack.contains(&node) {
            return true;
        }

        if visited.contains(&node) {
            return false;
        }

        visited.insert(node);
        stack.insert(node);

        let children = &self.adj.get(&node).expect(Self::EXPECTED_NODE);

        if children.iter().any(|node| self.is_cyclical_util(*node, visited, stack)) {
            return true;
        }

        stack.remove(&node);

        false
    }
}