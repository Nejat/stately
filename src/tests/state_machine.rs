use std::cell::RefCell;
use std::rc::Rc;

use Event::{Cycle, Next, Start, Stop};
use State::{Initial, Loop, Started, Stopped};

use crate::prelude::*;
use crate::state_machine::StateError::{AlreadyStarted, EndState, InvalidTransition, NotAStartEvent, NotStarted};

const DEFINED_TRIGGERS: bool = true;
const CUSTOM_TRIGGERS: bool = false;
const NO_TRIGGER: &str = "did not expect trigger to fire";

#[test]
fn given_a_new_fsm_should_not_transition_without_starting() {
    let (mut sut, _) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    let transition = sut.event(Stop);

    assert!(matches!(transition, Err(NotStarted)));
}

#[test]
fn given_a_new_fsm_should_not_start_with_incorrect_event() {
    let (mut sut, _) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    let transition = sut.start(Cycle);

    assert!(matches!(transition, Err(NotAStartEvent { event: Cycle })));
}

#[test]
fn given_a_started_fsm_it_should_not_start_again() {
    let (mut sut, _) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    let transition = sut.start(Start);

    assert!(matches!(transition, Err(AlreadyStarted)));
}

#[test]
fn given_an_fsm_a_transition_on_an_end_state_should_not_transition() {
    let (mut sut, _triggered) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    sut.event(Stop).unwrap();

    assert_eq!(Stopped, sut.current_state());
    assert!(sut.is_end());

    assert!(matches!(sut.event(Start), Err(EndState { end: Stopped })));
}

#[test]
fn given_an_fsm_an_event_should_transition() {
    let (mut sut, triggered) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    sut.event(Stop).unwrap();

    assert_eq!(Stopped, sut.current_state());
    assert!(sut.is_end());

    let triggered = *triggered.borrow();

    assert_eq!(2, triggered);
}

#[test]
fn given_an_fsm_an_incorrect_event_should_not_transition() {
    let (mut sut, _triggered) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    assert!(
        matches!(sut.event(Start),
            Err(InvalidTransition { event: Start, current_state: Started }))
    );
}

#[test]
fn given_an_fsm_clearing_trigger_should_not_trigger() {
    let (mut sut, triggered) = subject_under_test(CUSTOM_TRIGGERS);

    sut.clear_triggers();

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    sut.event(Cycle).unwrap();

    assert_eq!(Loop, sut.current_state());

    sut.event(Next).unwrap();

    assert_eq!(Started, sut.current_state());

    sut.event(Cycle).unwrap();

    assert_eq!(Loop, sut.current_state());

    sut.event(Next).unwrap();

    assert_eq!(Started, sut.current_state());

    sut.event(Stop).unwrap();

    assert_eq!(Stopped, sut.current_state());
    assert!(sut.is_end());

    let triggered = *triggered.borrow();

    assert_eq!(0, triggered);
}

#[test]
fn given_an_fsm_it_should_be_possible_to_end() {
    let (mut sut, _) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    sut.event(Cycle).unwrap();

    assert_eq!(Loop, sut.current_state());

    sut.event(Next).unwrap();

    assert_eq!(Started, sut.current_state());

    sut.event(Stop).unwrap();

    assert_eq!(Stopped, sut.current_state());
    assert!(sut.is_end());
}

#[test]
fn given_an_fsm_it_should_start() {
    let (mut sut, triggered) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    let triggered = *triggered.borrow();

    assert_eq!(1, triggered);
}

#[test]
fn given_an_fsm_resetting_it_should_start_again() {
    let (mut sut, triggered) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    sut.event(Stop).unwrap();

    assert_eq!(Stopped, sut.current_state());
    assert!(sut.is_end());

    sut.reset();

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    sut.event(Stop).unwrap();

    assert_eq!(Stopped, sut.current_state());
    assert!(sut.is_end());

    let triggered = *triggered.borrow();

    assert_eq!(4, triggered);
}

#[test]
fn given_an_fsm_with_custom_triggers_events_should_trigger() {
    let custom_triggered = Rc::new(RefCell::new(false));

    let (mut sut, triggered) = subject_under_test(CUSTOM_TRIGGERS);

    sut.new_triggers(vec![
        (Started, vec![
            Box::new({
                let custom_triggered = custom_triggered.clone();

                move |event, previous, next| {
                    assert_eq!(Start, event);
                    assert_eq!(Initial, previous);
                    assert_eq!(Started, next);

                    *custom_triggered.borrow_mut() = true;
                }
            })
        ])
    ]);

    sut.start(Start).unwrap();

    assert!(*custom_triggered.borrow());

    let triggered = *triggered.borrow();

    assert_eq!(0, triggered);

    let custom_triggered = *custom_triggered.borrow();

    assert!(custom_triggered);
}

#[test]
fn given_an_fsm_with_cycles_it_should_loop() {
    let (mut sut, triggered) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    sut.event(Cycle).unwrap();

    assert_eq!(Loop, sut.current_state());

    sut.event(Next).unwrap();

    assert_eq!(Started, sut.current_state());

    sut.event(Cycle).unwrap();

    assert_eq!(Loop, sut.current_state());

    sut.event(Next).unwrap();

    assert_eq!(Started, sut.current_state());

    sut.event(Stop).unwrap();

    assert_eq!(Stopped, sut.current_state());
    assert!(sut.is_end());

    let triggered = *triggered.borrow();

    assert_eq!(6, triggered);
}

