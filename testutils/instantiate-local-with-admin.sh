#!/bin/bash
export PROVENANCE_DIR="$HOME/provenance"
# Run this after deploying and getting the code ID
# User should pass in the code ID to the contract like:
# ./instantiate-local.sh 2
if [ -z "$1" ]
then
    echo "Must provide code ID (Example ./instantiate-local.sh 19 tp...)"
    exit 1
else
    CODE_ID=$1

fi

INIT='{"gp":"tp13k86awgexqdt2f2wtu6ukdhrg8dc8nrtmc49pl","securities":[{"name":"Security1","amount":"1000","security_type":{"tranche":{}},"minimum_amount":"10","price_per_unit":{"denom":"nhash","amount":"1000000000"}}],"capital_denom":"nhash"}'
${PROVENANCE_DIR}/build/provenanced -t tx wasm instantiate "$CODE_ID" "$INIT" --label "securitization.pb" --from validator --home ${PROVENANCE_DIR}/build/run/provenanced  --node http://localhost:26657 --chain-id testing --gas-prices 190500nhash --gas auto --gas-adjustment 2 --output json -b block --admin "$(provenanced keys show -a validator -t --home ${PROVENANCE_DIR}/build/run/provenanced)"  -y | jq