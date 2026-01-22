RUSTC = rustup run nightly rustc
RUSTDOC = rustup run nightly rustdoc --out-dir docs 
CARGO = rustup run nightly cargo

all:	install versions build test
	cp README.md docs

install:
	rustup toolchain list
	rustup toolchain install nightly
	rustup override set nightly
	$(CARGO) update

build:
	$(RUSTDOC) src/introduction.rs
	$(RUSTDOC) src/assert_matches.rs
	$(RUSTDOC) src/makefile.rs
	$(RUSTDOC) src/workflow.rs

clippy:
	$(CARGO) clippy

test:
	$(RUSTC) --test src/assert_matches.rs && ./assert_matches

versions:
	$(RUSTC) --version
	$(RUSTDOC) --version
	$(CARGO) --version

clean:
	$(CARGO) clean 
	
doc:
	rustup doc
	cp -r ~/.rustup/toolchains/nightly-aarch64-apple-darwin/share/doc/* docs
