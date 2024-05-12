MAKE=make
CARGO=cargo

BENCHES=

all: build

build:
	@$(CARGO) fmt && $(CARGO) build

fmt:
	@echo "Running cargo fmt..."
	@$(CARGO) fmt --all -v

lint:
	@echo "Running cargo check & clippy..."
	@$(CARGO) check && $(CARGO) clippy --all-targets --all-features --tests --benches -- -D warnings

bench: $(BENCHES)

$(BENCHES):
	@$(CARGO) bench --bench $@

test:
	@$(CARGO) test
