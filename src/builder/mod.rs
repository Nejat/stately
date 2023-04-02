//! Generic state machine builder
//!
//! The state machine builder, is designed to transition between contextually
//! relevant builder states to provide an ergonomic api for defining state
//! machines, while minimizing the number of run time validations required;
//! therefore catching some definition errors at compile time.
//!
//! <br><br>
//! <div>
//!     <img src="https://raw.githubusercontent.com/Nejat/stately/master/design/diagrams/builder.svg" alt="builder states" width="700"
//!     style="background: transparent; display: block; margin-left: auto; margin-right: auto;"/>
//! </div>
//! <br><br>
//!
//! __each state represents a phase of the builder, and it's contextual api__; _illustrated by the edges_
//!
//! <br><br>
//!
//! ### Example
//!
//!  <img src="https://raw.githubusercontent.com/Nejat/stately/master/design/diagrams/turnstile.svg" alt="turnstile states" width="200"
//!        style="background: transparent; position: absolute; left: 600px; margin-top: 150px; z-index: 10000;"/>
//!
//! ```rust
//! // https://en.wikipedia.org/wiki/Finite-state_machine
//!
//! use stately::builder::Result;
//! use stately::prelude::*;
//! use Event::{Coin, On, Push};
//! use State::{Initial, Locked, Unlocked};
//!
//! type BuilderResult = Result<StateMachineDefinition<State, Event>, State, Event>;
//!
//! fn turnstile_fsm() -> BuilderResult {
//!     StateMachineBuilder::new()
//!         .add_start_state(On, Locked)?
//!             .no_triggers()
//!             .transition_on(Push, Locked)?
//!             .final_transition_on(Coin, Unlocked)?
//!         .add_state(Unlocked)?
//!             .no_triggers()
//!             .transition_on(Coin, Unlocked)?
//!             .final_transition_on(Push, Locked)?
//!         .build()
//! }
//!
//! #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
//! enum State {
//!     #[default] Initial, Locked, Unlocked,
//! }
//!
//! #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
//! enum Event {
//!     On, Push, Coin,
//! }
//! ```
#[doc(inline)]
pub use build_states::{
    BuilderState, EndTriggersState, EndTriggerState, InitialState,
    TransitionsState, TransitionState, TriggersState, TriggerState,
};
#[doc(inline)]
pub use builder::StateMachineBuilder;
#[doc(inline)]
pub use error::BuilderError;
#[doc(inline)]
pub use result::Result;

mod build_states;
#[allow(clippy::module_inception)] // it's not leaky
mod builder;
mod error;
mod result;
