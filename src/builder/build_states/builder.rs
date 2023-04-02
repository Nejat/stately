use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

use crate::builder::{EndTriggerState, TriggerState};
use crate::builder::builder::{NodeType, StateMachineBuilder};
use crate::builder::BuilderError::ValidationError;
use crate::builder::Result;
use crate::StateMachineDefinition;

pub trait BuilderState<TState, TEvent> {
    type EndState: EndTriggerState<TState, TEvent>;

    type TriggerState: TriggerState<TState, TEvent>;

    fn add_end_state(self, state: TState) -> Result<Self::EndState, TState, TEvent>;

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

    fn add_state(self, state: TState) -> Result<Self::TriggerState, TState, TEvent>;

    fn build(self) -> Result<StateMachineDefinition<TState, TEvent>, TState, TEvent>;
}

impl<TState, TEvent> BuilderState<TState, TEvent> for StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash,
{
    type EndState = Self;
    type TriggerState = Self;

    #[inline]
    fn add_end_state(mut self, state: TState) -> Result<Self::TriggerState, TState, TEvent> {
        self.add_state_impl(state, NodeType::END).map(|_| self)
    }

    #[inline]
    fn add_start_state(
        mut self,
        event: TEvent,
        state: TState,
    ) -> Result<Self::TriggerState, TState, TEvent> {
        self.add_start_state_impl(event, state).map(|_| self)
    }

    fn add_start_end_state(mut self, event: TEvent, state: TState) -> Result<Self::EndState, TState, TEvent> {
        self.add_start_end_state_impl(event, state).map(|_| self)
    }

    #[inline]
    fn add_state(mut self, state: TState) -> Result<Self::TriggerState, TState, TEvent> {
        self.add_state_impl(state, NodeType::STATE).map(|_| self)
    }

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
