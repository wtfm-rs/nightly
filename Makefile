RUSTC = rustup run nightly rustc
RUSTDOC = rustup run nightly rustdoc
CARGO = rustup run nightly cargo

all:	install versions build

install:
	rustup toolchain list
	rustup toolchain install nightly
	$(CARGO) update

build:
	rm -rf doc
	$(RUSTDOC) introduction/src/lib.rs
	cp -r doc docs
	cp README.md docs
	rm -rf doc

test:
	$(CARGO) fmt
	$(CARGO) test

versions:
	$(RUSTC) --version
	$(RUSTDOC) --version
	$(CARGO) --version

clean:
	$(CARGO) clean 
	
