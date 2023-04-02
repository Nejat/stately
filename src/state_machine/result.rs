use crate::state_machine::StateError;

/// Result is a type that represents either success ([`Ok`]) or failure ([`Err`])
/// for [`FiniteStateMachine`] operations
///
/// ### Generic Data Types
///
/// * _`T`_ - successful return type
/// * _`TState`_ - represents the states of a state machine
/// * _`TEvent`_ - represents the transition events of a state machine
///
/// [`Ok`]: Ok
/// [`Err`]: Err
/// [`FiniteStateMachine`]: crate::FiniteStateMachine
pub type Result<T, TState, TEvent> = std::result::Result<T, StateError<TState, TEvent>>;
