use std::hash::Hash;

use crate::builder::{TransitionState, TriggersState};
use crate::builder::builder::StateMachineBuilder;

pub trait TriggerState<TState, TEvent> {
    type MultiState: TriggersState<TState, TEvent>;

    type TransitionState: TransitionState<TState, TEvent>;

    #[must_use]
    fn trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiState;

    #[must_use]
    fn no_triggers(self) -> Self::TransitionState;

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
