.PHONY: all
all: clean contract cli
	@cargo build -p cli

.PHONY: cli
cli:
	@cargo build -p cli

.PHONY: contract
contract:
	make -C crates/contract optimize
	cp -r crates/contract/artifacts .

.PHONY: clean
clean:
	rm -rf target
	rm -rf artifacts
