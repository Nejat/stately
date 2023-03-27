use std::hash::Hash;

use crate::builder::build_states::TransitionsState;
use crate::builder::BuilderState;
use crate::builder::Result;
use crate::StateMachineBuilder;

pub trait TransitionState<TState, TEvent>
    where Self: Sized
{
    type MultiState: TransitionsState<TState, TEvent>;
    type BuildState: BuilderState<TState, TEvent>;

    fn transition_on(
        self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::MultiState, TState, TEvent>;

    fn only_transition_on(
        self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::BuildState, TState, TEvent>;
}

impl<TState, TEvent> TransitionState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type MultiState = StateMachineBuilder<TState, TEvent>;
    type BuildState = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn transition_on(
        mut self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::MultiState, TState, TEvent> {
        self.add_transition_impl(self.current, event, next_state).map(|_| self)
    }

    #[inline]
    fn only_transition_on(
        mut self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::BuildState, TState, TEvent>
    {
        self.add_transition_impl(self.current, event, next_state)
            .map(|_| self as StateMachineBuilder<TState, TEvent>)
    }
}
