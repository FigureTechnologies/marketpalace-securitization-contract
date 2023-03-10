# Contract Controller
The purpose of this contract is to manage and migrate multiple securities contracts. There is potential for a large amount of security contracts and without this assistant it can be difficult to remember and migrate all the securities.
## Process / Concepts
In order to understand the contract it's important to have an understanding of some of the processes and concepts. The following subsections should help in clarifying what these processes and concepts are.
### Management
A contract is considered managed when it has successfully been added to an instance of this contract. It can be unmanaged by running a transaction to remove it.
### Migration
Migration is not a simple task when there is a possibility of a large number of securities. Gas fees and limits can prevent all of them from being migrated at once so we introduced the concept of migration batching. This allows the admin to keep running the same transaction until the `migration_finished` attribute is emitted with the `true` value. The admin can put the contract into migration state by running the `migrate_all_contracts` transaction.

## Account Roles
There is only one type of account that interact with this smart contract.

1. Admin Account: This account is responsible for initializing, configuring, and transacting with the contrract.
   
## Contract Interaction


### [Instantiation](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/bf2df77d56cc82495131d6942f5e6e94618eefaf/crates/contract-controller/src/core/msg.rs#L6-L8)
In order to use this contract it must first be instantiated by the admin. The admin must specify the `batch_size` for migration. A more detailed view of message can be seen in the [json](schema/instantiate_msg.json).

When a contract is instantiated it first validates the message and ensures the following are true:
1. No funds are attached to the message

After validation has succeed, the contract routes the message to the correct handler and begins updating state. The contract version is updated, and the stores are updated with the request params.

#### Request Parameters
- `batch_size`: The number of contracts to migrate in one `migrate_all_contracts` transaction. A value of `0` removes batching, and all contracts will attempt to be migrated.

#### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `init`.

#### Request Sample
```
{
    "batch_size": "2",
}
```

### Execution Routes
This contract contains five different types of execution messages. Every message is first validated and then handed off to the execute router. The router will then forward the message to the correct handler to be ran. A more detailed view of these messages can be seen in the [json](schema/execute_msg.json).

#### [Add Contracts](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/bf2df77d56cc82495131d6942f5e6e94618eefaf/crates/contract-controller/src/core/msg.rs#L12-L14)
The `AddContracts` message adds one or more contracts to the instance of the Contract Controller.

The contracts list cannot be empty. Additionally, this message must be ran by the admin, and the contract cannot be in the `migrating` state.

##### Request Parameters
- `contracts`: A list containing the addresses of the contracts to start managing.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `add_contracts`.

##### Emitted Events
- `contract_added`: One or more of these events will be emitted for each contract added.
  - `contract_address`: The address of the contract added.

##### Request Sample
```
{
    "add_contracts": {
        "contracts": [
            "pb1nxwgs92ug3cd93kz094rr60ja7g80y0ncjnd22",
            "pb1lfua3p52y96qtje75zp9djrwh6q82textfsv3n",
        ]
    }
}
```

#### [Remove Contracts](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/bf2df77d56cc82495131d6942f5e6e94618eefaf/crates/contract-controller/src/core/msg.rs#L15-L17)
The `RemoveContracts` message removes one or more contracts from the instance of the Contract Controller.

The contracts list cannot be empty, and each listed contract must be managed by the Contract Controller. Additionally, this message must be ran by the admin, and the contract cannot be in the `migrating` state.

##### Request Parameters
- `contracts`: A list containing the addresses of the contracts to stop managing.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `remove_contracts`.

##### Emitted Events
- `contract_removed`: One or more of these events will be emitted for each contract removed.
  - `contract_address`: The address of the contract added.

##### Request Sample
```
{
    "remove_contracts": {
        "contracts": [
            "pb1nxwgs92ug3cd93kz094rr60ja7g80y0ncjnd22",
            "pb1lfua3p52y96qtje75zp9djrwh6q82textfsv3n",
        ]
    }
}
```

#### [Modify Batch Size](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/bf2df77d56cc82495131d6942f5e6e94618eefaf/crates/contract-controller/src/core/msg.rs#L25-L27)
The `ModifyBatchSize` updates the batching size of the `MigrateAllContracts` message. 

This message must be ran by the admin.

##### Request Parameters
- `batch_size`: A value of 0 turns of batching, and all contracts will be migrated at once.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `modify_batch_size`.
- `new_batch_size`: The new batch size that was set by the transaction.

##### Request Sample
```
{
    "modify_batch_size": {
        "batch_size": "5",
    }
}
```

