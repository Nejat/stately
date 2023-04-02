use std::collections::HashMap;
use std::hash::Hash;
use std::iter::empty;
use std::ops::Deref;
use std::rc::Rc;

use crate::state_machine::{Result, StateMachineDefinition};
use crate::state_machine::fsm::FiniteStateMachine;
use crate::state_machine::StateError::{
    AlreadyStarted, EndState, InvalidTransition, NotAStartEvent, NotStarted,
};
use crate::Trigger;

const ALL_STATES_WITH_TRANSITIONS: &str = "all states should have defined transitions";

// StateMachineBuilder built implementation of a FiniteStateMachine trait object
pub struct StateMachine<TState, TEvent> {
    pub(crate) current_state: TState,
    pub(crate) has_cycle: Option<bool>,
    pub(crate) definition: StateMachineDefinition<TState, TEvent>,
}

impl<TState, TEvent> StateMachine<TState, TEvent>
    where TState: Copy
{
    /// Initializes a new instance of a state machine
    ///
    /// # Arguments
    ///
    /// * _`definition`_ - an instance of state machine definition
    ///
    /// # Results
    ///
    /// Returns a new initialized instance of a state machine
    pub const fn new(definition: StateMachineDefinition<TState, TEvent>) -> Self {
        Self {
            current_state: definition.initial_state,
            has_cycle: None,
            definition,
        }
    }
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
    fn clear_triggers(&mut self) {
        self.definition.triggers = Rc::new(HashMap::default());
    }

    fn current_state(&self) -> TState {
        self.current_state
    }

    fn event(&mut self, event: TEvent) -> Result<TState, TState, TEvent> {
        if !self.is_started() {
            return Err(NotStarted);
        }

        self.transition_on(event)
    }

    fn has_cycles(&mut self) -> Option<bool> {
        self.has_cycle.get_or_insert(crate::detect_cycles(&self.definition));

        self.has_cycle
    }


    fn is_end(&self) -> bool {
        self.definition.end_states.contains(&self.current_state)
    }

    fn is_started(&self) -> bool {
        self.current_state != self.definition.initial_state
    }

    fn new_triggers(&mut self, triggers: Vec<(TState, Vec<Trigger<TState, TEvent>>)>) {
        self.clear_triggers();

        self.definition.triggers = Rc::new(triggers.into_iter().collect());
    }

    fn next_states<'a>(&'a self) -> Box<dyn Iterator<Item=(&'a TEvent, &'a TState)> + 'a> {
        if self.is_end() {
            Box::new(empty())
        } else {
            Box::new(self.definition.transitions
                .get(&self.current_state)
                .expect(ALL_STATES_WITH_TRANSITIONS)
                .iter())
        }
    }

    fn reset(&mut self) -> TState {
        let last = self.current_state;

        self.current_state = self.definition.initial_state;

        last
    }

    fn start(&mut self, event: TEvent) -> Result<TState, TState, TEvent> {
        if self.is_started() {
            return Err(AlreadyStarted {
                current_state: self.current_state
            });
        }

        if !self.definition.transitions.get(&self.definition.initial_state)
            .expect(ALL_STATES_WITH_TRANSITIONS).contains_key(&event) {
            return Err(NotAStartEvent { event });
        }

        self.transition_on(event)
    }
}

impl<TState, TEvent> StateMachine<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    fn transition_on(&mut self, event: TEvent) -> Result<TState, TState, TEvent> {
        if self.is_end() {
            return Err(EndState { end: self.current_state });
        }

        let transitions = &self.definition
            .transitions.get(&self.current_state)
            .expect("all states should have defined transitions");

        let transition = *transitions.get(&event)
            .ok_or(InvalidTransition { event, current_state: self.current_state })?;

        if let Some(triggers) = self.definition.triggers.get(&transition) {
            for trigger in triggers {
                trigger(event, self.current_state, transition);
            }
        }

        self.current_state = transition;

        Ok(transition)
    }
}
