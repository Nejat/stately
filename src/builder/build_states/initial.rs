use std::hash::Hash;

use crate::builder::{EndTriggerState, Result, TriggerState};
use crate::builder::builder::StateMachineBuilder;

/// The initial builder in the state machine builder's phased build states
///
/// The initial builder ensures that at least one start state is defined
/// and triggers are configured
///
/// ### Generic Data Types
///
/// * _`TState`_ - represents the states of a state machine
/// * _`TEvent`_ - represents the transition events of a state machine
pub trait InitialState<TState, TEvent> {
    /// Associates the next build phase for end states
    type EndState: EndTriggerState<TState, TEvent>;

    /// Associates the next build phase for start states
    type TriggerState: TriggerState<TState, TEvent>;

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
    /// any validation errors
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
}

impl<TState, TEvent> InitialState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type EndState = Self;
    type TriggerState = Self;

    #[inline]
    fn add_start_state(
        mut self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::TriggerState, TState, TEvent> {
        self.add_start_state_impl(event, state).map(|_| self)
    }

    #[inline]
    fn add_start_end_state(
        mut self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::EndState, TState, TEvent> {
        self.add_start_end_state_impl(event, state).map(|_| self)
    }
}
