use std::hash::Hash;

use crate::builder::{BuilderState, EndTriggersState};
use crate::builder::builder::StateMachineBuilder;

pub trait EndTriggerState<TState, TEvent> {
    type BuilderState: BuilderState<TState, TEvent>;
    type MultiState: EndTriggersState<TState, TEvent>;

    #[must_use]
    fn trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiState;

    #[must_use]
    fn no_triggers(self) -> Self::BuilderState;

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
