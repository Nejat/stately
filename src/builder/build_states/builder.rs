use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

use crate::builder::{EndTriggerState, NodeType, StateMachineDefinition, TriggerState};
use crate::builder::BuilderError::ValidationError;
use crate::builder::Result;
use crate::StateMachineBuilder;

pub trait BuilderState<TState, TEvent>
    where Self: Sized
{
    type EndState: EndTriggerState<TState, TEvent>;
    type StartState: TriggerState<TState, TEvent>;

    fn add_end_state(self, end_state: TState) -> Result<Self::EndState, TState, TEvent>;

    fn add_start_state(
        self,
        start_event: TEvent,
        start_state: TState,
    ) -> Result<Self::StartState, TState, TEvent>;

    fn add_start_end_state(
        self,
        start_event: TEvent,
        start_state: TState,
    ) -> Result<Self::EndState, TState, TEvent>;

    fn add_state(self, state: TState) -> Result<Self::StartState, TState, TEvent>;

    fn build(self) -> Result<StateMachineDefinition<TState, TEvent>, TState, TEvent>;
}

impl<TState, TEvent> BuilderState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type EndState = StateMachineBuilder<TState, TEvent>;
    type StartState = StateMachineBuilder<TState, TEvent>;

    #[inline]
    fn add_end_state(mut self, end_state: TState) -> Result<Self::StartState, TState, TEvent> {
        self.add_state_impl(end_state, NodeType::END).map(|_| self)
    }

    #[inline]
    fn add_start_state(
        mut self,
        start_event: TEvent,
        start_state: TState,
    ) -> Result<Self::StartState, TState, TEvent> {
        self.add_start_state_impl(self.initial_state, start_event, start_state).map(|_| self)
    }

    fn add_start_end_state(mut self, start_event: TEvent, start_state: TState) -> Result<Self::EndState, TState, TEvent> {
        self.add_start_end_state_impl(self.initial_state, start_event, start_state).map(|_| self)
    }

    #[inline]
    fn add_state(mut self, state: TState) -> Result<Self::StartState, TState, TEvent> {
        self.add_state_impl(state, NodeType::STATE).map(|_| self)
    }

    #[inline]
    fn build(self) -> Result<StateMachineDefinition<TState, TEvent>, TState, TEvent> {
        let undefined_states = self.transitions.iter()
            .flat_map(|(_, states)| states.values().collect::<Vec<_>>())
            .filter(|state| !self.states.contains(state))
            .copied()
            .collect::<HashSet<_>>().into_iter()
            .collect::<Vec<_>>();

        let unreachable = self.states.iter()
            .filter(|state| self.transitions.iter()
                .all(|(from, transitions)|
                    from == *state || transitions.values().all(|next| { next != *state })
                ))
            .copied()
            .collect::<Vec<_>>();

        if undefined_states.is_empty() && unreachable.is_empty() {
            Ok(StateMachineDefinition {
                end_states: Rc::new(self.end_states),
                initial_state: self.initial_state,
                transitions: Rc::new(self.transitions),
                triggers: Rc::new(self.triggers),
            })
        } else {
            Err(ValidationError {
                undefined_states,
                unreachable,
            })
        }
    }
}
