//! <https://wtfm-rs.github.io/nightly/rust/html/unstable-book/index.html>
//! This only works in nightly
//! ```sh
//! rustup run nightly cargo  run --bin coroutine
//!   Compiling wtfm_nightly v0.1.0 (/Users/sam/github/wtfm-rs/nightly)
//! error[E0554]: `#![feature]` may not be used on the stable release channel
//!  --> src/bin/coroutine.rs:1:1
//!   |
//! 1 | #![feature(coroutines, coroutine_trait, stmt_expr_attributes)]
//!   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!
//!
//! For more information about this error, try `rustc --explain E0554`.
//! error: could not compile `wtfm_nightly` (bin "coroutine") due to 1 previous error
//! ```
#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

fn main() {
    let mut coroutine = #[coroutine] || {
        yield 1;
        return "foo"
    };

    match Pin::new(&mut coroutine).resume(()) {
        CoroutineState::Yielded(1) => {}
        _ => panic!("unexpected value from resume"),
    }
    match Pin::new(&mut coroutine).resume(()) {
        CoroutineState::Complete("foo") => {}
        _ => panic!("unexpected value from resume"),
    }
    println!("It works!");
}
