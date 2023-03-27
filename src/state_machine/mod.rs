use std::ops::Deref;

pub use definition::StateMachineDefinition;
pub use detect::detect_cycles;
pub use error::StateError;
pub use result::Result;

use crate::Trigger;

mod definition;
mod detect;
mod error;
mod machine;
mod result;

pub trait FiniteStateMachine<TState, TEvent>: Deref {
    fn new(definition: StateMachineDefinition<TState, TEvent>) -> Self;

    fn has_cycles(&mut self) -> Option<bool> {
        None
    }

    fn clear_triggers(&mut self);

    fn current_state(&self) -> TState;

    fn event(&mut self, event: TEvent) -> Result<TState, TState, TEvent>;

    fn is_end(&self) -> bool;

    fn is_started(&self) -> bool;

    fn new_triggers(&mut self, triggers: Vec<(TState, Vec<Trigger<TState, TEvent>>)>);

    fn next_states<'a>(&'a self) -> Box<dyn Iterator<Item=(&'a TEvent, &'a TState)> + 'a>;

    fn reset(&mut self) -> TState;

    fn start(&mut self, event: TEvent) -> Result<TState, TState, TEvent>;
}
