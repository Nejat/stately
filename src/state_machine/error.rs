use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StateError<TState, TEvent> {
    AlreadyStarted {
        current_state: TState,
    },
    EndState {
        end: TState
    },
    NotAStartEvent {
        event: TEvent
    },
    NotStarted,
    InvalidTransition {
        event: TEvent,
        current_state: TState,
    },
}

impl<TState, TEvent> Display for StateError<TState, TEvent>
    where TState: Display,
          TEvent: Display,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyStarted { current_state } =>
                fmt.write_fmt(format_args!(
                    "State machine is already started; current state {current_state}"
                )),

            Self::EndState { end } =>
                fmt.write_fmt(format_args!("Reached end state {end}")),

            Self::NotAStartEvent { event } =>
                fmt.write_fmt(format_args!("{event} is not a starting event")),

            Self::NotStarted =>
                fmt.write_fmt(format_args!("State machine is not started")),

            Self::InvalidTransition { event, current_state } =>
                fmt.write_fmt(format_args!("Can not transition from {current_state} on {event}")),
        }
    }
}
