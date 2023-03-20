use std::hash::Hash;

use crate::builder::*;
use crate::StateMachineBuilder;

pub trait MultiTriggerEndBuilder<TState, TEvent>
    where Self: Sized
{
    type StateBuilder: StateBuilder<TState, TEvent>;

    #[must_use]
    fn on_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self;

    #[must_use]
    fn final_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::StateBuilder;
}

impl<TState, TEvent> MultiTriggerEndBuilder<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type StateBuilder = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn on_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn final_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::StateBuilder {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
