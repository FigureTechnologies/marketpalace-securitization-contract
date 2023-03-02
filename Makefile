.PHONY: all
all: clean contract cli contract-controller
	@cargo build -p cli

.PHONY: cli
cli:
	@cargo build -p cli

.PHONY: contract
contract:
	make -C crates/contract optimize
	cp -r crates/contract/artifacts .

.PHONY: contract-controller
contract-controller:
	make -C crates/contract-controller optimize
	cp -r crates/contract-controller/artifacts .

.PHONY: clean
clean:
	rm -rf target
	rm -rf artifacts
