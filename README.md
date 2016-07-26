# coded
Code activity monitor

## Rust language notes

_We're still learning Rust. Leave your learnings here._

### Put ~/.cargo/bin on your PATH

Rust tools installed via `cargo install` need to be available. Ex: `cargo-fmt`

### The `tests` dir is for integration tests

Cargo compiles all tests in */tests* as a separate crate. This guarantees
that they're integration tests, because they _must_ declare

```rust
extern crate coded;
use coded::helpers;
```
----

### println!(...) doesn't work in tests

The test runner captures stdout. Disable this behavior like

```
cargo test -- --nocapture
```

----

### Lambdas

```rust
let f = |x: Vec<i32>| -> Vec<i32> {
  println!(x);
  x   // return x
}
```

---- 

### Shell Aliases

`cb="cargo build"` and `alias ct="cargo test -- --nocapture"`

----

### Get help in chat

Chill with others learning rust in IRC

Web client that looks decent: https://kiwiirc.com/client
Network: irc.mozilla.org
Channel: #rust-beginners


