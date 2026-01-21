RUSTC = rustup run nightly rustc
RUSTDOC = rustup run nightly rustdoc --playground-url "https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024" -Z unstable-options
CARGO = rustup run nightly cargo

all:	install versions build

install:
	rustup toolchain list
	rustup toolchain install nightly
	$(CARGO) update

build:
	rm -rf doc
	$(RUSTDOC) src/introduction.rs
	$(RUSTDOC) src/rustup.rs
	$(RUSTDOC) src/workflow.rs
	cp -r doc docs
	cp README.md docs
	rm -rf doc

clippy:
	$(CARGO) clippy

test:
	$(CARGO) fmt
	$(CARGO) test

versions:
	$(RUSTC) --version
	$(RUSTDOC) --version
	$(CARGO) --version

clean:
	$(CARGO) clean 
	
