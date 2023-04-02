#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]

// dispensation from the pope
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)] // I'm ok with code organization
// #![allow(clippy::wildcard_imports)] // Internal wildcard imports don't hurt anyone ... do they?
#![allow(clippy::use_self)] // todo: clippy bug? compiler does not allow Self in some places clippy complains about

// temporary dispensation from the pope
#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cargo_common_metadata)]

pub use builder::StateMachineBuilder;
pub use state_machine::{detect_cycles, FiniteStateMachine, StateMachineDefinition};

// type TransitionPredicate<TState> = Box<dyn Fn(TState) -> bool>;

pub type Trigger<TState, TEvent> = Box<dyn Fn(TEvent, TState, TState)>;

pub mod builder;
mod graph;
pub mod state_machine;

pub mod prelude {
    pub use super::builder::{
        BuilderState, EndTriggersState, EndTriggerState, InitialState,
        StateMachineBuilder, TransitionsState, TransitionState, TriggersState,
        TriggerState,
    };
    pub use super::state_machine::{detect_cycles, FiniteStateMachine, StateMachineDefinition};
}

#[cfg(test)]
mod tests;
