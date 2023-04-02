pub use build_states::{
    BuilderState, EndTriggersState, EndTriggerState, InitialState,
    TransitionsState, TransitionState, TriggersState, TriggerState,
};
pub use builder::StateMachineBuilder;
pub use error::BuilderError;
pub use result::Result;

mod build_states;
#[allow(clippy::module_inception)]
mod builder;
mod error;
mod result;
