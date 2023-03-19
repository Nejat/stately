use std::hash::Hash;

use crate::builder::build_rules::{MultiTriggerBuilder, TransitionBuilder, TriggerEndBuilder};
use crate::StateMachineBuilder;

pub trait TriggerBuilder<TState, TEvent> {
    type TriggerBuilder: TriggerEndBuilder<TState, TEvent>;
    type TransitionBuilder: TransitionBuilder<TState, TEvent>;
    type MultiTriggerBuilder: MultiTriggerBuilder<TState, TEvent>;

    #[must_use]
    fn also_end_state(self) -> Self::TriggerBuilder;

    #[must_use]
    fn on_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiTriggerBuilder;

    #[must_use]
    fn no_triggers(self) -> Self::TransitionBuilder;

    #[must_use]
    fn only_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionBuilder;
}

impl<TState, TEvent> TriggerBuilder<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type TriggerBuilder = StateMachineBuilder<TState, TEvent>;
    type TransitionBuilder = StateMachineBuilder<TState, TEvent>;
    type MultiTriggerBuilder = StateMachineBuilder<TState, TEvent>;

    fn also_end_state(mut self) -> Self::TriggerBuilder {
        self.also_end_state_impl(self.current);

        self
    }

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
