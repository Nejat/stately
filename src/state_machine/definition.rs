use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

use crate::state_machine::{FiniteStateMachine, StateMachine};
use crate::Triggers;

pub struct StateMachineDefinition<TState, TEvent> {
    pub(crate) end_states: Rc<HashSet<TState>>,
    pub(crate) initial_state: TState,
    pub(crate) start_states: Rc<HashSet<TState>>,
    pub(crate) states: Rc<HashSet<TState>>,
    pub(crate) transitions: Rc<HashMap<TState, HashMap<TEvent, TState>>>,
    pub(crate) triggers: Rc<HashMap<TState, Vec<Triggers<TState, TEvent>>>>,
}

impl<TState, TEvent> StateMachineDefinition<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    pub fn new(initial_state: TState) -> Self {
        Self {
            end_states: Rc::new(HashSet::default()),
            initial_state,
            start_states: Rc::new(HashSet::default()),
            states: Rc::new(HashSet::default()),
            transitions: Rc::new(HashMap::default()),
            triggers: Rc::new(HashMap::default()),
        }
    }

    pub fn create(&self) -> StateMachine<TState, TEvent> {
        StateMachine::new(Self {
            end_states: self.end_states.clone(),
            initial_state: self.initial_state,
            start_states: self.start_states.clone(),
            states: self.states.clone(),
            transitions: self.transitions.clone(),
            triggers: self.triggers.clone(),
        })
    }
}
