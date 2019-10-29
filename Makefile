build:
	cargo bootimage
run:
	cargo bootimage
	cargo xrun
deps:
	rustup override add nightly
	cargo install cargo-xbuild bootimage
	rustup component add rust-src
	rustup component add llvm-tools-preview