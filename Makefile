RUSTC = rustup run nightly rustc
RUSTDOC = rustup run nightly rustdoc --out-dir docs 

all:	install versions build test fail
	cp README.md docs

install:
	rustup toolchain list
	rustup toolchain install nightly
	rustup override set nightly

build:
	$(RUSTDOC) src/introduction.rs
	$(RUSTDOC) src/assert_matches.rs
	$(RUSTDOC) src/bin/coroutine.rs
	$(RUSTDOC) src/makefile.rs
	$(RUSTDOC) src/workflow.rs

test:
	$(RUSTC) --test src/assert_matches.rs && ./assert_matches && rm ./assert_matches
	$(RUSTC) src/bin/coroutine.rs && ./coroutine && rm ./coroutine

fail:
	-rustup run nightly cargo  run --bin coroutine
	rustup run nightly cargo clean

versions:
	$(RUSTC) --version
	$(RUSTDOC) --version

doc:
	rustup doc
	cp -r ~/.rustup/toolchains/nightly-aarch64-apple-darwin/share/doc/* docs
