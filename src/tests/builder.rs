use std::fmt::{Display, Formatter};

use Event::{Cycle, Next, Start, Stop};
use State::{Initial, Loop, Started, Stopped};

use crate::builder::BuilderError::{
    RedefinedInitialState, StateAlreadyDefined, TransitionAlreadyDefined, ValidationError,
};
use crate::prelude::*;

#[test]
fn given_a_start_end_state_it_should_be_possible_to_define_multiple_triggers() {
    let sut = StateMachineBuilder::new()
        .add_start_end_state(Stop, Stopped).unwrap()
            .trigger(unreachable_placeholder_trigger)
            .final_trigger(unreachable_placeholder_trigger)
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_start_end_state_it_should_be_possible_to_define_no_triggers() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .no_triggers()
            .transition_on(Next, Started).unwrap()
            .final_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .only_trigger(unreachable_placeholder_trigger)
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_start_end_state_it_should_be_possible_to_define_only_one_trigger() {
    let sut = StateMachineBuilder::new()
        .add_start_end_state(Stop, Stopped).unwrap()
            .only_trigger(unreachable_placeholder_trigger)
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_start_state_it_should_be_possible_to_define_multiple_transitions() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .transition_on(Stop, Stopped).unwrap()
            .final_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .no_triggers()
            .only_transition_on(Next, Started).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_start_state_it_should_be_possible_to_define_multiple_triggers() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .trigger(unreachable_placeholder_trigger)
            .final_trigger(unreachable_placeholder_trigger)
            .only_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_start_state_it_should_be_possible_to_define_no_triggers() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_start_state_it_should_be_possible_to_define_only_one_transition() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_start_state_it_should_be_possible_to_define_only_one_trigger() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .only_trigger(unreachable_placeholder_trigger)
            .only_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_state_it_should_be_possible_to_define_multiple_transitions() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .no_triggers()
            .transition_on(Next, Started).unwrap()
            .final_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_state_it_should_be_possible_to_define_multiple_triggers() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .trigger(unreachable_placeholder_trigger)
            .final_trigger(unreachable_placeholder_trigger)
            .transition_on(Next, Started).unwrap()
            .final_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_state_it_should_be_possible_to_define_no_triggers() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .no_triggers()
            .transition_on(Next, Started).unwrap()
            .final_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_state_it_should_be_possible_to_define_only_one_transition() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .transition_on(Stop, Stopped).unwrap()
            .final_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .no_triggers()
            .only_transition_on(Next, Started).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_a_state_it_should_be_possible_to_define_only_one_trigger() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .only_trigger(unreachable_placeholder_trigger)
            .transition_on(Next, Started).unwrap()
            .final_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_an_end_state_it_should_be_possible_to_define_multiple_triggers() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .no_triggers()
            .transition_on(Next, Started).unwrap()
            .final_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .trigger(unreachable_placeholder_trigger)
            .final_trigger(unreachable_placeholder_trigger)
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_an_end_state_it_should_be_possible_to_define_no_triggers() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .no_triggers()
            .transition_on(Next, Started).unwrap()
            .final_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_an_end_state_it_should_be_possible_to_define_only_one_trigger() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .no_triggers()
            .transition_on(Next, Started).unwrap()
            .final_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .only_trigger(unreachable_placeholder_trigger)
        .build();

    assert!(matches!(sut, Ok(_)));
}

#[test]
fn given_an_initial_state_definition_should_not_build_fsm() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Stop, Stopped).unwrap()
        .add_state(Initial);

    assert!(matches!(sut, Err(RedefinedInitialState)));
}

#[test]
fn given_an_undefined_state_should_not_build_fsm() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Next, Loop).unwrap()
        .build();

    let expected_undefined = vec![Loop];

    assert!(matches!(
        sut,
        Err(ValidationError { undefined_states, unreachable })
            if undefined_states.iter().all(|itm| expected_undefined.contains(itm)) &&
                unreachable.is_empty()
    ));
}

#[test]
fn given_an_unreachable_state_should_not_build_fsm() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Cycle, Loop).unwrap()
        .add_state(Loop).unwrap()
            .no_triggers()
            .only_transition_on(Next, Started).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    let expected_unreachable = vec![Stopped];

    assert!(matches!(
        sut,
        Err(ValidationError { undefined_states, unreachable })
            if unreachable.iter().all(|itm| expected_unreachable.contains(itm)) &&
                undefined_states.is_empty()
    ));
}

#[test]
fn given_duplicate_states_should_not_build_fsm() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .add_end_state(Stopped);

    assert!(matches!(sut, Err(StateAlreadyDefined { state: Stopped })));
}

#[test]
fn given_duplicate_transitions_should_not_build_fsm() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .transition_on(Stop, Stopped).unwrap()
            .final_transition_on(Stop, Started);

    assert!(matches!(sut, Err(TransitionAlreadyDefined { event: Stop, existing: Stopped })));
}

#[test]
fn given_no_cycles_an_fsm_should_define_an_end_state() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));

    let sut = sut.unwrap();

    assert!(!sut.create().has_cycles().unwrap());

    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Next, Stopped).unwrap()
        .add_start_end_state(Stop, Stopped).unwrap()
            .no_triggers()
        .build();

    assert!(matches!(sut, Ok(_)));

    let sut = sut.unwrap();

    assert!(!sut.create().has_cycles().unwrap());
}

#[test]
fn given_no_end_states_fsm_should_define_a_cycle() {
    let sut = StateMachineBuilder::new()
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Next, Loop).unwrap()
        .add_state(Loop).unwrap()
            .no_triggers()
            .only_transition_on(Next, Started).unwrap()
        .build();

    assert!(matches!(sut, Ok(_)));

    let sut = sut.unwrap();

    assert!(sut.create().has_cycles().unwrap());
}

fn unreachable_placeholder_trigger(_event: Event, _prior: State, _next: State) {
    unreachable!();
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Event {
    Cycle,
    Next,
    Start,
    Stop,
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
pub enum State {
    #[default]
    Initial,
    Started,
    Loop,
    Stopped,
}

impl Display for State {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!("{}", match self {
            Initial => "Initial",
            Started => "Started",
            Loop => "Loop",
            Stopped => "Stopped"
        }))
    }
}