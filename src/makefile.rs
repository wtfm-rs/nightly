//! This crate has no code, only [rustdoc](https://doc.rust-lang.org/stable/rustdoc/).
//! [RTFM](https://rust-lang.github.io/rustup/index.html).
//!
//! # rustup run
//!
//! ```sh
//! rustup run --help 
//! Run a command with an environment configured for a given toolchain
//!
//! Usage: rustup[EXE] run [OPTIONS] <TOOLCHAIN> <COMMAND>...
//!
//! Arguments:
//!  <TOOLCHAIN>   Toolchain name, such as 'stable', 'nightly', '1.8.0', or a custom toolchain name, or an
//!                absolute path. For more information see `rustup help toolchain`
//!  <COMMAND>...  
//!
//! Options:
//!      --install  Install the requested toolchain if needed
//!  -h, --help     Print help
//!
//! Discussion:
//!    Configures an environment to use the given toolchain and then runs
//!    the specified program. The command may be any program, not just
//!    rustc or cargo. This can be used for testing arbitrary toolchains
//!    without setting an override.
//!
//!    Commands explicitly proxied by `rustup` (such as `rustc` and
//!    `cargo`) also have a shorthand for this available. The toolchain
//!    can be set by using `+toolchain` as the first argument. These are
//!    equivalent:
//!
//!        $ cargo +nightly build
//!
//!        $ rustup run nightly cargo build
//! ```
//! It seems the online help above was out-dated.
//! ```sh
//! % cargo --version
//! cargo 1.92.0 (Homebrew)
//! % cargo +nightly build
//! error: no such command: `+nightly`
//!
//! help: invoke `cargo` through `rustup` to handle `+toolchain` directives
//! ```
//! So we will use `rustuprun nightly ...` version. It might be a good idea
//! to wrap them with Makefile macros.
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
//!	rm -rf docs
//!	$(RUSTDOC) src/introduction.rs
//!	$(RUSTDOC) src/assert_matches.rs
//!	$(RUSTDOC) src/makefile.rs
//!	$(RUSTDOC) src/workflow.rs
//! ```
//! ```sh
//! % make build
//! rm -rf docs
//! rustup run nightly rustdoc --out-dir docs  src/introduction.rs
//! rustup run nightly rustdoc --out-dir docs  src/assert_matches.rs
//! rustup run nightly rustdoc --out-dir docs  src/makefile.rs
//! rustup run nightly rustdoc --out-dir docs  src/workflow.rs
//! ```

//!
//! `make test`
//! ```make
//!     $(RUSTC) --test src/assert_matches.rs && ./assert_matches && rm ./assert_matches
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
//! ``` 		 			
