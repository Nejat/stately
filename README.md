# Stately

Initial poc of finite-state-machine defined with an ergonomic builder and soon with `derive` or custom `dsl`.

## Road Map

*  `Core` crate features; 


* [x] ~~Initial implementation~~
* [ ] Add `build` validation logic
* [ ] Add state definition reflection from metadata
* [ ] Refactor; separate out state machine definite, into singleton, from actual state instance
* [ ] Move trigger implementation to state instance
* [ ] Refactor; apply stricter clippy rules; a.k.a "Clippify" project
* [ ] Add unit tests
* [ ] Rust Docs
* [ ] Add more examples


*  `Feature` gated expansion;


* [ ] Support for embedded state machine
* [ ] Implement derive macro configuration
* [ ] Implement expression macro dsl
* [ ] Generate [Mermaid](https://mermaid.js.org/syntax/stateDiagram.html) diagrams
