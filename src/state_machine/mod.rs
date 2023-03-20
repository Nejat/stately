use std::collections::HashMap;
use std::hash::Hash;
use std::iter::empty;
use std::ops::Deref;
use std::rc::Rc;

pub use definition::StateMachineDefinition;
use crate::Triggers;

mod definition;

pub trait FiniteStateMachine<TState, TEvent>: Deref {
    fn new(definition: StateMachineDefinition<TState, TEvent>) -> Self;

    fn clear_triggers(&mut self);

    fn current_state(&self) -> TState;

    fn event(&mut self, event: TEvent) -> TState;

    fn has_trigger(&self) -> bool;

    fn is_end(&self) -> bool;

    fn is_start(&self) -> bool;

    fn new_triggers(&mut self, triggers: Vec<(TState, Vec<Triggers<TState, TEvent>>)>);

    fn next_states<'a>(&'a self) -> Box<dyn Iterator<Item=(&'a TEvent, &'a TState)> + 'a>;

    fn reset(&mut self) -> TState;

    fn start(&mut self, event: TEvent) -> TState;
}

pub struct StateMachine<TState, TEvent> {
    pub(crate) current_state: TState,
    pub(crate) definition: StateMachineDefinition<TState, TEvent>
}

impl<TState, TEvent> Deref for StateMachine<TState, TEvent>
    where TEvent: Copy + Eq + Hash,
          TState: Copy + Eq + Hash
{
    type Target = TState;

    fn deref(&self) -> &Self::Target {
        &self.current_state
    }
}

impl<TState, TEvent> FiniteStateMachine<TState, TEvent> for StateMachine<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    fn new(definition: StateMachineDefinition<TState, TEvent>) -> Self {
        Self {
            current_state: definition.initial_state,
            definition,
        }
    }

    fn clear_triggers(&mut self) {
        self.definition.triggers = Rc::new(HashMap::default());
    }

    fn current_state(&self) -> TState {
        self.current_state
    }

    fn event(&mut self, event: TEvent) -> TState {
        let transitions = &self.definition
            .transitions.get(&self.current_state)
            .expect("a transition for event for current state");

        let transition = *transitions.get(&event)
            .expect("a transition for event");

        if let Some(triggers) = self.definition.triggers.get(&transition) {
            for trigger in triggers {
                trigger(event, self.current_state, transition);
            }
        }

        self.current_state = transition;

        transition
    }

    fn has_trigger(&self) -> bool {
        self.definition.triggers.contains_key(&self.current_state)
    }

    fn is_end(&self) -> bool {
        self.definition.end_states.contains(&self.current_state)
    }

    fn is_start(&self) -> bool {
        self.definition.start_states.contains(&self.current_state)
    }

    fn new_triggers(&mut self, triggers: Vec<(TState, Vec<Triggers<TState, TEvent>>)>) {
        self.clear_triggers();

        self.definition.triggers = Rc::new(triggers.into_iter().collect());
    }

    fn next_states<'a>(&'a self) -> Box<dyn Iterator<Item=(&'a TEvent, &'a TState)> + 'a> {
        if self.is_end() {
            Box::new(empty())
        } else {
            Box::new(self.definition.transitions
                .get(&self.current_state)
                .expect("all states to have an event transition")
                .iter())
        }
    }

    fn reset(&mut self) -> TState {
        let last = self.current_state;

        self.current_state = self.definition.initial_state;

        last
    }

    fn start(&mut self, event: TEvent) -> TState {
        self.current_state = self.definition.initial_state;

        self.event(event)
    }
}

