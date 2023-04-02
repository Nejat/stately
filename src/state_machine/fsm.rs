use std::ops::Deref;

use crate::{state_machine, Trigger};

pub trait FiniteStateMachine<TState, TEvent>: Deref<Target=TState> {
    fn clear_triggers(&mut self);

    fn current_state(&self) -> TState;

    fn event(&mut self, event: TEvent) -> state_machine::Result<TState, TState, TEvent>;

    fn has_cycles(&mut self) -> Option<bool> {
        None
    }

    fn is_end(&self) -> bool;

    fn is_started(&self) -> bool;

    fn new_triggers(&mut self, triggers: Vec<(TState, Vec<Trigger<TState, TEvent>>)>);

    fn next_states<'a>(&'a self) -> Box<dyn Iterator<Item=(&'a TEvent, &'a TState)> + 'a>;

    fn reset(&mut self) -> TState;

    fn start(&mut self, event: TEvent) -> state_machine::Result<TState, TState, TEvent>;
}
