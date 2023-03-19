use std::fmt::{Display, Formatter};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuilderError<TState, TEvent> {
    InitialState,
    StateAlreadyDefined(TState),
    TransitionAlreadyDefined {
        event: TEvent,
        existing: TState,
    },
}

impl<TState, TEvent> Display for BuilderError<TState, TEvent>
    where TState: Display,
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
        }
    }
}
