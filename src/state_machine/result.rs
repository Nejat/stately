use crate::state_machine::StateError;

pub type Result<T, TState, TEvent> = std::result::Result<T, StateError<TState, TEvent>>;
