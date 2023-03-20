use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bitflags::bitflags;

use build_rules::*;
use error::BuilderError;
use result::Result;

use crate::state_machine::StateMachineDefinition;
use crate::Triggers;

pub mod build_rules;
pub mod error;
mod result;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct NodeType: u8 {
        const END   = 0b10;
        const START = 0b01;
        const STATE = 0b00;
    }
}
pub struct StateMachineBuilder<TState, TEvent> {
    current: TState,
    initial_state: TState,
    end_states: HashSet<TState>,
    start_states: HashSet<TState>,
    states: HashSet<TState>,
    transitions: HashMap<TState, HashMap<TEvent, TState>>,
    triggers: HashMap<TState, Vec<Triggers<TState, TEvent>>>,
}

impl<TState, TEvent> StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    #[allow(clippy::new_ret_no_self)]
    pub fn new(initial_state: TState) -> impl InitialBuilder<TState, TEvent>
    {
        Self {
            current: initial_state,
            initial_state,
            end_states: HashSet::default(),
            start_states: HashSet::default(),
            states: HashSet::default(),
            transitions: HashMap::default(),
            triggers: HashMap::default(),
        }
    }
}

impl<TState, TEvent> StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash
{
    fn also_end_state_impl(&mut self, state: TState)
    {
        self.end_states.borrow_mut().insert(state);
    }

    fn add_state_impl(
        &mut self,
        state: TState,
        r#type: NodeType,
    ) -> Result<(), TState, TEvent> {
        if state == self.initial_state {
            return Err(BuilderError::StateAlreadyDefined(state));
        }

        if self.states.contains(&state) {
            return Err(BuilderError::StateAlreadyDefined(state));
        }

        self.states.borrow_mut().insert(state);

        if r#type.contains(NodeType::END) {
            self.end_states.borrow_mut().insert(state);
        }

        if r#type.contains(NodeType::START) {
            self.start_states.borrow_mut().insert(state);
        }

        self.current = state;

        Ok(())
    }

    fn add_start_state_impl(
        &mut self,
        initial_state: TState,
        start_start_event: TEvent,
        start_state: TState,
    ) -> Result<(), TState, TEvent> {
        self.add_state_impl(start_state, NodeType::START)?;

        self.current = start_state;

        self.add_transition_impl(initial_state, start_start_event, start_state)
    }

    fn add_transition_impl(
        &mut self,
        state: TState,
        event: TEvent,
        next_state: TState,
    ) -> Result<(), TState, TEvent> {
        let entry = self.transitions.entry(state)
            .or_insert_with(HashMap::new);

        if entry.contains_key(&event) {
            let existing = entry[&event];

            return Err(BuilderError::TransitionAlreadyDefined {
                event,
                existing,
            });
        }

        entry.borrow_mut().insert(event, next_state);

        Ok(())
    }

    fn trigger_on_impl(&mut self, state: TState, trigger: impl Fn(TEvent, TState, TState) + 'static) {
        self.triggers.entry(state)
            .or_insert_with(Vec::new)
            .push(Box::new(trigger));
    }
}
