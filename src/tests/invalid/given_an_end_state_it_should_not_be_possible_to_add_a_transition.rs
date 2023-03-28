use stately::builder::{InitialState, Result, StateMachineBuilder, TriggerState};

fn main() -> Result<(), State, Event> {
    let _fsm = StateMachineBuilder::new()
        .add_start_state(Event::Start, State::Started).unwrap()
            .no_triggers()
            .only_transition(Event::Stop, State::Stopped).unwrap()
        .add_end_state(State::Stopped).unwrap()
            .no_triggers()
            .only_transition(Event::Next, State::Never).unwrap()
        .build()?;

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Event {
    Next,
    Start,
    Stop,
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
enum State {
    #[default]
    Initial,
    Never,
    Started,
    Stopped,
}
