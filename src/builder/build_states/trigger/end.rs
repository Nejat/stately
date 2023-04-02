use std::hash::Hash;

use crate::builder::{BuilderState, EndTriggersState};
use crate::builder::builder::StateMachineBuilder;

/// The initial end trigger builder in the state machine builder's phased
/// build states
///
/// The initial end trigger builder allows you to optionally define
/// one or more triggers and ensures that no transitions are defined
///
/// A [`Trigger`] is a callback that is invoked when the state
/// machine transitions to a new state.
///
/// _* the callback provides the event that caused the transition,
/// the state prior to the transition, and the newly transitioned
/// state_
///
/// [`Trigger`]: crate::Trigger
pub trait EndTriggerState<TState, TEvent> {
    /// Associates the next build phase when no more than one
    /// trigger is defined
    type BuilderState: BuilderState<TState, TEvent>;

    /// Associates the next build phase when more than one
    /// trigger is defined
    type MultiState: EndTriggersState<TState, TEvent>;

    /// Defines a trigger for the current state being defined
    ///
    /// # Arguments
    ///
    /// * _`trigger`_ - the trigger callback
    ///
    /// # Results
    ///
    /// Returns the [`EndTriggersState`] builder phase
    #[must_use]
    fn trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiState;

    /// Declares there are no triggers for the current state being defined
    ///
    /// # Results
    ///
    /// Returns the [`BuilderState`] builder phase
    #[must_use]
    fn no_triggers(self) -> Self::BuilderState;

    /// Defines a trigger and declares there are no other trigger
    /// definitions for the current state being defined
    ///
    /// # Arguments
    ///
    /// * _`trigger`_ - the trigger callback
    ///
    /// # Results
    ///
    /// Returns the [`BuilderState`] builder phase
    #[must_use]
    fn only_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::BuilderState;
}

impl<TState, TEvent> EndTriggerState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type BuilderState = Self;
    type MultiState = Self;

    #[inline]
    fn trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiState {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn no_triggers(self) -> Self::BuilderState {
        self
    }

    #[inline]
    fn only_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::BuilderState {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
