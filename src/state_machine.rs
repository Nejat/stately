use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::empty;
use std::ops::Deref;

use crate::Triggers;

pub trait FiniteStateMachine<TState, TEvent> {
    fn new(initial_state: TState) -> Self;

    fn event(&mut self, event: TEvent) -> TState;

    fn has_trigger(&self) -> bool;

    fn is_start(&self) -> bool;

    fn is_end(&self) -> bool;

    fn next_states<'a>(&'a self) -> Box<dyn Iterator<Item=(&'a TEvent, &'a TState)> + 'a>;

    fn start(&mut self, event: TEvent) -> TState;
}

pub struct StateMachine<TState, TEvent> {
    pub(crate) current_state: TState,
    pub(crate) end_states: HashSet<TState>,
    pub(crate) initial_state: TState,
    pub(crate) start_states: HashSet<TState>,
    pub(crate) states: HashSet<TState>,
    pub(crate) transitions: HashMap<TState, HashMap<TEvent, TState>>,
    pub(crate) triggers: HashMap<TState, Vec<Triggers<TState, TEvent>>>,
}

impl<TState, TEvent> Deref for StateMachine<TState, TEvent> {
    type Target = TState;

    fn deref(&self) -> &Self::Target {
        &self.current_state
    }
}

impl<TState, TEvent> FiniteStateMachine<TState, TEvent> for StateMachine<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    fn new(initial_state: TState) -> Self {
        Self {
            current_state: initial_state,
            end_states: HashSet::default(),
            initial_state,
            start_states: HashSet::default(),
            states: HashSet::default(),
            transitions: HashMap::default(),
            triggers: HashMap::default(),
        }
    }

    fn event(&mut self, event: TEvent) -> TState {
        let transitions = &self
            .transitions.get(&self.current_state)
            .expect("a transition for event for current state");

        let transition = *transitions.get(&event)
            .expect("a transition for event");

        if let Some(triggers) = self.triggers.get(&transition) {
            for trigger in triggers {
                trigger(event, self.current_state, transition);
            }
        }

        self.current_state = transition;

        transition
    }

    fn has_trigger(&self) -> bool {
        self.triggers.contains_key(&self.current_state)
    }

    fn is_start(&self) -> bool {
        self.start_states.contains(&self.current_state)
    }

    fn is_end(&self) -> bool {
        self.end_states.contains(&self.current_state)
    }

    fn next_states<'a>(&'a self) -> Box<dyn Iterator<Item=(&'a TEvent, &'a TState)> + 'a> {
        if self.is_end() {
            Box::new(empty())
        } else {
            Box::new(self.transitions
                .get(&self.current_state)
                .expect("all states to have an event transition")
                .iter())
        }
    }

    fn start(&mut self, event: TEvent) -> TState {
        self.current_state = self.initial_state;

        self.event(event)
    }
}

impl<TState, TEvent> Default for StateMachine<TState, TEvent>
    where TState: Copy + Default + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    fn default() -> Self {
        Self::new(TState::default())
    }
}

