.PHONY: all
all: clean contracts

.PHONY: contracts
contracts:
	@docker run --rm -v $(CURDIR):/code \
		--mount type=volume,source="contract_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.16.0 ./crates/contract

.PHONY: clean
clean:
	rm -rf artifacts
	rm -rf target
