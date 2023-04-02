use std::hash::Hash;

use crate::builder::builder::StateMachineBuilder;
use crate::builder::BuilderState;

pub trait EndTriggersState<TState, TEvent> {
    type BuilderState: BuilderState<TState, TEvent>;

    #[must_use]
    fn trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self;

    #[must_use]
    fn final_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::BuilderState;
}

impl<TState, TEvent> EndTriggersState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type BuilderState = Self;

    #[inline]
    fn trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn final_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::BuilderState {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
