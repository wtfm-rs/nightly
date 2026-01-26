RUSTC = rustup run nightly rustc
RUSTDOC = rustup run nightly rustdoc --out-dir docs 
CARGO = rustup run nightly cargo

all:	CI build test pass versions
	cp README.md docs

CI:
	env
	rustup --verbose --version

	rustup toolchain install nightly
	rustup component add --toolchain nightly rustfmt
	rustup override set nightly

	cargo install cargo-expand


build:
	$(RUSTDOC) src/introduction.rs
	$(RUSTDOC) src/assert_matches.rs
	$(RUSTDOC) src/bin/coroutine.rs
	$(RUSTDOC) src/makefile.rs
	$(RUSTDOC) src/workflow.rs

test:
	$(RUSTC) --test src/assert_matches.rs && ./assert_matches && rm ./assert_matches
	$(RUSTC) src/bin/coroutine.rs && ./coroutine && rm ./coroutine

pass:
	rustup run nightly cargo --version
	-rustup run nightly cargo  run --bin coroutine
	cargo --version
	-cargo  run --bin coroutine
	rustup run nightly cargo clean

versions:
	rustc --version | tee rustc-version.log
	cargo --version | tee cargo-vesion.log

doc:
	rustup doc
	rm -rf docs/*
	cp -r ~/.rustup/toolchains/nightly-aarch64-apple-darwin/share/doc/* docs

