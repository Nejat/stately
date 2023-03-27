use stately::builder::{InitialState, Result, StateMachineBuilder, TriggerState};

fn main() -> Result<(), State, Event> {
    let _fsm = <StateMachineBuilder<State, Event>>::new(State::Initial)
        .add_start_state(Event::Start, State::Started).unwrap()
            .no_triggers()
        .build()?;

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Event {
    Start
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
enum State {
    #[default]
    Initial,
    Started,
}
