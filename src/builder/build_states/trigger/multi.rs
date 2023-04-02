use std::hash::Hash;

use crate::builder::builder::StateMachineBuilder;
use crate::builder::TransitionState;

/// The subsequent trigger builder in the state machine builder's
/// phased build states
///
/// The trigger builder allows you to define more than one trigger and
/// ensures that transitions are defined
///
/// A [`Trigger`] is a callback that is invoked when the state
/// machine transitions to a new state.
///
/// _* the callback provides the event that caused the transition,
/// the state prior to the transition, and the newly transitioned
/// state_
///
/// [`Trigger`]: crate::Trigger
pub trait TriggersState<TState, TEvent> {
    /// Associates the next build phase when the final trigger is defined
    type TransitionState: TransitionState<TState, TEvent>;

    /// Defines a trigger for the current state being defined
    ///
    /// # Arguments
    ///
    /// * _`trigger`_ - the trigger callback
    ///
    /// # Results
    ///
    /// Returns the [`TriggersState`] builder phase
    #[must_use]
    fn trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self;

    /// Defines a trigger and declares there are no other trigger
    /// definitions for the current state being defined
    ///
    /// # Arguments
    ///
    /// * _`trigger`_ - the trigger callback
    ///
    /// # Results
    ///
    /// Returns the [`TransitionState`] builder phase
    #[must_use]
    fn final_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionState;
}

impl<TState, TEvent> TriggersState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type TransitionState = Self;

    #[inline]
    fn trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn final_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionState {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
