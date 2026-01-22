//! This crate has no code, only [rustdoc](https://wtfm-rs.github.io/nightly/rust/html/rustdoc/index.html).
//!
//! # Makefile macros
//!
//! Since we use make as the top level CLI/CI, it would be convenient to 
//! wrap Rust nigthly toolchain commands with 
//! [Makefile macros](https://www.gnu.org/software/make/manual/html_node/Using-Variables.html).
//! 
//! ```make
//! RUSTC = rustup run nightly rustc
//! RUSTDOC = rustup run nightly rustdoc --out-dir docs
//! ```
//! Here is how to use it.
//!
//! `make versions`
//! ```make
//!	$(RUSTC) --version
//!	$(RUSTDOC) --version
//! ```
//! 
//! ```sh
//! % make versions
//! rustup run nightly rustc --version
//! rustc 1.95.0-nightly (eda76d9d1 2026-01-21)
//! rustup run nightly rustdoc --out-dir docs  --version
//! rustdoc 1.95.0-nightly (eda76d9d1 2026-01-21)
//! ```
//!
//! `make build`
//! ```make
//!	$(RUSTDOC) src/introduction.rs
//!	$(RUSTDOC) src/assert_matches.rs
//!	$(RUSTDOC) src/bin/coroutine.rs
//!	$(RUSTDOC) src/makefile.rs
//!	$(RUSTDOC) src/workflow.rs
//! ```
//!
//! ```sh
//! % make build
//! rustup run nightly rustdoc --out-dir docs  src/introduction.rs
//! rustup run nightly rustdoc --out-dir docs  src/assert_matches.rs
//! rustup run nightly rustdoc --out-dir docs  src/bin/coroutine.rs
//! rustup run nightly rustdoc --out-dir docs  src/makefile.rs
//! rustup run nightly rustdoc --out-dir docs  src/workflow.rs
//! ```
//!
//! `make test`
//! ```make
//!     $(RUSTC) --test src/assert_matches.rs && ./assert_matches && rm ./assert_matches
//!     $(RUSTC) src/bin/coroutine.rs && ./coroutine && rm ./coroutine
//! ```
//!
//! ```sh
//!
//! % make test
//! rustup run nightly rustc --test src/assert_matches.rs && ./assert_matches && rm ./assert_matches
//!
//! running 1 test
//! test tests::test_assert_matches - should panic ... ok
//!
//! test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//!
//! rustup run nightly rustc src/bin/coroutine.rs && ./coroutine && rm ./coroutine
//! It works!
//! ``` 		 			
