use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

use crate::state_machine::fsm::FiniteStateMachine;
use crate::state_machine::machine::StateMachine;
use crate::Trigger;

/// Definition of a state machine, including; _all states_, _transition
/// edges_, and _optional triggers_
///
/// An instance can be initialized with the [`StateMachineBuilder`] and
/// is instantiated with the [`BuilderState::build`] method..
///
/// ### Generic Data Types
///
/// * _`TState`_ - represents the states of a state machine
/// * _`TEvent`_ - represents the transition events of a state machine
///
/// _i.e._
///
/// ```rust
/// # use std::default::Default;
/// #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
/// enum State {
///     #[default] Initial, Locked, Unlocked,
/// }
///
/// #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// enum Event {
///     On, Push, Coin,
/// }
/// ```
///
/// [`StateMachineBuilder`]: crate::StateMachineBuilder
/// [`BuilderState::build`]: crate::builder::BuilderState::build
#[derive(Clone)]
pub struct StateMachineDefinition<TState, TEvent> {
    pub(crate) end_states: Rc<HashSet<TState>>,
    pub(crate) initial_state: TState,
    pub(crate) transitions: Rc<HashMap<TState, HashMap<TEvent, TState>>>,
    pub(crate) triggers: Rc<HashMap<TState, Vec<Trigger<TState, TEvent>>>>,
}

impl<TState, TEvent> StateMachineDefinition<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    /// Creates a new [`FiniteStateMachine`] from the state machine definition
    ///
    /// _* see the [`state machine`](crate::state_machine) module for an example_
    ///
    /// # Results
    ///
    /// Returns an implementation of a [`FiniteStateMachine`]
    ///
    pub fn create(&self) -> impl FiniteStateMachine<TState, TEvent> {
        StateMachine::new(self.clone())
    }
}
