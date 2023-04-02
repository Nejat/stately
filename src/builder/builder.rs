use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bitflags::bitflags;

use crate::{builder, Trigger};
use crate::builder::BuilderError::{
    RedefinedInitialState, StateAlreadyDefined, TransitionAlreadyDefined,
};
use crate::builder::InitialState;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct NodeType: u8 {
        const END   = 0b10;
        const START = 0b01;
        const STATE = 0b00;
    }
}

/// Fluent state machine builder, which is used to build a validated
/// [`StateMachineDefinition`]
///
/// Validation occurs in three separate phases;
///
/// 1. Transitioning between state builder states to eliminate a class of validations
/// enforced at compiletime
/// 2. Defining different components of the state machine definition
/// 3. Building the state machine definition; where unreachable and undefined
/// states are detected
///
/// ### Example
///
///  <img src="https://raw.githubusercontent.com/Nejat/stately/master/design/diagrams/cyclical.svg" alt="cyclical states" width="400"
///       style="background: transparent; position: absolute; Left: 450px; Margin-Top: 400px; z-index: 10000"/>
///
/// ```rust
/// use std::fmt::{Display, Formatter};
///
/// use stately::builder::Result;
/// use stately::prelude::*;
///
/// use Event::*;
/// use State::*;
///
/// type BuilderResult = Result<StateMachineDefinition<State, Event>, State, Event>;
///
/// fn cyclical_fsm() -> BuilderResult {
///     StateMachineBuilder::new()
///         .add_start_state(Start, A)?
///             .no_triggers()
///             .transition_on(Done, E)?
///             .final_transition_on(Next, B)?
///         .add_end_state(E)?
///             .no_triggers()
///         .add_state(B)?
///             .no_triggers()
///             .transition_on(Next, C)?
///             .transition_on(Loop, B1)?
///             .final_transition_on(Done, G)?
///         .add_state(B1)?
///             .no_triggers()
///             .only_transition_on(Next, D)?
///         .add_state(C)?
///             .no_triggers()
///             .only_transition_on(Next, D)?
///         .add_state(D)?
///             .no_triggers()
///             .transition_on(Next, F)?
///             .transition_on(Loop, B)?
///             .final_transition_on(Done, G)?
///         .add_end_state(F)?
///             .no_triggers()
///         .add_end_state(G)?
///             .no_triggers()
///         .add_start_end_state(Skip, H)?
///             .no_triggers()
///         .build()
/// }
///
/// #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// pub enum Event {
///     Done,
///     Loop,
///     Next,
///     Skip,
///     Start,
/// }
///
/// #[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
/// enum State {
///     #[default]
///     Initial,
///     A,
///     B,
///     B1,
///     C,
///     D,
///     E,
///     F,
///     G,
///     H,
/// }
/// ```
///
/// [`StateMachineDefinition`]: crate::state_machine::StateMachineDefinition
pub struct StateMachineBuilder<TState, TEvent> {
    pub(crate) current: TState,
    pub(crate) initial_state: TState,
    pub(crate) end_states: HashSet<TState>,
    pub(crate) start_states: HashSet<TState>,
    pub(crate) states: HashSet<TState>,
    pub(crate) transitions: HashMap<TState, HashMap<TEvent, TState>>,
    pub(crate) triggers: HashMap<TState, Vec<Trigger<TState, TEvent>>>,
}

impl<TState, TEvent> StateMachineBuilder<TState, TEvent>
    where TState: Copy + Default + Eq + Hash,
          TEvent: Copy + Eq + Hash
{
    /// Create a new instance of a `StateMachineBuilder`
    ///
    /// # Results
    ///
    /// Returns an [`InitialState`] to start the build process
    ///
    #[allow(clippy::new_ret_no_self)]
    #[must_use]
    pub fn new() -> impl InitialState<TState, TEvent> {
        let initial_state = TState::default();

        Self {
            current: initial_state,
            initial_state,
            end_states: HashSet::default(),
            start_states: HashSet::default(),
            states: HashSet::default(),
            transitions: HashMap::default(),
            triggers: HashMap::default(),
        }
    }
}

