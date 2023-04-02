use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

/// The [`Error`] type for [`FiniteStateMachine`] operations;
/// _[`start`]_, _and [`event`]_
///
/// [`Error`]: std::error::Error
/// [`FiniteStateMachine`]: crate::FiniteStateMachine
/// [`start`]: crate::FiniteStateMachine::start
/// [`event`]: crate::FiniteStateMachine::event
#[derive(Error, Debug)]
pub enum StateError<TState, TEvent> {
    /// Occurs when attempting to [`start`] a state machine
    /// which is already started
    ///
    /// [`start`]: crate::FiniteStateMachine::start
    AlreadyStarted {
        /// the current state of the machine
        current_state: TState,
    },

    /// Occurs when an [`event`] operation is attempted on a
    /// state machine that has ended
    ///
    /// [`event`]: crate::FiniteStateMachine::event
    EndState {
        /// the current state of the machine
        end: TState
    },

    /// Occurs when an undefined [`event`] operation, for the
    /// current state, is attempted on a state machine
    ///
    /// [`event`]: crate::FiniteStateMachine::event
    InvalidTransition {
        /// the invalid event
        event: TEvent,

        /// the current state of the machine
        current_state: TState,
    },

    /// Occurs when attempting to [`start`] a state machine
    /// with and an invalid event
    ///
    /// [`start`]: crate::FiniteStateMachine::start
    NotAStartEvent {
        /// the invalid event
        event: TEvent
    },

    /// Occurs when an [`event`] operation is attempted on a
    /// state machine that has not been started
    ///
    /// [`event`]: crate::FiniteStateMachine::event
    NotStarted,

    /// Occurs when new triggers are defined for undefined
    /// states
    UndefinedStates {
        /// the collection of undefined states
        states: Vec<TState>
    }
}

impl<TState, TEvent> Display for StateError<TState, TEvent>
    where TState: Debug + Display,
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

            Self::InvalidTransition { event, current_state } =>
                fmt.write_fmt(format_args!("Can not transition from {current_state} on {event}")),

            Self::NotAStartEvent { event } =>
                fmt.write_fmt(format_args!("{event} is not a starting event")),

            Self::NotStarted =>
                fmt.write_fmt(format_args!("State machine is not started")),

            Self::UndefinedStates { states } =>
                fmt.write_fmt(format_args!("{states:?} are not defined states"))
        }
    }
}
