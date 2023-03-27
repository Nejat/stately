use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::graph::Graph;
use crate::state_machine::StateMachineDefinition;

#[inline]
pub fn detect_cycles<TState, TEvent>(fsm: &StateMachineDefinition<TState, TEvent>) -> bool
    where TState: Copy + Eq + Hash
{
    detect_cycles_impl(&fsm.transitions, &fsm.end_states)
}

pub fn detect_cycles_impl<TState, TEvent>(
    transitions: &HashMap<TState, HashMap<TEvent, TState>>,
    end_states: &HashSet<TState>,
) -> bool
    where TState: Copy + Eq + Hash
{
    let mut graph = <Graph<TState>>::new(transitions.keys().chain(end_states.iter()).copied());

    for node in transitions.keys().copied() {
        let edges = transitions.get(&node).expect("every node requires a transition");

        for (_, edge) in edges.iter() {
            graph.add_edge(node, *edge);
        }
    }

    graph.is_cyclical()
}
