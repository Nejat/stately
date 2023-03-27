use stately::builder::{InitialState, Result, StateMachineBuilder};

fn main() -> Result<(), State, Event> {
    let _fsm = <StateMachineBuilder<State, Event>>::new(State::Initial)
        .add_start_state(Event::Start, State::Started).unwrap()
            .only_transition_on(Event::Next, State::Started).unwrap()
        .build()?;

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Event {
    Start,
    Next,
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
enum State {
    #[default]
    Initial,
    Started,
}
