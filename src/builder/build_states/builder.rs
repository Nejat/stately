use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

use crate::builder::{EndTriggerState, TriggerState};
use crate::builder::builder::{NodeType, StateMachineBuilder};
use crate::builder::BuilderError::ValidationError;
use crate::builder::Result;
use crate::StateMachineDefinition;

/// The final builder in the state machine builder's phased build states
///
/// The final builder lets you to define more states or build
/// the state machine definition
///
/// ### Generic Data Types
///
/// * _`TState`_ - represents the states of a state machine
/// * _`TEvent`_ - represents the transition events of a state machine
pub trait BuilderState<TState, TEvent> {
    /// Associates the next build phase for end states
    type EndState: EndTriggerState<TState, TEvent>;

    /// Associates the next build phase for all other states
    type TriggerState: TriggerState<TState, TEvent>;

    /// Defines an end state
    ///
    /// An end state indicates that a state machine has completed.
    ///
    /// _* a state machine can have multiple end points_
    ///
    /// # Arguments
    ///
    /// * `state` - a new end state
    ///
    /// # Results
    ///
    /// Returns the [`EndTriggerState`] builder phase if there aren't
    /// any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: crate::builder::BuilderError
    fn add_end_state(self, state: TState) -> Result<Self::EndState, TState, TEvent>;

    /// Defines a start state and it's starting transition
    ///
    /// A start state is an initial entry point into the state machine.
    ///
    /// _* a state machine can have multiple entry points_
    ///
    /// # Arguments
    ///
    /// * `event` - the event that starts the state machine
    /// * `state` - the start state that the state machine transitions to
    ///
    /// # Results
    ///
    /// Returns the [`TriggerState`] builder phase if there aren't
    /// any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: crate::builder::BuilderError
    fn add_start_state(
        self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::TriggerState, TState, TEvent>;

    /// Defines a start state, which is also an end state, and it's starting transition
    ///
    /// A start/end state is both an initial entry point into the state machine and an
    /// end state of the state machine.
    ///
    /// _* a state machine can have multiple entry and end points_
    ///
    /// # Arguments
    ///
    /// * `event` - the event that starts and ends the state machine
    /// * `state` - the start/end state that the state machine transitions to
    ///
    /// # Results
    ///
    /// Returns the [`EndTriggerState`] builder phase if there aren't
    /// any validation error
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: crate::builder::BuilderError
    fn add_start_end_state(
        self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::EndState, TState, TEvent>;

    /// Defines a state
    ///
    /// A state is one of the finite number of intermediary states in a
    /// state machine.
    ///
    /// _* a state machine can have multiple intermediary states_
    ///
    /// # Arguments
    ///
    /// * `state` - defined state
    ///
    /// # Results
    ///
    /// Returns the [`TriggerState`] builder phase if there aren't
    /// any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: crate::builder::BuilderError
    fn add_state(self, state: TState) -> Result<Self::TriggerState, TState, TEvent>;

    /// Builds a validated instance of a [`StateMachineDefinition`]
    ///
    /// # Results
    ///
    /// Returns an instance of a [`StateMachineDefinition`] if there
    /// aren't any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: crate::builder::BuilderError
    /// [`StateMachineDefinition`]: StateMachineDefinition
    fn build(self) -> Result<StateMachineDefinition<TState, TEvent>, TState, TEvent>;
}

impl<TState, TEvent> BuilderState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type EndState = Self;
    type TriggerState = Self;

    #[inline]
    fn add_end_state(mut self, state: TState) -> Result<Self::TriggerState, TState, TEvent> {
        self.add_state_impl(state, NodeType::END).map(|_| self)
    }

    #[inline]
    fn add_start_state(
        mut self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::TriggerState, TState, TEvent> {
        self.add_start_state_impl(event, state).map(|_| self)
    }

    fn add_start_end_state(mut self, event: TEvent, state: TState) -> Result<Self::EndState, TState, TEvent> {
        self.add_start_end_state_impl(event, state).map(|_| self)
    }

    #[inline]
    fn add_state(mut self, state: TState) -> Result<Self::TriggerState, TState, TEvent> {
        self.add_state_impl(state, NodeType::STATE).map(|_| self)
    }

    fn build(self) -> Result<StateMachineDefinition<TState, TEvent>, TState, TEvent> {
        let undefined_states = self.transitions.iter()
            .flat_map(|(_, states)| states.values().collect::<Vec<_>>())
            .filter(|state| !self.states.contains(state))
            .copied()
            .collect::<HashSet<_>>().into_iter()
            .collect::<Vec<_>>();

        let unreachable = self.states.iter()
            .filter(|state| self.transitions.iter()
                .all(|(from, transitions)|
                    from == *state || transitions.values().all(|next| { next != *state })
                ))
            .copied()
            .collect::<Vec<_>>();

        if undefined_states.is_empty() && unreachable.is_empty() {
            Ok(StateMachineDefinition {
                end_states: Rc::new(self.end_states),
                initial_state: self.initial_state,
                transitions: Rc::new(self.transitions),
                triggers: Rc::new(self.triggers),
            })
        } else {
            Err(ValidationError {
                undefined_states,
                unreachable,
            })
        }
    }
}
