use std::fmt::{Display, Formatter};

use thiserror::Error;

use stately::{builder, state_machine};
use stately::builder::BuilderError;
use stately::prelude::*;
use stately::state_machine::StateError;

use crate::Event::{Done, Loop, Next, Skip, Start};
use crate::State::{A, B, B1, C, D, E, F, G, H, Initial};

type Result<T> = std::result::Result<T, ExampleError>;
type BuilderResult = builder::Result<StateMachineDefinition<State, Event>, State, Event>;

fn main() -> Result<()> {
    let state_machine = state_machine().map_err(ExampleError::Build)?;
    let mut state = state_machine.create();

    assert!(state.has_cycles().expect("has cycles should have a value"));

    demonstrate_state_machine(&mut state).map_err(ExampleError::StateMachine)?;

    println!("re-run, hey! hey! hey!, demonstration without triggers");

    state.reset();
    state.clear_triggers();

    demonstrate_state_machine(&mut state).map_err(ExampleError::StateMachine)?;

    println!("re-run, hey! hey! hey!, demonstration with custom triggers");

    state.reset();
    state.new_triggers(
        vec![
            (F, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
            (H, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
            (G, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
            (E, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
        ]
    );

    demonstrate_state_machine(&mut state).map_err(ExampleError::StateMachine)
}

fn state_machine() -> BuilderResult {
    return StateMachineBuilder::new()
        .add_start_state(Start, A)?
            .only_trigger(started)
            .transition_on(Done, E)?
            .final_transition_on(Next, B)?
        .add_end_state(E)?
            .only_trigger(completed)
        .add_state(B)?
            .only_trigger(transitioned)
            .transition_on(Next, C)?
            .transition_on(Loop, B1)?
            .final_transition_on(Done, G)?
        .add_state(B1)?
            .only_trigger(transitioned)
            .only_transition_on(Next, D)?
        .add_state(C)?
            .only_trigger(transitioned)
            .only_transition_on(Next, D)?
        .add_state(D)?
            .only_trigger(transitioned)
            .transition_on(Next, F)?
            .transition_on(Loop, B)?
            .final_transition_on(Done, G)?
        .add_end_state(F)?
            .only_trigger(completed)
        .add_end_state(G)?
            .only_trigger(completed)
        .add_start_end_state(Skip, H)?
            .only_trigger(start_completed)
        .build();

    fn completed(event: Event, _state: State, status: State) {
        println!(" ━ |{event:?}| → {status:?} ●")
    }

    fn start_completed(event: Event, _state: State, status: State) {
        println!("◉ |{event:?}| → {status:?} ●")
    }

    fn started(event: Event, _prior_state: State, state: State) {
        print!("◉ |{event:?}| → {state:?}")
    }

    fn transitioned(event: Event, _prior_state: State, state: State) {
        print!(" ━ |{event:?}| → {state:?}")
    }
}

fn demonstrate_state_machine(
    email_state: &mut impl FiniteStateMachine<State, Event>,
) -> state_machine::Result<(), State, Event> {
    let current_state = email_state.current_state();

    assert_eq!(Initial, current_state);

    let expected = vec![
        (&Start, &A),
        (&Skip, &H),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.start(Start)?;

    assert_eq!(A, current_state);
    assert!(email_state.is_started());

    let expected = vec![
        (&Next, &B),
        (&Done, &E),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Next)?;

    assert_eq!(B, current_state);

    let expected = vec![
        (&Next, &C),
        (&Done, &G),
        (&Loop, &B1),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Next)?;

    assert_eq!(C, current_state);

    let expected = vec![(&Next, &D)];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Next)?;

    assert_eq!(D, current_state);

    let expected = vec![
        (&Next, &F),
        (&Loop, &B),
        (&Done, &G),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Next)?;

    assert_eq!(F, current_state);
    assert!(email_state.is_end());
    assert_eq!(email_state.next_states().count(), 0);

    return Ok(());

    fn assert_next_states(
        expected: &[(&Event, &State)],
        sut: &impl FiniteStateMachine<State, Event>,
    ) {
        assert!(sut.next_states().all(|itm| expected.contains(&itm)));
    }
}

#[derive(Error, Debug)]
enum ExampleError {
    Build(BuilderError<State, Event>),
    StateMachine(StateError<State, Event>),
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
pub enum Event {
    Done,
    Loop,
    Next,
    Skip,
    Start,
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
enum State {
    #[default]
    Initial,
    A,
    B,
    B1,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Display for State {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!("{}", match self {
            Initial => "Initial",
            A => "A",
            B => "B",
            B1 => "B1",
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            H => "H",
        }))
    }
}
