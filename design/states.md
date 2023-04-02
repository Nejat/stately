# Example of State Machines

## Some email processing system

```mermaid
stateDiagram-v2
    [*] --> Scheduled
    Scheduled --> Canceled
    Scheduled --> Processing
    Processing --> Sent
    Processing --> Failed
    Sent --> Verifying
    Verifying --> Successful
    Verifying --> Failed
    Successful --> [*]
    Sent --> Failed
    Failed --> [*]
    Canceled --> [*]
    [*] --> Invalid
    Invalid --> [*]
```

## Demonstration of State Machine w/Cycles
