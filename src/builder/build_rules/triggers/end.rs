use std::hash::Hash;

use crate::build_rules::StateBuilder;
use crate::build_rules::triggers::end_multi::MultiTriggerEndBuilder;
use crate::StateMachineBuilder;

pub trait TriggerEndBuilder<TState, TEvent> {
    type StateBuilder: StateBuilder<TState, TEvent>;
    type MultiBuilder: MultiTriggerEndBuilder<TState, TEvent>;

    #[must_use]
    fn on_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiBuilder;

    #[must_use]
    fn no_triggers(self) -> Self::StateBuilder;

    #[must_use]
    fn only_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::StateBuilder;
}

impl<TState, TEvent> TriggerEndBuilder<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type StateBuilder = StateMachineBuilder<TState, TEvent>;
    type MultiBuilder = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn on_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiBuilder {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn no_triggers(self) -> Self::StateBuilder {
        self
    }

    #[inline]
    fn only_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::StateBuilder {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
