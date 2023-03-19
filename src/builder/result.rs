use crate::builder::error::BuilderError;

pub type Result<T, TState, TEvent> = std::result::Result<T, BuilderError<TState, TEvent>>;
