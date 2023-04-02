use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::graph::Graph;
use crate::state_machine::StateMachineDefinition;

/// Detects if an instance of a [`StateMachineDefinition`] has any [cycles] defined
///
/// ### Arguments
///
/// * _fsm_ - a reference to an instance of a [`StateMachineDefinition`]
///
/// ### Returns
///
/// Returns `true` if a cycle was detected, `false` otherwise
///
/// ### Example
///
///  <img src="https://raw.githubusercontent.com/Nejat/stately/master/design/diagrams/turnstile.svg" alt="turnstile states" width="200"
///        style="background: transparent; position: absolute; left: 650px; margin-top: 250px; z-index: 10000;"/>
///
/// ```rust
/// use std::default::Default;
///
/// use stately::detect_cycles;
/// use stately::prelude::*;
///
/// fn main() {
///     let turnstile_fsm = turnstile_fsm();
///
///     assert!(detect_cycles(&turnstile_fsm));
/// }
///
/// // https://en.wikipedia.org/wiki/Finite-state_machine
/// fn turnstile_fsm() -> StateMachineDefinition<State, Event> {
///    // ...
///    # use Event::{Coin, Push, On};
///    # use State::{Locked, Unlocked};
///    # StateMachineBuilder::new()
///    #    .add_start_state(On, Locked).unwrap()
///    #        .no_triggers()
///    #        .transition_on(Push, Locked).unwrap()
///    #        .final_transition_on(Coin, Unlocked).unwrap()
///    #    .add_state(Unlocked).unwrap()
///    #        .no_triggers()
///    #        .transition_on(Coin, Unlocked).unwrap()
///    #        .final_transition_on(Push, Locked).unwrap()
///    #    .build().unwrap()
/// }
///
/// #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
/// enum State {
///     #[default] Initial, Locked, Unlocked,
/// }
///
/// #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// enum Event {
///     On, Push, Coin,
/// }
/// ```
///
/// _you can also use the_ [`FiniteStateMachine::has_cycles`] _method to determine
/// if it has any cycles defined_
///
/// [cycles]: https://en.wikipedia.org/wiki/Cycle_(graph_theory)
/// [`FiniteStateMachine::has_cycles`]: crate::FiniteStateMachine::has_cycles
#[inline]
pub fn detect_cycles<TState, TEvent>(fsm: &StateMachineDefinition<TState, TEvent>) -> bool
    where TState: Copy + Eq + Hash
{
    detect_cycles_impl(&fsm.transitions, &fsm.end_states)
}

/// Internal implementation of [`detect_cycles`]
///
/// todo - make generic over iterators
///
/// ### Arguments
///
/// * _`transitions`_ - a reference to a collections of transitions for states
/// * _`end_states`_ - a reference to a collection of end states
///
/// ### Returns
///
/// Returns `true` if a cycle was detected, `false` otherwise
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
