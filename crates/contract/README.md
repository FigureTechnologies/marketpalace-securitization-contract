# Marketpalace Securitization

## Documentation

## Process / Concepts

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

#### Deposit Commitment

### Withdraw Commitments



## Local Deployment