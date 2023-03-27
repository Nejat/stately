use crate::builder::BuilderError;

pub type Result<T, TState, TEvent> = std::result::Result<T, BuilderError<TState, TEvent>>;
