use std::hash::Hash;

use crate::builder::build_rules::{MultiTriggerBuilder, TransitionBuilder, TriggerEndBuilder};
use crate::StateMachineBuilder;

pub trait TriggerBuilder<TState, TEvent> {
    type EndBuilder: TriggerEndBuilder<TState, TEvent>;
    type MultiBuilder: MultiTriggerBuilder<TState, TEvent>;
    type TransitionBuilder: TransitionBuilder<TState, TEvent>;

    #[must_use]
    fn also_end_state(self) -> Self::EndBuilder;

    #[must_use]
    fn trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiBuilder;

    #[must_use]
    fn no_triggers(self) -> Self::TransitionBuilder;

    #[must_use]
    fn only_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionBuilder;
}

impl<TState, TEvent> TriggerBuilder<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type EndBuilder = StateMachineBuilder<TState, TEvent>;
    type MultiBuilder = StateMachineBuilder<TState, TEvent>;
    type TransitionBuilder = StateMachineBuilder<TState, TEvent>;

    fn also_end_state(mut self) -> Self::EndBuilder {
        self.also_end_state_impl(self.current);

        self
    }

    #[inline]
    fn trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiBuilder {
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
