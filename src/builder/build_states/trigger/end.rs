use std::hash::Hash;

use crate::builder::{BuilderState, EndTriggersState};
use crate::StateMachineBuilder;

pub trait EndTriggerState<TState, TEvent> {
    type BuildState: BuilderState<TState, TEvent>;
    type MultiState: EndTriggersState<TState, TEvent>;

    #[must_use]
    fn trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiState;

    #[must_use]
    fn no_triggers(self) -> Self::BuildState;

    #[must_use]
    fn only_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::BuildState;
}

impl<TState, TEvent> EndTriggerState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type BuildState = StateMachineBuilder<TState, TEvent>;
    type MultiState = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiState {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn no_triggers(self) -> Self::BuildState {
        self
    }

    #[inline]
    fn only_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::BuildState {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
