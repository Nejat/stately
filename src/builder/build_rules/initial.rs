use std::hash::Hash;

use crate::builder::*;
use crate::builder::types::*;
use crate::StateMachineBuilder;

pub trait InitialBuilder<TState, TEvent>
    where Self: Sized
{
    type TriggerBuilder: TriggerBuilder<TState, TEvent>;

    fn add_end_state(self, end_state: TState) -> Result<Self, TState, TEvent>;

    fn add_start_state(
        self,
        start_start_event: TEvent,
        start_state: TState,
    ) -> Result<Self::TriggerBuilder, TState, TEvent>;

    fn add_state(self, state: TState) -> Result<Self::TriggerBuilder, TState, TEvent>;
}

impl<TState, TEvent> InitialBuilder<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type TriggerBuilder = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn add_end_state(mut self, end_state: TState) -> Result<Self, TState, TEvent> {
        self.add_state_impl(end_state, NodeType::END).map(|_| self)
    }

    #[inline]
    fn add_start_state(
        mut self,
        start_start_event: TEvent,
        start_state: TState,
    ) -> Result<Self::TriggerBuilder, TState, TEvent> {
        self.add_start_state_impl(self.initial_state, start_start_event, start_state).map(|_| self)
    }

    #[inline]
    fn add_state(mut self, state: TState) -> Result<Self::TriggerBuilder, TState, TEvent> {
        self.add_state_impl(state, NodeType::STATE).map(|_| self)
    }
}
