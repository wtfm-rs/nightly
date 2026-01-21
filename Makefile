RUSTC = rustup run nightly rustc
CARGO = rustup run nightly cargo

all:	install versions build

install:
	rustup toolchain list
	rustup toolchain install nightly
	$(CARGO) update

build:
	$(CARGO) doc --release --target-dir docs
	rm -rf docs/debug
	rm -rf docs/release
	rm -f docs/.rustc_info.json
	rm -f docs/.rustdoc_fingerprint.json
	rm -f docs/CACHEDIR.TAG
	cp README.md docs

test:
	$(CARGO) fmt
	$(CARGO) test --target-dir docs
	

versions:
	$(RUSTC) --version
	$(CARGO) --version

clean:
	$(CARGO) clean --target-dir docs
	
