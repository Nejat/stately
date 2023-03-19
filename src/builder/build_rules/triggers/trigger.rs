use std::hash::Hash;

use crate::builder::build_rules::{MultiTriggerBuilder, TransitionBuilder};
use crate::StateMachineBuilder;

pub trait TriggerEndBuilder<TState, TEvent> {
    type TransitionBuilder: TransitionBuilder<TState, TEvent>;
    type MultiTriggerBuilder: MultiTriggerBuilder<TState, TEvent>;

    #[must_use]
    fn on_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiTriggerBuilder;

    #[must_use]
    fn no_triggers(self) -> Self::TransitionBuilder;

    #[must_use]
    fn only_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionBuilder;
}

impl<TState, TEvent> TriggerEndBuilder<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type TransitionBuilder = StateMachineBuilder<TState, TEvent>;
    type MultiTriggerBuilder = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn on_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiTriggerBuilder {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn no_triggers(self) -> Self::TransitionBuilder {
        self
    }

    #[inline]
    fn only_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionBuilder {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
