# oyez-api-wrapper-rust

Rust CLI program for utilizing the Oyez API.

This can be run by importing `case.rs` by using `mod case;`.

Initialize a case with:

```rust
let local_case = case::CourtCase {
    docket_num,
    year,
    json: proper_json.clone(),
};
```

From here, you can get various data back from the case object by calling the functions that exist.

-----------------------
The owner of this repo has no relation with [oyez.org](oyez.org)!
