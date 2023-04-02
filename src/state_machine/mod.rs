//! Generic finite state machine
//!
//! A [finite state machine] provides a mechanism for
//! encapsulating the rules of a system of states and
//! the transitions, _edges_, from one state to another.
//!
//! The state machine can occupy only one state at a time, and
//! only transition to the next state on prescribed events.
//!
//! The state machine has an initial state, at least one
//! start transition and either an end state and/or a cycle;
//! _which would cycle indefinitely_.
//!
//!  <img src="https://raw.githubusercontent.com/Nejat/stately/master/design/diagrams/turnstile.svg" alt="turnstile states" width="200"
//!        style="background: transparent; position: absolute; left: 600px; margin-top: 250px; z-index: 10000;"/>
//!
//! ```rust
//! use std::default::Default;
//!
//! use stately::prelude::*;
//! use Event::{Coin, Push, On};
//! use State::{Locked, Unlocked};
//!
//! type Result<T> = stately::state_machine::Result<T, State, Event>;
//!
//! fn main() -> Result<()> {
//!     let turnstile_fsm = turnstile_fsm();
//!     let mut turnstile = turnstile_fsm.create();
//!
//!     let current_state = turnstile.start(On)?;
//!
//!     assert_eq!(Locked, current_state);
//!
//!     let current_state = turnstile.event(Coin)?;
//!
//!     assert_eq!(Unlocked, current_state);
//!
//!     let current_state = turnstile.event(Push)?;
//!
//!     assert_eq!(Locked, current_state);
//!
//!     Ok(())
//! }
//!
//! // https://en.wikipedia.org/wiki/Finite-state_machine
//! fn turnstile_fsm() -> StateMachineDefinition<State, Event> {
//!    // ...
//!    # StateMachineBuilder::new()
//!    #    .add_start_state(On, Locked).unwrap()
//!    #        .no_triggers()
//!    #        .transition_on(Push, Locked).unwrap()
//!    #        .final_transition_on(Coin, Unlocked).unwrap()
//!    #    .add_state(Unlocked).unwrap()
//!    #        .no_triggers()
//!    #        .transition_on(Coin, Unlocked).unwrap()
//!    #        .final_transition_on(Push, Locked).unwrap()
//!    #    .build().unwrap()
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
//!
//! [finite state machine]: https://en.wikipedia.org/wiki/Finite-state_machine
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
