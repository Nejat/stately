use stately::prelude::*;

use crate::EmailEvent::*;
use crate::EMailState::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd)]
pub enum EmailEvent {
    Cancel,
    InvalidRequest,
    Process,
    Fail,
    Succeed,
    Schedule,
    Verify,
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, PartialOrd)]
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

fn main() -> Result<(), BuilderError<EMailState, EmailEvent>> {
    let mut email_state = StateMachineBuilder::new(Initial)
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
        .add_start_state(InvalidRequest, Invalid)?
        .also_end_state()
        .only_trigger(start_completed)
        .build()?;

    let current_state = *email_state;

    assert_eq!(Initial, current_state);

    let expected = vec![
        (&Schedule, &Scheduled),
        (&InvalidRequest, &Invalid),
    ];

    assert_next_states(&expected, &email_state);

    let current_state = email_state.start(Schedule);

    assert_eq!(Scheduled, current_state);
    assert!(email_state.is_start());
    assert!(email_state.has_trigger());

    let expected = vec![
        (&Process, &Processing),
        (&Cancel, &Canceled),
    ];

    assert_next_states(&expected, &email_state);

    let current_state = email_state.event(Process);

    assert_eq!(Processing, current_state);
    assert!(email_state.has_trigger());

    let expected = vec![
        (&Succeed, &Sent),
        (&Fail, &Failed),
    ];

    assert_next_states(&expected, &email_state);

    let current_state = email_state.event(Succeed);

    assert_eq!(Sent, current_state);
    assert!(email_state.has_trigger());

    let expected = vec![(&Verify, &Verifying)];

    assert_next_states(&expected, &email_state);

    let current_state = email_state.event(Verify);

    assert_eq!(Verifying, current_state);
    assert!(email_state.has_trigger());

    let expected = vec![
        (&Succeed, &Successful),
        (&Fail, &Failed),
    ];

    assert_next_states(&expected, &email_state);

    let current_state = email_state.event(Succeed);

    assert_eq!(Successful, current_state);
    assert!(email_state.is_end());
    assert!(email_state.has_trigger());
    assert_eq!(email_state.next_states().count(), 0);

    let current_state = email_state.start(InvalidRequest);

    assert_eq!(Invalid, current_state);
    assert!(email_state.is_start());
    assert!(email_state.is_end());
    assert!(email_state.has_trigger());
    assert_eq!(email_state.next_states().count(), 0);

    return Ok(());

    fn completed(event: EmailEvent, _state: EMailState, status: EMailState) {
        println!(" ━ |{event:?}| → {status:?} ●")
    }

    fn start_completed(event: EmailEvent, _state: EMailState, status: EMailState) {
        println!("◉ |{event:?}| → {status:?} ●")
    }

    fn started(event: EmailEvent, _prior_state: EMailState, state: EMailState) {
        print!("◉ |{event:?}| → {state:?}")
    }

    fn transitioned(event: EmailEvent, _prior_state: EMailState, state: EMailState) {
        print!(" ━ |{event:?}| → {state:?}")
    }

    fn assert_next_states(
        expected: &[(&EmailEvent, &EMailState)],
        sut: &StateMachine<EMailState, EmailEvent>,
    ) {
        assert!(sut.next_states().all(|itm| expected.contains(&itm)));
    }
}
