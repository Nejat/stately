use stately::prelude::*;

use crate::Event::*;
use crate::State::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd)]
pub enum Event {
    Done,
    Loop,
    Next,
    Skip,
    Start,
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, PartialOrd)]
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

fn main() -> Result<(), BuilderError<State, Event>> {
    let email_state_machine = StateMachineBuilder::new(Initial)
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
            .only_transition_on(Loop, B1)?
        .add_state(C)?
            .only_trigger(transitioned)
            .only_transition_on(Next, D)?
        .add_state(D)?
            .only_trigger(transitioned)
            .transition_on(Next, F)?
            .final_transition_on(Done, G)?
        .add_end_state(F)?
            .only_trigger(completed)
        .add_end_state(G)?
            .only_trigger(completed)
        .add_start_state(Skip, H)?
            .also_end_state()
            .only_trigger(start_completed)
        .build()?;

    let mut email_state = email_state_machine.create();

    assert!(email_state.has_cycles().expect("detect cycles should be implemented"));

    demonstrate_state_machine(&mut email_state, Triggers::Yes);

    email_state.reset();
    email_state.clear_triggers();

    demonstrate_state_machine(&mut email_state, Triggers::No);

    println!("re-ran demonstration without triggers");

    email_state.reset();
    email_state.new_triggers(
        vec![
            (F, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
            (H, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
            (G, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
            (E, vec![Box::new(|_, _, state| println!("◉ {state:?} ●"))]),
        ]
    );

    demonstrate_state_machine(&mut email_state, Triggers::DontCheck);

    return Ok(());

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

#[derive(Eq, PartialEq)]
enum Triggers {
    Yes,
    No,
    DontCheck
}

fn demonstrate_state_machine(
    email_state: &mut impl FiniteStateMachine<State, Event>,
    triggers: Triggers,
)
{
    let current_state = email_state.current_state();

    assert_eq!(Initial, current_state);

    let expected = vec![
        (&Start, &A),
        (&Skip, &H),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.start(Start);

    assert_eq!(A, current_state);
    assert!(email_state.is_start());

    if triggers != Triggers::DontCheck {
        assert_eq!(matches!(triggers, Triggers::Yes), email_state.has_trigger());
    }

    let expected = vec![
        (&Next, &B),
        (&Done, &E),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Next);

    assert_eq!(B, current_state);

    if triggers != Triggers::DontCheck {
        assert_eq!(matches!(triggers, Triggers::Yes), email_state.has_trigger());
    }

    let expected = vec![
        (&Next, &C),
        (&Done, &G),
        (&Loop, &B1),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Next);

    assert_eq!(C, current_state);

    if triggers != Triggers::DontCheck {
        assert_eq!(matches!(triggers, Triggers::Yes), email_state.has_trigger());
    }

    let expected = vec![(&Next, &D)];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Next);

    assert_eq!(D, current_state);

    if triggers != Triggers::DontCheck {
        assert_eq!(matches!(triggers, Triggers::Yes), email_state.has_trigger());
    }

    let expected = vec![
        (&Next, &F),
        (&Done, &G),
    ];

    assert_next_states(&expected, email_state);

    let current_state = email_state.event(Next);

    assert_eq!(F, current_state);
    assert!(email_state.is_end());
    assert_eq!(email_state.next_states().count(), 0);

    if triggers != Triggers::DontCheck {
        assert_eq!(matches!(triggers, Triggers::Yes), email_state.has_trigger());
    }

    let current_state = email_state.start(Skip);

    assert_eq!(H, current_state);
    assert!(email_state.is_start());
    assert!(email_state.is_end());
    assert_eq!(email_state.next_states().count(), 0);

    if triggers != Triggers::DontCheck {
        assert_eq!(matches!(triggers, Triggers::Yes), email_state.has_trigger());
    }

    fn assert_next_states(
        expected: &[(&Event, &State)],
        sut: &impl FiniteStateMachine<State, Event>,
    ) {
        assert!(sut.next_states().all(|itm| expected.contains(&itm)));
    }
}
