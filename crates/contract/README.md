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


### [Instantiation](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/057cc028f64ad67e1de2ceb76ecf943ea060025c/crates/contract/src/core/msg.rs#L13-L18)
In order to use this contract it must first be instantiated by the admin. The admin will specify who the GP is, what securities will be involved, the denomination of the deposits, and the investment vehicle rules. A more detailed view of message can be seen in the [json](schema/instantiate_msg.json).

When a contract is instantiated it first validates the message and ensures the following are true:
1. The list of securities is not empty.
2. All securities are of the same type.
3. All securities have unique names.
4. The capital denom is not empty.

After validation has succeed, the contract routes the message to the correct handler and begins updating state. The contract version is updated, and the stores are updated with the request params. Lastly, a marker is created for each security.

#### Request Parameters
- `gp`: The address of the General Partner. They will be the one to accept commitments and withdraw capital.
- `securities`: The list of securities that Limited Partners can commit to. A security can either be a `Tranche`, `Primary`, or `Fund`.
- `capital_denom`: The denomination of the collected capital.
- `rules`: A list of investment vehicle rules.

#### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `init`.

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
                "primary": {}
            },
            "minimum_amount": "100",
            "price_per_unit": {
                "denom": "nhash",
                "amount": "1000000000"
            }
        }
    ],
    "capital_denom": "nhash",
    "rules": []
}
```

### Execution Routes
This contract contains four different types of execution messages. Every message is first validated and then handed off to the execute router. The router will then forward the message to the correct handler to be ran. A more detailed view of these messages can be seen in the [json](schema/execute_msg.json).

#### [Propose Commitment](https://github.com/FigureTechnologies/marketpalace-securitization-contract/blob/7fb595c57620ada63566f0ceabaf0bade62ffddf/crates/contract/src/core/msg.rs#L22)
The ProposeCommitment message is sent by a Limited Partner. When they are interested in funding a GP they will make an offer containing how many of each security they are interested in purchasing.

This message must contain a non-empty list of existing securities. If a commitment already exists for the LP or the security amounts don't match the minimum, then the message will be rejected.

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

#### Accept Commitments
The AcceptCommitments message is sent by the General Partner. They will submit this message with a list containing the addresses of the LPs that they would like to receive commitments from. This list must be non-empty, and they must be considered pending. Lastly, the accepted commitments cannot commit to more than the remaining amount of each security.

##### Request Parameters
- `commitments`: The addresses of the LPs that the GP wishes to approve.

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
            "tp1d0a2la87mxxefduquqyjppkrg72msa6nhwek3d",
            "tp1n2zvcfsvqwe9dwal7kleq0qv0a676kvm4alekx"
        ]
    }
}
```

#### Deposit Commitment
The DepositCommitment message is sent by one of the accepted LPs. It's purpose is for the LP to partially or completely pay their commitment. These funds will then be stored in the contract, and the GP can withdraw them at a later time. LPs cannot deposit more than they have committed, and every deposit must include funds. These included funds must equal the sum of the cost of all the message's securities.

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

#### Withdraw Commitments
The WithdrawCommitments message is sent by the GP, and it allows them to take capital that was deposited into the contract. A commitment will only move into the `SETTLED` state when the GP has withdrawn all the funds that the LP promised to commit. When a commitment by a LP is settled the contract will mint and transfer them LP investment tokens. Additionally, the contract will emit an event for each newly settled LP.

##### Emitted Events
- `settled`: An event representing a settled LP.
  - `lp`: The address of the settled LP.

##### Emitted Attributes
- `action`: The action that was executed. The value of this will always be `withdraw_commitments`.
- `gp`: The address of the GP withdrawing funds.

##### Request Sample
```
{
    "withdraw_commitments": {}
}
```

## Local Deployment