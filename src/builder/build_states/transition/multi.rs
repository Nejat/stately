use std::hash::Hash;

use crate::builder::builder::StateMachineBuilder;
use crate::builder::BuilderState;
use crate::builder::Result;

pub trait TransitionsState<TState, TEvent>
    where Self: Sized
{
    type BuilderState: BuilderState<TState, TEvent>;

    fn transition_on(self, event: TEvent, next_state: TState) -> Result<Self, TState, TEvent>;

    fn final_transition_on(
        self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::BuilderState, TState, TEvent>;
}

impl<TState, TEvent> TransitionsState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type BuilderState = Self;

    #[inline]
    fn transition_on(
        mut self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self, TState, TEvent> {
        self.add_transition_impl(self.current, event, next_state).map(|_| self)
    }

    #[inline]
    fn final_transition_on(
        mut self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::BuilderState, TState, TEvent> {
        self.add_transition_impl(
            self.current,
            event,
            next_state,
        ).map(|_| self as Self)
    }
}
