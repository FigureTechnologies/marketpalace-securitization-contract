# Marketpalace Securitization
The purpose of this contract is to act as a third party to help a GP raise funding by producing one or more securities. Any LP can partake in this and deposit funding for shares of one or more of the securities created by the contract. These LPs will then be rewarded with investment tokens when they have finished settlement.
## Process / Concepts
In order to understand the contract it's important to have an understanding of some of the processes and concepts. The following subsections should help in clarifying what these processes and concepts are.
### Commitment Lifecycle
A commitment will transition between the three following states:
  - `PENDING`: A commitment will be considered `PENDING` when a LP proposes a commitment, but it has not yet been accepted yet.
  - `ACCEPTED`: A commitment will move into the `ACCEPTED` state when the GP accepts a proposed commitment. The LP will then have to pay their committed funds.
  - `SETTLED`: A commitment transitions into the `SETTLED` state when a GP has withdrawn all of a LP's commitment. The LP will be rewarded with investment tokens.
### Investment Tokens
These are tokens that represent the shares of a security. A LP will receive these when they have paid their commitment in full and have reached settlement. Each security has its own unique investment token.

## Account Roles
There are three types of accounts that interact with this smart contract.

1. Admin Account: This account is responsible for initializing and configuring the contract.
   
2. General Partner (GP) Account: This account is responsible for approving commitments and withdrawing deposits from the LPs.
   
3. Limited Partner (LP) Account: One or more of these accounts will commit to a securitization, and deposit funds. In exchange these accounts will receive investment tokens.
## Contract Interaction


### [Instantiation](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/04283f029387ac9df543a936bc661a32ca2130a2/crates/contract/src/core/msg.rs#L13-L20)
In order to use this contract it must first be instantiated by the admin. The admin will specify who the GP is, what securities will be involved, the denomination of the deposits, and the investment vehicle rules. A more detailed view of message can be seen in the [json](schema/instantiate_msg.json).

When a contract is instantiated it first validates the message and ensures the following are true:
1. The list of securities is not empty.
2. All securities are of the same type.
3. All securities have unique names.
4. The capital denom is not empty.

After validation has succeed, the contract routes the message to the correct handler and begins updating state. The contract version is updated, and the stores are updated with the request params. Lastly, a marker is created for each security. If a fee is provided, then a `MsgFees` message will be added to the response.

#### Request Parameters
- `gp`: The address of the General Partner. They will be the one to accept commitments and withdraw capital.
- `securities`: The list of securities that Limited Partners can commit to. A security can either be a `Tranche`, `Primary`, or `Fund`.
- `capital_denom`: The denomination of the collected capital.
- `fee`: An optional additional fee that can be added to the instantiation.
- `settlement_time`: An optional time in seconds since epoch, and a value of null will disable the settlement time. A contract with no settlement time will act is if there is unlimited time to settle.

#### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `init`.
- `fee_recipient`: The account received a portion or all of the fee. This will only be emitted when there is a `Fee`.
- `fee_amount`: The amount that was paid for the fee. This will only be emitted where there is a `Fee`.

#### Request Sample
```
{
    "gp": "tp1d0a2la87mxxefduquqyjppkrg72msa6nhwek3d",
    "securities": [
        {
            "name": "Security1",
            "amount": "1000",
            "security_type": {
                "tranche": {}
            },
            "minimum_amount": "10",
            "price_per_unit": {
                "denom": "nhash",
                "amount": "1000000000"
            }
        },
        {
            "name": "Security2",
            "amount": "5000",
            "security_type": {
                "tranche": {}
            },
            "minimum_amount": "100",
            "price_per_unit": {
                "denom": "nhash",
                "amount": "1000000000"
            }
        }
    ],
    "capital_denom": "nhash",
    "settlement_time": "1678975183"
    "fee": {
        "recipient": "tp1d0a2la87mxxefduquqyjppkrg72msa6nhwek3d",
        "amount": {
            "denom": "nhash",
            "amount": "1000000000"
        }
    }
}
```

### Execution Routes
This contract contains four different types of execution messages. Every message is first validated and then handed off to the execute router. The router will then forward the message to the correct handler to be ran. A more detailed view of these messages can be seen in the [json](schema/execute_msg.json).

#### [Propose Commitment](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/2255001f4f10fda9c1bf73b79be6efb953336b30/crates/contract/src/core/msg.rs#L24)
The ProposeCommitment message is sent by a Limited Partner when they are interested in funding a GP. They will make an offer containing how many of each security they are interested in purchasing. Multiple proposals by the same LP will be additive and update the current proposal.

This message must contain a non-empty list of existing securities. Additionally, the commitment's security amounts must be greater than or equal to the minimum otherwise the transaction will be rejected. The transaction will also be rejected if the blocktime is greater than the settlement time. Any duplicate proposal commitment must contain a list of new securities that were not already proposed by the LP. This modified proposal must also not have been accepted yet.

##### Request Parameters
- `securities`: A list containing the name and amount of each security they are interested in exchanging funding for.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `propose_commitment`.
- `lp`: The address of the lp proposing a commitment.

