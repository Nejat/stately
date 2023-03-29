use stately::builder::{
    BuilderState, InitialState, Result, StateMachineBuilder, TransitionState, TriggerState,
};

fn main() -> Result<(), State, Event> {
    let _fsm = StateMachineBuilder::new()
        .add_start_state(Event::Start, State::Started).unwrap()
            .no_triggers()
            .only_transition_on(Event::Next, State::Started).unwrap()
        .add_end_state(State::Stopped).unwrap()
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
    Stopped,
}
