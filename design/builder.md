**State Machine Builder - States**

This state machine of a state machine builder encapsulates the build rules.

Each state is an implementation `trait`[<sup>*</sup>]() which provides partial[<sup>**</sup>]() compile time validation.

```mermaid
flowchart 
    FSM(((FSM<Br>Builder))) -- Create --> IS(Initial)
    IS -- Add Start End --> ETS(End Trigger)
    ETS -- Trigger --> METS(End Triggers)
    METS -- Final Trigger --> BS(Build)
    BS -- Build --> SMD((State<br>Machine))
    IS -- Add Start --> TS(Trigger)
    TS -- Trigger --> MTS(Triggers)
    MTS -- Final Trigger --> NS(Transition)
    NS -- Transition On --> MNS(Transitions)
    METS -- Trigger --> METS
    BS -- Add End<br>Add Start End --> ETS
    BS -- Add Start<br>Add State --> TS
    TS -- No Triggers<br>Only Trigger --> NS
    MTS -- Trigger --> MTS
    NS -- Only Transition --> BS
    MNS -- Transition On --> MNS
    MNS -- Final Transition On --> BS
```

_* see the [`build_states`](../src/builder/build_states) for all the traits that are used to implement this state diagram_

_** additional [runtime validations](../src/builder/build_states/builder.rs) are required to ensure a valid state machine_