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

run: up build FORCE
	target/debug/kube-gestalt

watch: up FORCE
	cargo watch -x "test" -x "run"

.PHOMY: FORCE
FORCE: