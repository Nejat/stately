#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]

// dispensation from the pope
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)] // I'm ok with code organization
#![allow(clippy::wildcard_imports)] // Internal wildcard imports don't hurt anyone ... do they?
#![allow(clippy::use_self)] // todo: clippy bug? compiler does not allow Self in some places clippy complains about

// temporary dispensation from the pope
#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cargo_common_metadata)]

pub use builder::build_rules;
pub use builder::StateMachineBuilder;
pub use state_machine::StateMachineDefinition;

// type TransitionPredicate<TState> = Box<dyn Fn(TState) -> bool>;

type Triggers<TState, TEvent> = Box<dyn Fn(TEvent, TState, TState)>;

mod state_machine;
mod builder;

pub mod prelude {
    pub use super::builder::build_rules::*;
    pub use super::builder::error::BuilderError;
    pub use super::builder::StateMachineBuilder;
    pub use super::state_machine::{FiniteStateMachine, StateMachineDefinition};
}