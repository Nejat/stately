use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

use crate::state_machine::fsm::FiniteStateMachine;
use crate::state_machine::machine::StateMachine;
use crate::Trigger;

#[derive(Clone)]
pub struct StateMachineDefinition<TState, TEvent> {
    pub(crate) end_states: Rc<HashSet<TState>>,
    pub(crate) initial_state: TState,
    pub(crate) transitions: Rc<HashMap<TState, HashMap<TEvent, TState>>>,
    pub(crate) triggers: Rc<HashMap<TState, Vec<Trigger<TState, TEvent>>>>,
}

impl<TState, TEvent> StateMachineDefinition<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    pub fn create(&self) -> impl FiniteStateMachine<TState, TEvent> {
        StateMachine::new(self.clone())
    }
}
