//! Utility for detecting cycles in a directed graph

use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Directed graph, used for detecting cycles
///
/// _adapted to Rust from [Detect Cycle in a Directed Graph](https://www.geeksforgeeks.org/detect-cycle-in-a-graph)_
pub struct Graph<T> {
    v: usize,
    adj: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
    where T: Copy + Eq + Hash
{
    const EXPECTED_NODE: &'static str = "all values should have an entry";

    /// Creates a new [Graph] from a collection of nodes
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use crate::graph::Graph;
    /// # use Node::{StateA, StateB, StateC};
    ///
    /// let graph = Graph::new(vec![StateA, StateB, StateC]);
    ///
    /// #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    /// enum Node {
    ///     StateA,
    ///     StateB,
    ///     StateC,
    /// }
    /// ```
    pub fn new(nodes: impl IntoIterator<Item=T>) -> Self {
        let adj = nodes.into_iter()
            .map(|node| (node, <HashSet<T>>::new()))
            .collect::<HashMap<_, _>>();

        Self {
            v: adj.len(),
            adj,
        }
    }

    /// Defines an edge, source > destination, of a directed graph
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use crate::graph::Graph;
    /// # use Node::{StateA, StateB, StateC};
    ///
    /// let mut graph = Graph::new(vec![StateA, StateB, StateC]);
    ///
    /// graph.add_edge(StateA, StateB);
    /// graph.add_edge(StateB, StateC);
    /// graph.add_edge(StateC, StateA);
    ///
    /// #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    /// enum Node {
    ///     StateA,
    ///     StateB,
    ///     StateC,
    /// }
    /// ```
    pub fn add_edge(&mut self, src: T, dest: T) {
        let nodes: &mut HashMap<T, HashSet<T>> = self.adj.borrow_mut();
        let edges = nodes.get_mut(&src).expect(Self::EXPECTED_NODE);

        edges.insert(dest);
    }

    // Checks if directed graph has any cycles
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use crate::graph::Graph;
    /// # use Node::{StateA, StateB, StateC};
    ///
    /// let mut graph = Graph::new(vec![StateA, StateB, StateC]);
    ///
    /// graph.add_edge(StateA, StateB);
    /// graph.add_edge(StateB, StateC);
    /// graph.add_edge(StateC, StateA);
    ///
    /// if graph.is_cyclical() {
    ///     println!("Graph has a cycle");
    /// }
    ///
    /// #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    /// enum Node {
    ///     StateA,
    ///     StateB,
    ///     StateC,
    /// }
    /// ```
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

    fn is_cyclical_util(
        &self, node: T,
        visited: &mut HashSet<T>,
        stack: &mut HashSet<T>,
    ) -> bool {
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