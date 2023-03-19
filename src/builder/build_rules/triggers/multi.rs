use std::hash::Hash;

use crate::builder::*;
use crate::StateMachineBuilder;

pub trait MultiTriggerBuilder<TState, TEvent>
    where Self: Sized
{
    type TransitionBuilder: TransitionBuilder<TState, TEvent>;

    #[must_use]
    fn on_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self;

    #[must_use]
    fn final_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionBuilder;
}

impl<TState, TEvent> MultiTriggerBuilder<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type TransitionBuilder = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn on_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn final_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionBuilder {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
