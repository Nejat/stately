use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Deref;

use crate::Triggers;

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

/*

// pub type Result<T, TState, TEvent> = std::result::Result<T, StateError<TState, TEvent>>;

#[derive(Error, Debug)]
pub enum StateError<TState, TEvent> {
    NoStartTransitions,
    StateAlreadyDefined(TState),
    TransitionAlreadyDefined {
        event: TEvent,
        existing: TState,
    },
}

impl<TState, TEvent> Display for StateError<TState, TEvent>
    where TState: Display,
          TEvent: Display,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoStartTransitions =>
                fmt.write_fmt(format_args!("No Start Trans")),

            Self::StateAlreadyDefined(defined) =>
                fmt.write_fmt(format_args!("{defined} state has already been defined")),

            Self::TransitionAlreadyDefined { event, existing } =>
                fmt.write_fmt(format_args!("{event} event already transitions to {existing}")),
        }
    }
}
*/

impl<TState, TEvent> StateMachine<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    pub fn new(initial_state: TState) -> Self {
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

    pub fn event(&mut self, event: TEvent) -> TState {
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

    pub fn is_start(&self) -> bool {
        self.start_states.contains(&self.current_state)
    }

    pub fn is_end(&self) -> bool {
        self.end_states.contains(&self.current_state)
    }

    pub fn start(&mut self, event: TEvent) -> TState {
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

