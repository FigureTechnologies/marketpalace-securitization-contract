.PHONY: all
all: clean contracts

.PHONY: contracts
contracts:
	make optimize -f crates/contract/Makefile
	rm -rf target

.PHONY: clean
clean:
	rm -rf artifacts
	rm -rf target
