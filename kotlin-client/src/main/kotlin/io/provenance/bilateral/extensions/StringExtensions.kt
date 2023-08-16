package io.provenance.bilateral.extensions

import cosmos.base.abci.v1beta1.Abci.TxMsgData
import cosmos.base.abci.v1beta1.Abci.TxResponse
import cosmwasm.wasm.v1beta1.Tx.MsgExecuteContractResponse
import org.bouncycastle.util.encoders.Hex

/**
 * Takes the data node of a [TxResponse] and converts it to the data bytes sent by a smart contract.  This will only be
 * successful when used on a [TxResponse] that represents a smart contract response that has set the data on its
 * Response value.  It should be expected to fail for other types of responses, and for responses that include data
 * about multiple transactions that have been bundled together, because the parsed [TxMsgData] will no longer be a single
 * value.
 */
internal fun TxResponse.executeContractDataToJsonBytes(): ByteArray = Hex
    .decode(this.data)
    .let(TxMsgData::parseFrom)
    .dataList
    .singleOrNull()
    ?.data
    ?.let(MsgExecuteContractResponse::parseFrom)
    ?.data
    ?.toByteArray()
    ?: throw IllegalStateException("Could not parse tx data to contract json. Hex: ${this.data}")
