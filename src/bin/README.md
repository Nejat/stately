# EMails

State machine for processing emails.

```mermaid
flowchart LR
    A(((Start))) -- Schedule --> B(Scheduled)
    B -- Process --> E(Processing)
    E -- Succeed --> F(Sent)
    F -- Verify --> H(Verifying)
    H -- Succeed --> I(Successful)
    H -- Fail --> G
    E -- Fail --> G(Failed)
    B -- Cancel --> D(Canceled)
    A(((Start))) -- Invalid Request --> C(Invalid)
    I --> Z((End))
    G --> Z
    D --> Z
    C --> Z
```

## Example

Run example using

```shell
cargo run --package stately --bin emails
```

### Example Output
Successful conclusion
```
◉ |Schedule| → Scheduled ━ |Process| → Processing ━ |Succeed| → Sent ━ |Verify| → Verifying ━ |Succeed| → Successful ●
```

Invalid conclusion
```
◉ |InvalidRequest| → Invalid ●
```