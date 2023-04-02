use std::fmt::{Display, Formatter};

use thiserror::Error;

use stately::{builder, state_machine};
use stately::builder::BuilderError;
use stately::prelude::*;
use stately::state_machine::StateError;

use crate::EMailEvent::{Cancel, Fail, InvalidRequest, Process, Schedule, Succeed, Verify};
use crate::EMailState::{
    Canceled, Failed, Initial, Invalid, Processing, Scheduled, Sent, Successful, Verifying,
};

type Result<T> = std::result::Result<T, ExampleError>;
type BuilderResult = builder::Result<StateMachineDefinition<EMailState, EMailEvent>, EMailState, EMailEvent>;

fn main() -> Result<()> {
    let email_state_machine = email_state_machine().map_err(ExampleError::Build)?;
    let mut email_state = email_state_machine.create();

    assert!(!email_state.has_cycles().expect("has cycles should have a value"));

    demonstrate_state_machine(&mut email_state).map_err(ExampleError::StateMachine)?;

    println!("re-run, hey! hey! hey!, demonstration without triggers");

    email_state.reset();
    email_state.clear_triggers();

    demonstrate_state_machine(&mut email_state).map_err(ExampleError::StateMachine)?;

    println!("re-run, hey! hey! hey!, demonstration with custom triggers");

    email_state.reset();
    email_state.new_triggers(
        vec![
            (Successful, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
            (Invalid, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
            (Failed, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
            (Canceled, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
        ]
    )
        .map_err(ExampleError::StateMachine)?;

    demonstrate_state_machine(&mut email_state).map_err(ExampleError::StateMachine)
}

fn email_state_machine() -> BuilderResult {
    return StateMachineBuilder::new()
        .add_start_state(Schedule, Scheduled)?
            .only_trigger(started)
            .transition_on(Cancel, Canceled)?
            .final_transition_on(Process, Processing)?
        .add_end_state(Canceled)?
            .only_trigger(completed)
        .add_state(Processing)?
            .only_trigger(transitioned)
            .transition_on(Succeed, Sent)?
            .final_transition_on(Fail, Failed)?
        .add_state(Sent)?
            .only_trigger(transitioned)
            .only_transition_on(Verify, Verifying)?
        .add_state(Verifying)?
            .only_trigger(transitioned)
            .transition_on(Succeed, Successful)?
            .final_transition_on(Fail, Failed)?
        .add_end_state(Successful)?
            .only_trigger(completed)
        .add_end_state(Failed)?
            .only_trigger(completed)
        .add_start_end_state(InvalidRequest, Invalid)?
            .only_trigger(start_completed)
        .build();


    fn completed(event: EMailEvent, _state: EMailState, status: EMailState) {
        println!(" ━ |{event:?}| → {status:?} ●")
    }

    fn start_completed(event: EMailEvent, _state: EMailState, status: EMailState) {
        println!("◉ |{event:?}| → {status:?} ●")
    }

    fn started(event: EMailEvent, _prior_state: EMailState, state: EMailState) {
        print!("◉ |{event:?}| → {state:?}")
    }

    fn transitioned(event: EMailEvent, _prior_state: EMailState, state: EMailState) {
        print!(" ━ |{event:?}| → {state:?}")
    }
}

fn demonstrate_state_machine(
    email_state: &mut impl FiniteStateMachine<EMailState, EMailEvent>,
) -> state_machine::Result<(), EMailState, EMailEvent>
{
    let current_state = email_state.current_state();

    assert_eq!(Initial, current_state);

    let expected = vec![
        (&Schedule, &Scheduled),
        (&InvalidRequest, &Invalid),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.start(Schedule)?;

    assert_eq!(Scheduled, current_state);
    assert!(email_state.is_started());

    let expected = vec![
        (&Process, &Processing),
        (&Cancel, &Canceled),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Process)?;

    assert_eq!(Processing, current_state);

    let expected = vec![
        (&Succeed, &Sent),
        (&Fail, &Failed),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Succeed)?;

    assert_eq!(Sent, current_state);

    let expected = vec![(&Verify, &Verifying)];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Verify)?;

    assert_eq!(Verifying, current_state);

    let expected = vec![
        (&Succeed, &Successful),
        (&Fail, &Failed),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Succeed)?;

    assert_eq!(Successful, current_state);
    assert!(email_state.is_end());
    assert_eq!(email_state.next_states().count(), 0);

    return Ok(());

    fn assert_next_states(
        expected: &[(&EMailEvent, &EMailState)],
        sut: &impl FiniteStateMachine<EMailState, EMailEvent>,
    ) {
        assert!(sut.next_states().all(|itm| expected.contains(&itm)));
    }
}

#[derive(Error, Debug)]
enum ExampleError {
    Build(BuilderError<EMailState, EMailEvent>),
    StateMachine(StateError<EMailState, EMailEvent>),
}

impl Display for ExampleError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExampleError::Build(err) =>
                fmt.write_fmt(format_args!("Build: {err:?}")),
            ExampleError::StateMachine(err) =>
                fmt.write_fmt(format_args!("FSM: {err:?}")),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum EMailEvent {
    Cancel,
    InvalidRequest,
    Process,
    Fail,
    Succeed,
    Schedule,
    Verify,
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
enum EMailState {
    Canceled,
    Failed,
    #[default]
    Initial,
    Invalid,
    Processing,
    Scheduled,
    Sent,
    Successful,
    Verifying,
}

impl Display for EMailState {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!("{}", match self {
            Initial => "Initial",
            Canceled => "Canceled",
            Failed => "Failed",
            Invalid => "Invalid",
            Processing => "Processing",
            Scheduled => "Scheduled",
            Sent => "Sent",
            Successful => "Successful",
            Verifying => "Verifying",
        }))
    }
}
