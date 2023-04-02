use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bitflags::bitflags;

use crate::{builder, Trigger};
use crate::builder::BuilderError::{
    RedefinedInitialState, StateAlreadyDefined, TransitionAlreadyDefined,
};
use crate::builder::InitialState;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct NodeType: u8 {
        const END   = 0b10;
        const START = 0b01;
        const STATE = 0b00;
    }
}

pub struct StateMachineBuilder<TState, TEvent> {
    pub(crate) current: TState,
    pub(crate) initial_state: TState,
    pub(crate) end_states: HashSet<TState>,
    pub(crate) start_states: HashSet<TState>,
    pub(crate) states: HashSet<TState>,
    pub(crate) transitions: HashMap<TState, HashMap<TEvent, TState>>,
    pub(crate) triggers: HashMap<TState, Vec<Trigger<TState, TEvent>>>,
}

impl<TState, TEvent> StateMachineBuilder<TState, TEvent>
    where TState: Copy + Default + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    #[allow(clippy::new_ret_no_self)]
    #[must_use]
    pub fn new() -> impl InitialState<TState, TEvent> {
        let initial_state = TState::default();

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
    pub(crate) fn add_state_impl(
        &mut self,
        state: TState,
        node_type: NodeType,
    ) -> builder::Result<(), TState, TEvent> {
        // check if redefining `initial` state
        if state == self.initial_state {
            return Err(RedefinedInitialState);
        }

        // check if redefining a state that was already defined
        if self.states.contains(&state) {
            return Err(StateAlreadyDefined { state });
        }

        // add state to tracker
        self.states.borrow_mut().insert(state);

        // if the state is an end state, define it as such
        if node_type.contains(NodeType::END) {
            self.end_states.borrow_mut().insert(state);
        }

        // if the state is a start state, define it as such
        if node_type.contains(NodeType::START) {
            self.start_states.borrow_mut().insert(state);
        }

        // record the new state as the current state
        self.current = state;

        Ok(())
    }

    #[inline]
    pub(crate) fn add_start_end_state_impl(
        &mut self,
        event: TEvent,
        state: TState,
    ) -> builder::Result<(), TState, TEvent> {
        self.add_state_impl(state, NodeType::START | NodeType::END)?;
        self.add_transition_impl(self.initial_state, event, state)
    }

    #[inline]
    pub(crate) fn add_start_state_impl(
        &mut self,
        event: TEvent,
        state: TState,
    ) -> builder::Result<(), TState, TEvent> {
        self.add_state_impl(state, NodeType::START)?;
        self.add_transition_impl(self.initial_state, event, state)
    }

    pub(crate) fn add_transition_impl(
        &mut self,
        state: TState,
        event: TEvent,
        next: TState,
    ) -> builder::Result<(), TState, TEvent> {
        let entry = self.transitions.entry(state)
            .or_insert_with(HashMap::new);

        if entry.contains_key(&event) {
            let existing = entry[&event];

            return Err(TransitionAlreadyDefined { event, existing });
        }

        entry.borrow_mut().insert(event, next);

        Ok(())
    }

    pub(crate) fn trigger_on_impl(
        &mut self,
        state: TState,
        trigger: impl Fn(TEvent, TState, TState) + 'static,
    ) {
        self.triggers.entry(state)
            .or_insert_with(Vec::new)
            .push(Box::new(trigger));
    }
}