impl<TState, TEvent> StateMachineBuilder<TState, TEvent>
    where TState: Copy + Eq + Hash,
          TEvent: Eq + Hash
{
    /// Defines a new state for the state machine
    ///
    /// _used by specific trait implementations of the state machine builder_
    ///
    /// # Arguments
    ///
    /// * _`state`_ - state to add
    /// * _`NodeType`_ - node type of the state being added; _i.e start, end, start|end_
    ///
    /// # Results
    ///
    /// Returns `()` if there aren't any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: builder::BuilderError
    pub(crate) fn add_state_impl(
        &mut self,
        state: TState,
        node_type: NodeType,
    ) -> builder::Result<(), TState, TEvent> {
        // check if redefining `initial` state
        if state == self.initial_state {
            return Err(RedefinedInitialState);
        }

        // check if redefining a state that was already defined
        if self.states.contains(&state) {
            return Err(StateAlreadyDefined { state });
        }

        // add state to tracker
        self.states.borrow_mut().insert(state);

        // if the state is an end state, define it as such
        if node_type.contains(NodeType::END) {
            self.end_states.borrow_mut().insert(state);
        }

        // if the state is a start state, define it as such
        if node_type.contains(NodeType::START) {
            self.start_states.borrow_mut().insert(state);
        }

        // record the new state as the current state
        self.current = state;

        Ok(())
    }

    /// Defines a new start/end state for the state machine
    ///
    /// _used by specific trait implementations of the state machine builder_
    ///
    /// # Arguments
    ///
    /// * `event` - the event that starts the state machine
    /// * `state` - the start/end state that the state machine transitions to
    ///
    /// # Results
    ///
    /// Returns `()` if there aren't any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: builder::BuilderError
    #[inline]
    pub(crate) fn add_start_end_state_impl(
        &mut self,
        event: TEvent,
        state: TState,
    ) -> builder::Result<(), TState, TEvent> {
        self.add_state_impl(state, NodeType::START | NodeType::END)?;
        self.add_transition_impl(self.initial_state, event, state)
    }

    /// Defines a new start state for the state machine
    ///
    /// _used by specific trait implementations of the state machine builder_
    ///
    ///
    /// # Arguments
    ///
    /// * `event` - the event that starts the state machine
    /// * `state` - the start state that the state machine transitions to
    ///
    /// # Results
    ///
    /// Returns `()` if there aren't any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: builder::BuilderError
    #[inline]
    pub(crate) fn add_start_state_impl(
        &mut self,
        event: TEvent,
        state: TState,
    ) -> builder::Result<(), TState, TEvent> {
        self.add_state_impl(state, NodeType::START)?;
        self.add_transition_impl(self.initial_state, event, state)
    }

    /// Defines a new transition for the current state being defined
    /// for the state machine
    ///
    /// _used by specific trait implementations of the state machine builder_
    ///
    /// # Arguments
    ///
    /// * _`state`_ - the event that transitions the state machine
    /// * _`event`_ - the state that the state machine transitions from
    /// * _`next`_ - the state that the state machine transitions to
    ///
    /// # Results
    ///
    /// Returns `()` if there aren't any validation errors
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if there are any validation errors
    ///
    /// [`BuildError`]: builder::BuilderError
    pub(crate) fn add_transition_impl(
        &mut self,
        state: TState,
        event: TEvent,
        next: TState,
    ) -> builder::Result<(), TState, TEvent> {
        let entry = self.transitions.entry(state)
            .or_insert_with(HashMap::new);

        if entry.contains_key(&event) {
            let existing = entry[&event];

            return Err(TransitionAlreadyDefined { event, existing });
        }

        entry.borrow_mut().insert(event, next);

        Ok(())
    }

    /// Defines a new trigger for the current state being defined
    /// for the state machine
    ///
    /// _used by specific trait implementations of the state machine builder_
    ///
    /// # Arguments
    ///
    /// * _`state`_ - the state that the state machine transitions to
    /// * _`trigger`_ - the callback to invoke on transition
    ///
    pub(crate) fn trigger_on_impl(
        &mut self,
        state: TState,
        trigger: impl Fn(TEvent, TState, TState) + 'static,
    ) {
        self.triggers.entry(state)
            .or_insert_with(Vec::new)
            .push(Box::new(trigger));
    }
}
