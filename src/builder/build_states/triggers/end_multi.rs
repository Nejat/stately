use std::hash::Hash;

use crate::builder::BuilderState;
use crate::StateMachineBuilder;

pub trait EndTriggersState<TState, TEvent>
    where Self: Sized
{
    type BuildState: BuilderState<TState, TEvent>;

    #[must_use]
    fn trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self;

    #[must_use]
    fn final_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::BuildState;
}

impl<TState, TEvent> EndTriggersState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type BuildState = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn final_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::BuildState {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
