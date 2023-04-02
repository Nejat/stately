use std::hash::Hash;

use crate::builder::{EndTriggerState, Result, TriggerState};
use crate::builder::builder::StateMachineBuilder;

pub trait InitialState<TState, TEvent> {
    type EndState: EndTriggerState<TState, TEvent>;

    type TriggerState: TriggerState<TState, TEvent>;

    fn add_start_state(
        self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::TriggerState, TState, TEvent>;

    fn add_start_end_state(
        self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::EndState, TState, TEvent>;
}

impl<TState, TEvent> InitialState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type EndState = Self;
    type TriggerState = Self;

    #[inline]
    fn add_start_state(
        mut self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::TriggerState, TState, TEvent> {
        self.add_start_state_impl(event, state).map(|_| self)
    }

    #[inline]
    fn add_start_end_state(
        mut self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::EndState, TState, TEvent> {
        self.add_start_end_state_impl(event, state).map(|_| self)
    }
}
