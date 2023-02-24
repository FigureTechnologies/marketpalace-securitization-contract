# Marketpalace Securitization
This repository houses both the Marketpalace Securitization Contract and the CLI to generate
the JSON messages for its queries and transactions. The contract's main purpose is to allow
General Partners to create securities and Limited Partners to commit and deposit funds
for shares. For more information please review the smart contract's [documentation](https://github.com/FigureTechnologies/marketpalace-securitization-contract/tree/main/crates/contract).

## Status
[![Apache 2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Build
Since this repository contains multiple projects, one or more projects can be built using Makefile and Cargo.

To quickly build everything run the following make command:
`make all`

If you would like to build just the smart contract, then the following command can be used:
`make contract`

Alternatively, the CLI can be built by itself with:
`make cli`