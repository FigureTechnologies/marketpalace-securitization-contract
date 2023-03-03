.PHONY: all
all: clean contracts cli
	@cargo build -p cli

.PHONY: cli
cli:
	@cargo build -p cli

.PHONY: contract
contracts:
	make optimize -f crates/contract/Makefile
	rm -rf target

.PHONY: clean
clean:
	rm -rf artifacts
	rm -rf target
