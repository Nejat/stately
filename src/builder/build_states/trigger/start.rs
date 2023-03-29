use std::hash::Hash;

use crate::builder::{EndTriggerState, TransitionState, TriggersState};
use crate::StateMachineBuilder;

pub trait StartTriggerState<TState, TEvent> {
    type EndState: EndTriggerState<TState, TEvent>;
    type MultiState: TriggersState<TState, TEvent>;
    type TransitionState: TransitionState<TState, TEvent>;

    #[must_use]
    fn also_end_state(self) -> Self::EndState;

    #[must_use]
    fn trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiState;

    #[must_use]
    fn no_triggers(self) -> Self::TransitionState;

    #[must_use]
    fn only_trigger(self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionState;
}

impl<TState, TEvent> StartTriggerState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type EndState = StateMachineBuilder<TState, TEvent>;
    type MultiState = StateMachineBuilder<TState, TEvent>;
    type TransitionState = StateMachineBuilder<TState, TEvent>;

    fn also_end_state(mut self) -> Self::EndState {
        self.also_end_state_impl(self.current);

        self
    }

    #[inline]
    fn trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::MultiState {
        self.trigger_on_impl(self.current, trigger);

        self
    }

    #[inline]
    fn no_triggers(self) -> Self::TransitionState {
        self
    }

    #[inline]
    fn only_trigger(mut self, trigger: impl Fn(TEvent, TState, TState) + 'static) -> Self::TransitionState {
        self.trigger_on_impl(self.current, trigger);

        self
    }
}
