package io.provenance.bilateral.client

import cosmos.auth.v1beta1.Auth.BaseAccount
import cosmos.tx.v1beta1.ServiceOuterClass.BroadcastMode

/**
 * For functions in the [BilateralContractClient] that directly run broadcasts to the Provenance Blockchain, this
 * class allows different specifications for how those requests are to be processed.
 *
 * @param broadcastMode The mode to use when broadcasting.  Defaults to block, which is the safest (but slowest)
 * broadcast type.
 * @param sequenceOffset A number to offset the sequence number of the broadcasting signer, allowing for correctly-managed
 * asynchronous broadcasts to properly track the expected sequence number.
 * @param gasAdjustment A multiplier for gas fees above the default amount.  Increase this number if the transaction
 * runs out of gas.
 * @param baseAccount A provided BaseAccount for the signer.  If this value is not provided, the client will automatically
 * resolve a BaseAccount via querying the Provenance Blockchain.
 */
data class BilateralBroadcastOptions(
    val broadcastMode: BroadcastMode = BroadcastMode.BROADCAST_MODE_BLOCK,
    val sequenceOffset: Int = 0,
    val gasAdjustment: Double? = null,
    val baseAccount: BaseAccount? = null,
)
