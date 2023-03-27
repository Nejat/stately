use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuilderError<TState, TEvent> {
    RedefinedInitialState,
    StateAlreadyDefined {
        state: TState
    },
    TransitionAlreadyDefined {
        event: TEvent,
        existing: TState,
    },
    ValidationError {
        undefined_states: Vec<TState>,
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
