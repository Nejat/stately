pub use definition::StateMachineDefinition;
pub use detect::detect_cycles;
pub use error::StateError;
pub use fsm::FiniteStateMachine;
pub use result::Result;

mod definition;
mod detect;
mod error;
mod fsm;
mod machine;
mod result;
