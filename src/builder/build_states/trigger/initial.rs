use std::hash::Hash;

use crate::builder::{TransitionState, TriggersState};
use crate::builder::builder::StateMachineBuilder;

/// The initial trigger builder in the state machine builder's phased
/// build states
///
/// The initial trigger builder allows you to optionally define
/// one or more triggers and ensures that transitions are defined
///
/// A [`Trigger`] is a callback that is invoked when the state
/// machine transitions to a new state.
///
/// _* the callback provides the event that caused the transition,
/// the state prior to the transition, and the newly transitioned
/// state_
///
/// [`Trigger`]: crate::Trigger
pub trait TriggerState<TState, TEvent> {
    /// Associate the next build phase for states with more
    /// than one trigger
    type MultiState: TriggersState<TState, TEvent>;

    /// Associates the next build phase when no more than one
    /// trigger is defined
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
    fn trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiState;

    /// Declares there are no triggers for the current state being defined
    ///
    /// # Results
    ///
    /// Returns the [`TransitionState`] builder phase
    #[must_use]
    fn no_triggers(self) -> Self::TransitionState;

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
    fn only_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionState;
}

impl<TState, TEvent> TriggerState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type MultiState = Self;
    type TransitionState = Self;

    #[inline]
    fn trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiState {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn no_triggers(self) -> Self::TransitionState {
        self
    }

    #[inline]
    fn only_trigger(
        mut self,
        trigger: impl Fn(TEvent, TState, TState) + 'static,
    ) -> Self::TransitionState {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
