use std::hash::Hash;

use crate::build_rules::StateBuilder;
use crate::builder::result::Result;
use crate::StateMachineBuilder;

pub trait MultiTransitionBuilder<TState, TEvent>
    where Self: Sized
{
    type StateBuilder: StateBuilder<TState, TEvent>;

    fn transition_on(self, event: TEvent, next_state: TState) -> Result<Self, TState, TEvent>;

    fn final_transition_on(
        self,
        event: TEvent,
        next_state: TState,
    ) -> Result<Self::StateBuilder, TState, TEvent>;
}

impl<TState, TEvent> MultiTransitionBuilder<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type StateBuilder = StateMachineBuilder<TState, TEvent>;

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
    ) -> Result<Self::StateBuilder, TState, TEvent> {
        self.add_transition_impl(
            self.current,
            event,
            next_state,
        ).map(|_| self as StateMachineBuilder<TState, TEvent>)
    }
}
