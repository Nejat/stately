use std::hash::Hash;

use crate::builder::{EndTriggerState, Result, TriggerState};
use crate::StateMachineBuilder;

pub trait InitialState<TState, TEvent> {
    type TriggerState: TriggerState<TState, TEvent>;
    type EndState: EndTriggerState<TState, TEvent>;

    fn add_start_state(
        self,
        start_event: TEvent,
        start_state: TState,
    ) -> Result<Self::TriggerState, TState, TEvent>;

    fn add_start_end_state(
        self,
        start_event: TEvent,
        start_state: TState,
    ) -> Result<Self::EndState, TState, TEvent>;
}

impl<TState, TEvent> InitialState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type TriggerState = StateMachineBuilder<TState, TEvent>;
    type EndState = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn add_start_state(
        mut self,
        start_event: TEvent,
        start_state: TState,
    ) -> Result<Self::TriggerState, TState, TEvent> {
        self.add_start_state_impl(self.initial_state, start_event, start_state).map(|_| self)
    }

    fn add_start_end_state(mut self, start_event: TEvent, start_state: TState) -> Result<Self::EndState, TState, TEvent> {
        self.add_start_end_state_impl(self.initial_state, start_event, start_state).map(|_| self)
    }
}