##### Request Sample
```
{
    "propose_commitment": {
        "securities": [
            {
                "name": "Security1",
                "amount": "100"
            },
            {
                "name": "Security2",
                "amount": "200"
            }
        ]
    }
}
```

#### [Accept Commitment](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/2255001f4f10fda9c1bf73b79be6efb953336b30/crates/contract/src/core/msg.rs#L25)
The AcceptCommitment message is sent by the General Partner. They will submit this message with a list containing the addresses of the accepted LPs and their security commitment amounts. This list must be non-empty, and each supplied commitment must be in the `PENDING` state. The number of shares/units these commitments have cannot be greater than the remaining amount of their respective security. The securities that are listed for a LP must match what the LP proposed. Lastly, this transaction will fail if the blocktime is greater than the settlement time.

##### Request Parameters
- `commitments`: A list of proposed commitments that the GP wishes to approve. Each commitment contains the lp and their proposed securities.
  - `lp`: The address of the LP.
  - `securities`: A list of security names and amounts. This must match what the LP proposed.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `accept_commitments`.
- `gp`: The address of the GP proposing a commitment.

##### Emitted Events
- `accepted`: An event representing an accepted LP.
  - `lp`: The address of the accepted LP.

##### Request Sample
```
{
    "accept_commitment": {
        "commitments": [
            {
                "lp": "tp1d0a2la87mxxefduquqyjppkrg72msa6nhwek3d",
                "securities": [
                    {
                        "name": "Security1",
                        "amount": "100"
                    },
                    {
                        "name": "Security2",
                        "amount": "200"
                    }
                ]
            },
            {
                "lp": "tp1n2zvcfsvqwe9dwal7kleq0qv0a676kvm4alekx",
                "securities": [
                    {
                        "name": "Security1",
                        "amount": "100"
                    },
                    {
                        "name": "Security2",
                        "amount": "200"
                    }
                ]
            }
        ]
    }
}
```

#### [Deposit Commitment](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/2255001f4f10fda9c1bf73b79be6efb953336b30/crates/contract/src/core/msg.rs#L26)
The DepositCommitment message is sent by one of the accepted LPs. Its purpose is for the LP to partially or completely pay off their commitment. The included funds will then be stored in the contract, and the GP can withdraw them at a later time. LPs cannot deposit more than they have committed, the funds must equal the sum of the cost of all the message's securities. Lastly, every deposit must have funds and this transaction will fail if the blocktime is greater than the settlement time.

##### Request Parameters
- `securities`: A list of partial or complete security commitments that the LP is depositing funds for.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `deposit_commitment`.
- `lp`: The address of the LP depositing funds.

##### Request Sample
```
{
    "deposit_commitment": {
        "securities": [
            {
                "name": "Security1",
                "amount": "50"
            },
            {
                "name": "Security2",
                "amount": "75"
            }
        ]
    }
}
```

#### [Withdraw Commitment](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/2255001f4f10fda9c1bf73b79be6efb953336b30/crates/contract/src/core/msg.rs#L27)
The WithdrawCommitment message is sent by the GP, and it allows them to take capital that was deposited into the contract by a specific LP. If and only if the LP's deposited capital  matches the promised commitment funds will the tx succeed and transition the commitment to `SETTLED`. Once settled, the contract will mint and transfer the LP their investment tokens. This transaction will fail if the blocktime is greater than the settlement time.

This contract will emit an event for the settled LP.

##### Request Parameters
- `lp`: The addresses of the LP to settle with.

##### Emitted Events
- `settled`: An event representing the settled LP.
  - `lp`: The address of the settled LP.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `withdraw_commitments`.
- `gp`: The address of the GP withdrawing funds.

##### Request Sample
```
{
    "withdraw_commitment": {
        "lp": "tp1d0a2la87mxxefduquqyjppkrg72msa6nhwek3d"
    }
}
```

#### [Withdraw All Commitments](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/2255001f4f10fda9c1bf73b79be6efb953336b30/crates/contract/src/core/msg.rs#L28)
The WithdrawAllCommitments message is sent by the GP, and it allows them to attempt take capital that was deposited into the contract by all `ACCEPTED` LPs. If and only if the LP's deposited capital matches the promised commitment funds will the commitment transition to `SETTLED`. Once settled, the contract will mint and transfer the LP their investment tokens. This transaction will fail if the blocktime is greater than the settlement time.

This contract will emit an event each settled LP.

##### Emitted Events
- `settled`: An event representing the settled LP.
  - `lp`: The address of the settled LP.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `withdraw_all_commitments`.
- `gp`: The address of the GP withdrawing funds.

##### Request Sample
```
{
    "withdraw_all_commitments": {}
}
```

#### [Update Settlement Time](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/2255001f4f10fda9c1bf73b79be6efb953336b30/crates/contract/src/core/msg.rs#L29)
The UpdateSettlementTime message is sent by the GP, and it allows them to change the settlement time.

##### Request Parameters
- `settlement_time`: An optional time in seconds since epoch, and a value of null will disable the settlement time. A contract with no settlement time will act is if there is unlimited time to settle.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `update_settlement_time`.

