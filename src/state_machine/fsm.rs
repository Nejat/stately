use std::ops::Deref;

use crate::{state_machine, Trigger};

/// _`FiniteStateMachine<TState, TEvent>`_ is a trait representing the
/// api of a [state machine]
///
/// ### Generic Data Types
///
/// * _`TState`_ - represents the states of a state machine
/// * _`TEvent`_ - represents the transition events of a state machine
///
/// ### `Deref<Target=TState>`
///
/// `FiniteStateMachine` implements `Deref` with a target of type of `TState`,
/// de-referencing to the current state
///
/// [state machine]: https://en.wikipedia.org/wiki/Finite-state_machine
pub trait FiniteStateMachine<TState, TEvent>: Deref<Target=TState> {
    /// Clears all triggers for this instance of a state machine
    ///
    /// _* does not impact the triggers of the shared_ [`StateMachineDefinition`]
    /// _instance_
    ///
    /// [`StateMachineDefinition`]: crate::StateMachineDefinition
    fn clear_triggers(&mut self);

    /// Gets the current state of the state machine
    ///
    fn current_state(&self) -> TState;

    /// Transitions the state machine from the current state to the next
    /// state based on the defined event transition edge
    ///
    /// # Arguments
    ///
    /// * _`event`_ - event triggering the transition
    ///
    /// _* see_ [`StateError`] _for details on the possible errors that can occur_
    ///
    /// # Results
    ///
    /// Returns the new state of the state machine after the transition event
    ///
    /// # Errors
    ///
    /// Returns a [`StateError`] if there are any errors transitioning on event
    ///
    /// [`StateError`]: state_machine::StateError
    fn event(&mut self, event: TEvent) -> state_machine::Result<TState, TState, TEvent>;

    /// Indicates if the state machine has [cycles]
    ///
    /// # Results
    ///
    /// Returns `None` if a check has not been implemented, _default behavior_,
    /// otherwise it returns a value `Some(true|false)`
    ///
    /// [cycles]: https://en.wikipedia.org/wiki/Cycle_(graph_theory)
    fn has_cycles(&mut self) -> Option<bool> {
        None
    }

    /// Checks if the current state of the state machine is an end state
    ///
    /// # Results
    ///
    /// Returns `true` if the state machine has ended, `false` otherwise
    fn is_end(&self) -> bool;

    /// Checks if the state machine has started
    ///
    /// # Results
    ///
    /// Returns `true` if the state machine has started, `false` otherwise
    fn is_started(&self) -> bool;

    /// Defines new triggers for transitions of a collection of states
    ///
    /// # Arguments
    ///
    /// * _`triggers`_ - a collection of (state, [`Trigger`] collection) pairs
    ///
    /// _* overrides the current existing triggers, if any exist_<br>
    /// _** does not impact the triggers of the shared_ [`StateMachineDefinition`]
    /// _instance_
    ///
    /// [`Trigger`]: Trigger
    /// [`StateMachineDefinition`]: crate::StateMachineDefinition
    // todo: this needs to validate triggers were not defined for end states and only valid states
    fn new_triggers(&mut self, triggers: Vec<(TState, Vec<Trigger<TState, TEvent>>)>);

    /// Gets all of the valid transition edges of the state machine
    ///
    /// # Results
    ///
    /// Returns an `Iterator<Item=(&'a TEvent, &'a TState)>`, which are pairs of event
    /// transition edges for the current state
    fn next_states<'a>(&'a self) -> Box<dyn Iterator<Item=(&'a TEvent, &'a TState)> + 'a>;

    /// Resets the state machine to it's initial state
    ///
    /// # Results
    ///
    /// Returns the prior state of the state machine before it was reset
    fn reset(&mut self) -> TState;

    /// Starts the state machine
    ///
    /// # Results
    ///
    /// Returns the new state of the state machine
    ///
    /// # Errors
    ///
    /// Returns a [`StateError`] if there are any errors starting the state machine
    ///
    /// _* see_ [`StateError`] _for details on the possible errors that can occur_
    ///
    /// [`StateError`]: state_machine::StateError
    fn start(&mut self, event: TEvent) -> state_machine::Result<TState, TState, TEvent>;
}
