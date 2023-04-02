use std::hash::Hash;

use crate::builder::builder::StateMachineBuilder;
use crate::builder::BuilderState;
use crate::builder::Result;

/// The subsequent transition builder in the state machine builder's
/// phased build states
///
/// The transition builder allows you to define more than one state transition.
///
/// A transition, _defined by an edge_, is where an event causes one state
/// to transition to another state.
pub trait TransitionsState<TState, TEvent>
    where Self: Sized
{
    /// Associate the next build phase when all transitions are defined
    type BuilderState: BuilderState<TState, TEvent>;


    /// Defines a transition on an event for the current state being defined
    ///
    /// # Arguments
    ///
    /// * `event` - the event that transitions the state machine
    /// * `state` - the state that the state machine transitions to
    ///
    /// # Results
    ///
    /// Returns the [`TransitionsState`] builder phase if there aren't
    /// any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: crate::builder::BuilderError
    fn transition_on(self, event: TEvent, next_state: TState) -> Result<Self, TState, TEvent>;

    /// Defines the final transition on an event definition for the current
    /// state being defined
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
    fn final_transition_on(
        self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::BuilderState, TState, TEvent>;
}

impl<TState, TEvent> TransitionsState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type BuilderState = Self;

    #[inline]
    fn transition_on(
        mut self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self, TState, TEvent> {
        self.add_transition_impl(self.current, event, next_state).map(|_| self)
    }

    #[inline]
    fn final_transition_on(
        mut self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::BuilderState, TState, TEvent> {
        self.add_transition_impl(
            self.current,
            event,
            next_state,
        ).map(|_| self as Self)
    }
}
