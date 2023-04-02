//! Utility for detecting cycles in a directed graph

use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Directed graph, used for detecting cycles
///
/// _adapted to Rust from [Detect Cycle in a Directed Graph]_
///
/// [Detect Cycle in a Directed Graph]: https://www.geeksforgeeks.org/detect-cycle-in-a-graph
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
    /// # Arguments
    ///
    /// * _`nodes`_ - an iterator of all the nodes of the graph
    ///
    /// # Results
    ///
    /// Returns an initialized instance of a `Graph`
    ///
    /// # Example
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
    /// # Arguments
    ///
    /// * _`src`_ - source node
    /// * _`dest`_ - destination node
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use crate::graph::Graph;
    /// # use Node::{StateA, StateB, StateC};
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
    /// # Results
    ///
    /// Returns a `true` if the graph has any cycles defined,
    /// a `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use crate::graph::Graph;
    /// # use Node::{StateA, StateB, StateC};
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

    /// Recursively searches for cycles in the directed graph
    ///
    /// # Arguments
    ///
    /// * _`visited`_ - a mutable reference to a history of visited nodes
    /// * _`stack`_ - a mutable reference to a collection of nodes that can
    /// indicate a cycle in the current searched branch
    ///
    /// # Results
    ///
    /// Returns a `true` if a cycle is detected, `false` otherwise
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