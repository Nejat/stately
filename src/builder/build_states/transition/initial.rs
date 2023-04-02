use std::hash::Hash;

use crate::builder::{BuilderState, TransitionsState};
use crate::builder::builder::StateMachineBuilder;
use crate::builder::Result;

/// The initial transition builder in the state machine builder's phased
/// build states
///
/// The initial transition builder ensures that you define at least one
/// state transition.
///
/// A transition, _defined by an edge_, is where an event causes one state
/// to transition to another state.
pub trait TransitionState<TState, TEvent> {
    /// Associate the next build phase for states with only one transition
    type BuilderState: BuilderState<TState, TEvent>;

    /// Associate the next build phase for states with more than one transition
    type MultiState: TransitionsState<TState, TEvent>;

    /// Defines a transition on an event for the current state being defined
    ///
    /// # Arguments
    ///
    /// * `event` - the event that transitions the state machine
    /// * `state` - the state that the state machine transitions to
    ///
    /// # Results
    ///
    /// Returns the [`BuilderState`] builder phase if there aren't
    /// any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: crate::builder::BuilderError
    fn transition_on(
        self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::MultiState, TState, TEvent>;

    /// Defines the only transition on an event for the current state
    /// being defined
    ///
    /// # Arguments
    ///
    /// * `event` - the event that transitions the state machine
    /// * `state` - the state that the state machine transitions to
    ///
    /// # Results
    ///
    /// Returns the [`BuilderState`] builder phase if there aren't
    /// any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: crate::builder::BuilderError
    fn only_transition_on(
        self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::BuilderState, TState, TEvent>;
}

impl<TState, TEvent> TransitionState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type BuilderState = Self;
    type MultiState = Self;

    #[inline]
    fn transition_on(
        mut self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::MultiState, TState, TEvent> {
        self.add_transition_impl(self.current, event, state).map(|_| self)
    }

    #[inline]
    fn only_transition_on(
        mut self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::BuilderState, TState, TEvent>
    {
        self.add_transition_impl(self.current, event, state).map(|_| self as Self)
    }
}