#### [Migrate Contracts](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/bf2df77d56cc82495131d6942f5e6e94618eefaf/crates/contract-controller/src/core/msg.rs#L18-L21)
The `MigrateContracts` message manually migrates one or more managed contracts.

The contracts list cannot be empty, and each contract must be managed and owned by the Contract Controller. Additionally, this message must be ran by the admin, and the contract cannot be in the `migrating` state.

##### Request Parameters
- `contracts`: A list containing the addresses of the contracts to migrate.
- `new_contract`: The `code_id` of the contract to migrate to.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `migrate_contracts`.

##### Emitted Events
- `migration`: One or more of these events will be emitted for each contract that attempted to migrate.
  - `contract`: The address of the contract being migrated.
  - `success`: A bool representing if the migration was successful or not.
  - `error`: A detailed error explaining why the migration was not successful.

##### Request Sample
```
{
    "migrate_contracts": {
        "contracts": [
            "pb1nxwgs92ug3cd93kz094rr60ja7g80y0ncjnd22",
            "pb1lfua3p52y96qtje75zp9djrwh6q82textfsv3n",
        ],
        "new_contract": "5",
    }
}
```

#### [Migrate All Contracts](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/bf2df77d56cc82495131d6942f5e6e94618eefaf/crates/contract-controller/src/core/msg.rs#L22-L24)
The `MigrateAllContracts` message automatically migrates `batch_size` managed contracts. This transaction must be run multiple times if the number of managed contracts is greater than the batch size, excluding the case where `batch_size` is 0. In this case all contracts will be migrated. Once all contracts have been migrated, this transaction must be ran once more to transition out of the `migrating` state.

This message must be ran by the admin.

##### Request Parameters
- `new_contract`: The `code_id` of the contract to migrate to.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `migrate_all_contracts`.
- `migration_finished`: A boolean value representing if all contracts have been migrated.

##### Emitted Events
- `migration`: One or more of these events will be emitted for each contract that attempted to migrate.
  - `contract`: The address of the contract being migrated.
  - `success`: A bool representing if the migration was successful or not.
  - `error`: A detailed error explaining why the migration was not successful.

##### Request Sample
```
{
    "migrate_all_contracts": {
        "new_contract": "5",
    }
}
```

### Query Routes
This contract exposes five different query routes which allow users to view the state of the contract. A more detailed view of these messages can be seen in the [json](schema/query_msg.json).

#### Query Version
This route can be used to obtain the contract's version.

##### Request Sample
```
{
    "query_version": {}
}
```

#### Query State
This route can be used to obtain the `batch_size`, and if the contract is in the `migrating` state.

##### Request Sample
```
{
    "query_state": {}
}
```

#### Query Contracts
This route can be used to obtain all contracts managed by the Contract Controller.

##### Request Sample
```
{
    "query_contracts": {}
}
```
## Local Deployment
The following steps will show you how to locally run the contract with a local Provenance Blockchain instance.

1. Download and run a Provenance Blockchain localnet. The remaining commands in this tutorial are assumed to be run
   from the provenance directory.If you already have the provenance repository cloned locally, this step can be skipped.

```shell
git clone https://github.com/provenance-io/provenance.git
git checkout main
make clean
make localnet-start
```

2. Next, lets obtain the address of the `node0` account. This is an account that is setup and configured to have funds. We
    can use it to instantiate our contract.

```shell
export node0=$(provenanced keys show -a node0 --home build/node0 --testnet)
```

3. Now let's instantiate the contract!  Run the following, making sure to use the correct location of the wasm file
   that should exist in the `artifacts` directory of this repositories root.

```shell
provenanced tx wasm store controller_contract.wasm
--from "$node0" \
--home build/node0 \
--chain-id chain-local \
--gas auto \
--gas-prices 1905nhash \
--gas-adjustment 1.5 \
--broadcast-mode block \
--testnet \
--output json \
--yes | jq
```

4. Find the `code_id` output from the previous command.  If you're following this guide from a fresh install, the value
   should just be 1.  Let's assume it is for this next command.  Time to instantiate the contract!

```shell
provenanced tx wasm instantiate 1 \
'{"batch_size":"2"}' \
--admin "$node0" \
--from "$node0" \
--home build/node0 \
--label controller \
--chain-id chain-local \
--gas auto \
--gas-prices 1905nhash \
--gas-adjustment 1.5 \
--broadcast-mode block \
--testnet \
--output json \
--yes | jq
```

Success!  The contract is now deployed locally!!