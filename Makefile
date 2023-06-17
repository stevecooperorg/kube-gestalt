all: build

setup:
	brew bundle

build:
	cargo build

clean:
	cargo clean

up:
	$(MAKE) -C test-cluster up

down:
	$(MAKE) -C test-cluster down

run: up
run: build
	target/debug/kube-gestalt