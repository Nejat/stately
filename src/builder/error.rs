use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuilderError<TState, TEvent> {
    InitialState,
    StateAlreadyDefined(TState),
    TransitionAlreadyDefined {
        event: TEvent,
        existing: TState,
    },
    ValidationError {
        no_end_states: bool,
        undefined_states: Vec<TState>,
    },
}

impl<TState, TEvent> Display for BuilderError<TState, TEvent>
    where TState: Debug + Display,
          TEvent: Display,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitialState =>
                fmt.write_fmt(format_args!("Initial state can not be explicitly defined")),

            Self::StateAlreadyDefined(defined) =>
                fmt.write_fmt(format_args!("{defined} state has already been defined")),

            Self::TransitionAlreadyDefined { event, existing } =>
                fmt.write_fmt(format_args!("{event} event already transitions to {existing}")),

            Self::ValidationError { no_end_states, undefined_states } => {
                let has_undefined_states = !undefined_states.is_empty();

                if *no_end_states {
                    fmt.write_fmt(format_args!("No end states"))?;

                    if has_undefined_states {
                        fmt.write_fmt(format_args!(", "))?;
                    }
                }

                if has_undefined_states {
                    fmt.write_fmt(format_args!("Undefined state(s) {undefined_states:?}"))?;
                }

                Ok(())
            }
        }
    }
}
