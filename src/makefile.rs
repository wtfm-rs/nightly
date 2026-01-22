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
//! CARGO = rustup run nightly cargo
//! ```
//! Here is how to use it.
//!
//! `make versions`
//! ```make
//!	$(RUSTC) --version --verbose
//!	$(RUSTDOC) --version --verbose
//!	$(CARGO) --version --verbose
//! ```
//! Local MacOS (You can see CI output in [workflow](../workflow/index.html).)
//! ```sh
//! % make versions
//! rustup run nightly rustc --version --verbose
//! rustc 1.95.0-nightly (eda76d9d1 2026-01-21)
//! binary: rustc
//! commit-hash: eda76d9d1d133effbf7facb28168fd78d75fd434
//! commit-date: 2026-01-21
//! host: aarch64-apple-darwin
//! release: 1.95.0-nightly
//! LLVM version: 21.1.8
//! rustup run nightly rustdoc --out-dir docs  --version --verbose
//! rustdoc 1.95.0-nightly (eda76d9d1 2026-01-21)
//! binary: rustdoc
//! commit-hash: eda76d9d1d133effbf7facb28168fd78d75fd434
//! commit-date: 2026-01-21
//! host: aarch64-apple-darwin
//! release: 1.95.0-nightly
//! LLVM version: 21.1.8
//! rustup run nightly cargo --version --verbose
//! cargo 1.95.0-nightly (85eff7c80 2026-01-15)
//! release: 1.95.0-nightly
//! commit-hash: 85eff7c80277b57f78b11e28d14154ab12fcf643
//! commit-date: 2026-01-15
//! host: aarch64-apple-darwin
//! libgit2: 1.9.2 (sys:0.20.3 vendored)
//! libcurl: 8.7.1 (sys:0.4.84+curl-8.17.0 system ssl:(SecureTransport) LibreSSL/3.3.6)
//! ssl: OpenSSL 3.5.4 30 Sep 2025
//! os: Mac OS 26.2.0 [64-bit]
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
