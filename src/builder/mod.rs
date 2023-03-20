use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::hash::Hash;

use build_rules::*;
use error::BuilderError;
use result::Result;
use types::NodeType;

use crate::state_machine::{FiniteStateMachine, StateMachine};

pub mod build_rules;
pub mod error;
mod result;
mod types;

pub struct StateMachineBuilder<TState, TEvent> {
    current: TState,
    initial_state: TState,
    machine: StateMachine<TState, TEvent>,
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
            machine: StateMachine::new(initial_state),
        }
    }
}

impl<TState, TEvent> StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash
{
    fn also_end_state_impl(&mut self, state: TState)
    {
        self.machine.end_states.borrow_mut().insert(state);
    }

    fn add_state_impl(
        &mut self,
        state: TState,
        r#type: NodeType,
    ) -> Result<(), TState, TEvent> {
        if state == self.machine.initial_state {
            return Err(BuilderError::StateAlreadyDefined(state));
        }

        if self.machine.states.contains(&state) {
            return Err(BuilderError::StateAlreadyDefined(state));
        }

        self.machine.states.borrow_mut().insert(state);

        if r#type.contains(NodeType::END) {
            self.machine.end_states.borrow_mut().insert(state);
        }

        if r#type.contains(NodeType::START) {
            self.machine.start_states.borrow_mut().insert(state);
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
        let entry = self.machine
            .transitions.entry(state)
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
        self.machine
            .triggers.entry(state)
            .or_insert_with(Vec::new)
            .push(Box::new(trigger));
    }
}