##### Request Sample
```
{
    "update_settlement_time": {
        "settlement_time": "86400"
    }
}
```

#### [CancelCommitment](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/2255001f4f10fda9c1bf73b79be6efb953336b30/crates/contract/src/core/msg.rs#L29)
The CancelCommitment message can only be sent by either the GP or the LP of the commitment. This message will completely remove the LP's commitment and refund them as long as they have not settled yet.

##### Request Parameters
- `lp`: The address of the LP to cancel the commitment from

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `cancel_commitment`.
- `sender`: The address of the message sender.
- `canceled_lp`: The address of the LP with the canceled commitment.

##### Request Sample
```
{
    "cancel_commitment": {
        "lp": "tp1d0a2la87mxxefduquqyjppkrg72msa6nhwek3d"
    }
}
```

### Query Routes
This contract exposes five different query routes which allow users to view the state of the contract, investors, and the investor's commitments. A more detailed view of these messages can be seen in the [json](schema/query_msg.json).

#### [Query Version](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/04283f029387ac9df543a936bc661a32ca2130a2/crates/contract/src/core/msg.rs#L46-L47)
This route can be used to obtain the contract's version.

##### Request Sample
```
{
    "query_version": {}
}
```

##### Response Sample
```
{
  "data": {
    "contract_version": {
      "contract": "contract",
      "version": "1.0.7"
    }
  }
}
```

#### [Query State](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/04283f029387ac9df543a936bc661a32ca2130a2/crates/contract/src/core/msg.rs#L43-L44)
This route can be used to obtain gp, securities, capital denom, and rules that were setup during instatiation.

##### Request Sample
```
{
    "query_state":{}
}
```

##### Response Sample
```
{
  "data": {
    "gp": "tp1ykdj7kdtv8t2lqvflmmp7y4j596q3nf3cxjw7s",
    "securities": [
      "Security1",
      "Security2"
    ],
    "capital_denom": "nhash",
    "settlement_time": null
  }
}
```

#### [Query Investor](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/04283f029387ac9df543a936bc661a32ca2130a2/crates/contract/src/core/msg.rs#L34-L35)
This route can be used to obtain the commitment made by an investor, and how much of that commitment they have paid.

##### Request Sample
```
{
    "query_investor": {
        "investor": "tp1udtttp4crmfyp3s7z2mqzxa9dxyx6lrphf4uzz"
    }
}
```

##### Response Sample
```
{
  "data": {
    "commitment": {
      "lp": "tp1ykdj7kdtv8t2lqvflmmp7y4j596q3nf3cxjw7s",
      "commitments": [
        {
          "name": "Security1",
          "amount": "5"
        }
      ],
      "state": "p_e_n_d_i_n_g",
      "settlment_date": null
    },
    "paid_in_capital": []
  }
}
```

#### [Query Commitments](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/04283f029387ac9df543a936bc661a32ca2130a2/crates/contract/src/core/msg.rs#L37-L38)
This route can be used to obtain a list of all the commitments in the specified state. The state can either be "p_e_n_d_i_n_g", "a_c_c_e_p_t_e_d", or "s_e_t_t_l_e_d".

##### Request Sample
```
{
    "query_commitments": {
        "commitment_state": "p_e_n_d_i_n_g"
    }
}
```

##### Response Sample
```
{
    "data": {
        "commitments": [
            {
                "lp": "tp19v8dpxddfacfj78u5d2kducghudh6llsn7ff2k",
                "commitments": [
                    {
                        "name": "Security1",
                        "amount": "5"
                    }
                ],
                "state": "p_e_n_d_i_n_g",
                "settlment_date": null
            },
            {
                "lp": "tp1ykdj7kdtv8t2lqvflmmp7y4j596q3nf3cxjw7s",
                "commitments": [
                    {
                        "name": "Security1",
                        "amount": "5"
                    }
                ],
                "state": "p_e_n_d_i_n_g",
                "settlment_date": null
            }
        ]
    }
}
```

#### [Query Securitizations](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/04283f029387ac9df543a936bc661a32ca2130a2/crates/contract/src/core/msg.rs#L40-L41)
This route can be used to obtain initialization information about one or more securities. 

##### Request Sample
```
{
    "query_securitizations": {
        "securities": [
            "Security1"
        ]
    }
}
```

##### Response Sample
```
{
  "data": {
    "securities": [
      {
        "name": "Security1",
        "amount": "10",
        "security_type": {
          "tranche": {}
        },
        "minimum_amount": "1",
        "price_per_unit": {
          "denom": "nhash",
          "amount": "10"
        }
      }
    ]
  }
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
provenanced tx wasm store contract.wasm
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
'{"gp":"tp13k86awgexqdt2f2wtu6ukdhrg8dc8nrtmc49pl","securities":[{"name":"Security1","amount":"1000","security_type":{"tranche":{}},"minimum_amount":"10","price_per_unit":{"denom":"nhash","amount":"1000000000"}}],"capital_denom":"nhash","rules":[]}' \
--admin "$node0" \
--from "$node0" \
--home build/node0 \
--label securities \
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