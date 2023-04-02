use crate::builder::BuilderError;

/// Result is a type that represents either success ([`Ok`]) or failure ([`Err`])
/// for [`StateMachineBuild`] operations
///
/// ### Generic Data Types
///
/// * _`T`_ - [`Ok`] return type
/// * _`TState`_ - represents the states of a state machine
/// * _`TEvent`_ - represents the transition events of a state machine
///
/// [`Ok`]: Ok
/// [`Err`]: Err
/// [`StateMachineBuild`]: crate::StateMachineBuilder
pub type Result<T, TState, TEvent> = std::result::Result<T, BuilderError<TState, TEvent>>;
