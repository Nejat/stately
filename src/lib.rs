//! # Stately
//!
//! Stately is a [finite state machine] library
//!
//! # Features
//!
//! ## Fluent Builder
//!
//! Using Rust's powerful type system this library provides a fluent builder that validates
//! a lot of the state machine definition at compile time. The builder itself phases through
//! states to contextually provide only the necessary api for defining a state machine,
//! reducing the number of validations required at runtime.
//!
//! ### Compile Time Validation
//!
//! By splitting the builder into multiple states, the api provides a contextually relevant
//! builder; that allows it to make some compile time assumptions of the validity of the
//! state(s) being defined.
//!
//! _for instance, it ensures that every non-end state has at least one transition edge defined_
//!
//! <br>
//! <div>
//!     <img src="https://raw.githubusercontent.com/Nejat/stately/master/design/diagrams/builder.svg" alt="builder states" width="700"
//!     style="background: transparent; display: block; margin-left: auto; margin-right: auto;"/>
//! </div>
//! <br>
//!
//! **each state represents a phase of the builder, and it's contextual api**; _illustrated by the edges_
//!
//! <br>
//!
//! ### Run Time Validation
//!
//! Not all the validations of the state machine can be reasoned about at compile time.
//!
//! Validation also occurs during various state machine definition operations and then at
//! the final [`build`] operation.
//!
//! It is possible to end up with the following invalid state machine definition issues;
//!
//! * State Definition Validations
//!     * Duplicate states
//!     * Duplicate state transitions
//!     * _and_ Redefinition of the `Initial` state
//! * Build Validations
//!     * Unreachable states
//!     * Undefined states
//!
//! _* see_ [`BuildError`] _for more details_
//!
//! ## Detecting Cycles
//!
//! [`FiniteStateMachine<TState, TEvent>`] is defined as a directed graph,
//! and it is possible to define [cyclical] states, the [`has_cycle`]
//! method checks if a cycle exists
//!
//! # Example
//!
//! This example demonstrates all of the possible definable state machine components;
//!
//! * start state(s) \[A\]
//! * start end state(s) \[H\]
//! * end state(s) \[G, F, E\]
//! * state(s) \[B, B1, C, D\],
//! * single transitions
//! * multiple transitions
//! * _triggers are optional_
//! * and a cycle
//!
//!  <img src="https://raw.githubusercontent.com/Nejat/stately/master/design/diagrams/cyclical.svg" alt="cyclical states" width="400"
//!       style="background: transparent; position: absolute; Left: 450px; Margin-Top: 400px; z-index: 10000"/>
//!
//! ```rust
//! use std::fmt::{Display, Formatter};
//!
//! use stately::builder;
//! use stately::prelude::*;
//!
//! use crate::Event::*;
//! use crate::State::*;
//!
//! type BuilderResult = builder::Result<(), State, Event>;
//!
//! fn main() -> BuilderResult {
//!     // start with a new builder and an initial value
//!     let example = StateMachineBuilder::new()
//!         .add_start_state(Start, A)?
//!             .no_triggers()
//!             .transition_on(Done, E)?
//!             .final_transition_on(Next, B)?
//!         .add_end_state(E)?
//!             .no_triggers()
//!         .add_state(B)?
//!             .no_triggers()
//!             .transition_on(Next, C)?
//!             .transition_on(Loop, B1)?
//!             .final_transition_on(Done, G)?
//!         .add_state(B1)?
//!             .no_triggers()
//!             .only_transition_on(Next, D)?
//!         .add_state(C)?
//!             .no_triggers()
//!             .only_transition_on(Next, D)?
//!         .add_state(D)?
//!             .no_triggers()
//!             .transition_on(Next, F)?
//!             .transition_on(Loop, B)?
//!             .final_transition_on(Done, G)?
//!         .add_end_state(F)?
//!             .no_triggers()
//!         .add_end_state(G)?
//!             .no_triggers()
//!         .add_start_end_state(Skip, H)?
//!             .no_triggers()
//!         .build()?;
//!
//!     let mut state_machine = example.create();
//!
//!     assert!(state_machine.has_cycles().expect("fsm should have cycles"));
//!     assert_eq!(Initial, state_machine.current_state());
//!
//!     Ok(())
//! }
//!
//! #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
//! pub enum Event {
//!     Done,
//!     Loop,
//!     Next,
//!     Skip,
//!     Start,
//! }
//!
//! #[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
//! enum State {
//!     #[default]
//!     Initial,
//!     A,
//!     B,
//!     B1,
//!     C,
//!     D,
//!     E,
//!     F,
//!     G,
//!     H,
//! }
//! ```
//!
//! [finite state machine]: https://en.wikipedia.org/wiki/Finite-state_machine
//! [`build`]: builder::BuilderState::build
//! [`BuildError`]: builder::BuilderError
//! [`FiniteStateMachine<TState, TEvent>`]: FiniteStateMachine
//! [cyclical]: https://en.wikipedia.org/wiki/Cycle_(graph_theory)
//! [`has_cycle`]: FiniteStateMachine::has_cycles
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]

// doc lints
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::broken_intra_doc_links)]

// dispensation from the pope
#![allow(clippy::module_name_repetitions)] // this one makes naming modules difficult
#![allow(clippy::items_after_statements)]  // I'm ok with code organization

#[doc(inline)]
pub use builder::StateMachineBuilder;
#[doc(inline)]
pub use state_machine::{detect_cycles, FiniteStateMachine, StateMachineDefinition};

// type TransitionPredicate<TState> = Box<dyn Fn(TState) -> bool>;

/// A type that represents an event transition trigger callback
///
///### Arguments
///
/// * _`TEvent`_ - the event that triggered the transition
/// * _`TState`_ -  the prior state
/// * _`TState`_ -  the current state
pub type Trigger<TState, TEvent> = Box<dyn Fn(TEvent, TState, TState)>;

pub mod builder;
mod graph;
pub mod state_machine;

pub mod prelude {
    //! Required public api for using `Stately`; grouped for convenience
    #[doc(no_inline)]
    pub use super::builder::{
        BuilderState, EndTriggersState, EndTriggerState, InitialState,
        StateMachineBuilder, TransitionsState, TransitionState, TriggersState,
        TriggerState,
    };
    #[doc(no_inline)]
    pub use super::state_machine::{FiniteStateMachine, StateMachineDefinition};
}

#[cfg(test)]
mod tests;
