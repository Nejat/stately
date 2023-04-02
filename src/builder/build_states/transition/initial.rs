use std::hash::Hash;

use crate::builder::{BuilderState, TransitionsState};
use crate::builder::builder::StateMachineBuilder;
use crate::builder::Result;

pub trait TransitionState<TState, TEvent> {
    type BuilderState: BuilderState<TState, TEvent>;

    type MultiState: TransitionsState<TState, TEvent>;

    fn transition_on(
        self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::MultiState, TState, TEvent>;

    fn only_transition_on(
        self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::BuilderState, TState, TEvent>;
}

impl<TState, TEvent> TransitionState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type BuilderState = Self;
    type MultiState = Self;

    #[inline]
    fn transition_on(
        mut self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::MultiState, TState, TEvent> {
        self.add_transition_impl(self.current, event, state).map(|_| self)
    }

    #[inline]
    fn only_transition_on(
        mut self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::BuilderState, TState, TEvent>
    {
        self.add_transition_impl(self.current, event, state).map(|_| self as Self)
    }
}
