use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

/// The [`Error`] type for [`StateMachineBuilder`] operations
///
/// [`Error`]: std::error::Error
/// [`StateMachineBuilder`]: crate::StateMachineBuilder
#[derive(Error, Debug)]
pub enum BuilderError<TState, TEvent> {
    /// Occurs when the initial state of `TState` is redefined
    ///
    /// _*_ `TState` _implements_ [`Default`]_, which is used as the initial state_
    RedefinedInitialState,

    /// Occurs when a `TState` is redefined for an existing state definition
    StateAlreadyDefined {
        /// Existing `TState` definition
        state: TState
    },

    /// Occurs when a transition on `TEvent` is defined for an existing transition
    TransitionAlreadyDefined {
        /// `TEvent` to transition on
        event: TEvent,

        /// Existing `TState` to transition to
        existing: TState,
    },

    /// Occurs when a state machine definition build fails validation
    ValidationError {
        /// A collection of all the expected `TState`s that are undefined
        undefined_states: Vec<TState>,

        /// A collection of all defined `TState`s that are unreachable,
        /// _i.e._ there are no transitions to the state
        unreachable: Vec<TState>,
    },
}

impl<TState, TEvent> Display for BuilderError<TState, TEvent>
    where TState: Debug + Display,
          TEvent: Display,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RedefinedInitialState =>
                fmt.write_fmt(format_args!("Initial state can not be explicitly defined")),

            Self::StateAlreadyDefined { state } =>
                fmt.write_fmt(format_args!("{state} state has already been defined")),

            Self::TransitionAlreadyDefined { event, existing } =>
                fmt.write_fmt(format_args!("{event} event already transitions to {existing}")),

            Self::ValidationError { undefined_states, unreachable } => {
                let has_unreachable = !undefined_states.is_empty();

                if !unreachable.is_empty() {
                    fmt.write_fmt(format_args!("Undefined state(s) {undefined_states:?}"))?;

                    if has_unreachable {
                        fmt.write_fmt(format_args!(", "))?;
                    }
                }

                if has_unreachable {
                    fmt.write_fmt(format_args!("Unreachable state(s) {unreachable:?}"))?;
                }

                Ok(())
            }
        }
    }
}
