use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

use crate::builder::*;
use crate::StateMachineBuilder;

pub trait StateBuilder<TState, TEvent>
    where Self: Sized
{
    type EndBuilder: TriggerEndBuilder<TState, TEvent>;
    type TriggerBuilder: TriggerBuilder<TState, TEvent>;

    fn add_end_state(self, end_state: TState) -> Result<Self::EndBuilder, TState, TEvent>;

    fn add_start_state(
        self,
        start_start_event: TEvent,
        start_state: TState,
    ) -> Result<Self::TriggerBuilder, TState, TEvent>;

    fn add_state(self, state: TState) -> Result<Self::TriggerBuilder, TState, TEvent>;

    fn build(self) -> Result<StateMachineDefinition<TState, TEvent>, TState, TEvent>;
}

impl<TState, TEvent> StateBuilder<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type EndBuilder = StateMachineBuilder<TState, TEvent>;
    type TriggerBuilder = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn add_end_state(mut self, end_state: TState) -> Result<Self::TriggerBuilder, TState, TEvent> {
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

    #[inline]
    fn build(self) -> Result<StateMachineDefinition<TState, TEvent>, TState, TEvent> {
        let danglers = self            .states.iter()
            .filter(|state|
                !self.transitions.contains_key(state) &&
                    !self.end_states.contains(state)
            )
            .copied()
            .collect::<HashSet<_>>().into_iter()
            .collect::<Vec<_>>();

        let no_end_states = self.end_states.is_empty();

        let undefined_states = self            .transitions.iter()
            .flat_map(|(_, itms)| itms.values().collect::<Vec<_>>())
            .filter(|state| !self.states.contains(state)).copied()
            .collect::<HashSet<_>>().into_iter()
            .collect::<Vec<_>>();

        if danglers.is_empty() && undefined_states.is_empty() {
            Ok(StateMachineDefinition {
                end_states: Rc::new(self.end_states),
                initial_state: self.initial_state,
                start_states: Rc::new(self.start_states),
                states: Rc::new(self.states),
                transitions: Rc::new(self.transitions),
                triggers: Rc::new(self.triggers),
            })
        } else {
            Err(BuilderError::ValidationError {
                no_end_states,
                danglers,
                undefined_states,
            })
        }
    }
}
