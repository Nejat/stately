use stately::builder::{Result, StateMachineBuilder};

fn main() -> Result<(), State, Event> {
    let _fsm = StateMachineBuilder::new()
        .add_state(State::Step42);

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Event {}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash)]
enum State {
    #[default]
    Initial,
    Step42
}
