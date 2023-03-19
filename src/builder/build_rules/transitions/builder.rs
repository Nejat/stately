use std::hash::Hash;

use crate::build_rules::StateBuilder;
use crate::builder::build_rules::MultiTransitionBuilder;
use crate::builder::result::Result;
use crate::StateMachineBuilder;

pub trait TransitionBuilder<TState, TEvent>
    where Self: Sized
{
    type MultiTransitionBuilder: MultiTransitionBuilder<TState, TEvent>;
    type StateBuilder: StateBuilder<TState, TEvent>;

    fn transition_on(
        self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::MultiTransitionBuilder, TState, TEvent>;

    fn no_transitions(self) -> Self::StateBuilder;

    fn only_transition_on(
        self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::StateBuilder, TState, TEvent>;
}

impl<TState, TEvent> TransitionBuilder<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type MultiTransitionBuilder = StateMachineBuilder<TState, TEvent>;
    type StateBuilder = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn transition_on(
        mut self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::MultiTransitionBuilder, TState, TEvent> {
        self.add_transition_impl(self.current, event, next_state).map(|_| self)
    }

    #[inline]
    fn no_transitions(self) -> Self::StateBuilder {
        self as StateMachineBuilder<TState, TEvent>
    }

    #[inline]
    fn only_transition_on(
        mut self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::StateBuilder, TState, TEvent>
    {
        self.add_transition_impl(
            self.current,
            event,
            next_state,
        ).map(|_| self as StateMachineBuilder<TState, TEvent>)
    }
}