#[test]
fn given_an_fsm_with_multiple_triggers_events_should_trigger() {
    let (mut sut, triggered) = subject_under_test_multiple_triggers(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    sut.event(Cycle).unwrap();

    assert_eq!(Loop, sut.current_state());

    sut.event(Next).unwrap();

    assert_eq!(Started, sut.current_state());

    sut.event(Stop).unwrap();

    assert_eq!(Stopped, sut.current_state());
    assert!(sut.is_end());

    let triggered = *triggered.borrow();

    assert_eq!(8, triggered);
}

#[test]
fn given_an_fsm_with_triggers_events_should_trigger() {
    let (mut sut, triggered) = subject_under_test(DEFINED_TRIGGERS);

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    sut.event(Cycle).unwrap();

    assert_eq!(Loop, sut.current_state());

    sut.event(Next).unwrap();

    assert_eq!(Started, sut.current_state());

    sut.event(Stop).unwrap();

    assert_eq!(Stopped, sut.current_state());
    assert!(sut.is_end());

    let triggered = *triggered.borrow();

    assert_eq!(4, triggered);
}

#[test]
fn given_an_fsm_without_cycles_it_should_end() {
    let mut sut = subject_under_test_without_cycles();

    assert!(!sut.is_started());

    sut.start(Start).unwrap();

    assert_eq!(Started, sut.current_state());
    assert!(sut.is_started());

    sut.event(Stop).unwrap();

    assert_eq!(Stopped, sut.current_state());
    assert!(sut.is_end());
}

fn subject_under_test(
    expect_trigger: bool
) -> (impl FiniteStateMachine<State, Event>, Rc<RefCell<u32>>) {
    let triggered = Rc::new(RefCell::new(0));

    let sut = <StateMachineBuilder<State, Event>>::new(Initial)
        .add_start_state(Start, Started).unwrap()
            .only_trigger(trigger_start(triggered.clone(), expect_trigger))
            .transition_on(Cycle, Loop).unwrap()
            .final_transition_on(Stop, Stopped).unwrap()
        .add_state(Loop).unwrap()
            .only_trigger(trigger_state(triggered.clone(), expect_trigger))
            .only_transition_on(Next, Started).unwrap()
        .add_end_state(Stopped).unwrap()
            .only_trigger(trigger_end(triggered.clone(), expect_trigger))
        .build().unwrap()
        .create();

    (sut, triggered)
}

fn subject_under_test_multiple_triggers(
    expect_trigger: bool
) -> (impl FiniteStateMachine<State, Event>, Rc<RefCell<u32>>) {
    let triggered = Rc::new(RefCell::new(0));

    let sut = <StateMachineBuilder<State, Event>>::new(Initial)
        .add_start_state(Start, Started).unwrap()
            .trigger(trigger_start(triggered.clone(), expect_trigger))
            .final_trigger(trigger_start(triggered.clone(), expect_trigger))
            .transition_on(Cycle, Loop).unwrap()
            .final_transition_on(Stop, Stopped).unwrap()
        .add_state(Loop).unwrap()
            .trigger(trigger_state(triggered.clone(), expect_trigger))
            .final_trigger(trigger_state(triggered.clone(), expect_trigger))
            .only_transition_on(Next, Started).unwrap()
        .add_end_state(Stopped).unwrap()
            .trigger(trigger_end(triggered.clone(), expect_trigger))
            .final_trigger(trigger_end(triggered.clone(), expect_trigger))
        .build().unwrap()
        .create();

    (sut, triggered)
}

fn subject_under_test_without_cycles() -> impl FiniteStateMachine<State, Event> {
    <StateMachineBuilder<State, Event>>::new(Initial)
        .add_start_state(Start, Started).unwrap()
            .no_triggers()
            .only_transition_on(Stop, Stopped).unwrap()
        .add_end_state(Stopped).unwrap()
            .no_triggers()
        .build().unwrap()
        .create()
}

fn trigger_end(triggered: Rc<RefCell<u32>>, expect_trigger: bool) -> impl Fn(Event, State, State) {
    move |event, previous, next| {
        assert!(expect_trigger, "{}", NO_TRIGGER);

        assert_eq!(Stop, event);
        assert_eq!(Started, previous);
        assert_eq!(Stopped, next);

        *triggered.borrow_mut() += 1;
    }
}

fn trigger_start(triggered: Rc<RefCell<u32>>, expect_trigger: bool) -> impl Fn(Event, State, State) {
    move |event, previous, next| {
        assert!(expect_trigger, "{}", NO_TRIGGER);

        match event {
            Next => assert_eq!(Loop, previous),
            Start => assert_eq!(Initial, previous),
            Cycle | Stop => unreachable!()
        }

        assert_eq!(Started, next);

        *triggered.borrow_mut() += 1;
    }
}

fn trigger_state(triggered: Rc<RefCell<u32>>, expect_trigger: bool) -> impl Fn(Event, State, State) {
    move |event, previous, next| {
        assert!(expect_trigger, "{}", NO_TRIGGER);

        assert_eq!(Cycle, event);
        assert_eq!(Started, previous);
        assert_eq!(Loop, next);

        *triggered.borrow_mut() += 1;
    }
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
