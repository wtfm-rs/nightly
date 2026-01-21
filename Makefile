RUSTC = rustup run nightly rustc
RUSTDOC = rustup run nightly rustdoc \
	--out-dir docs \
	-Z unstable-options \
	--playground-url "https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024"
CARGO = rustup run nightly cargo

all:	install versions build
	cp README.md docs

install:
	rustup toolchain list
	rustup toolchain install nightly
	$(CARGO) update

build:
	rm -rf docs
	$(RUSTDOC) src/introduction.rs
	$(RUSTDOC) src/rustup.rs
	$(RUSTDOC) src/workflow.rs

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
	
