package io.provenance.bilateral.interfaces

import com.fasterxml.jackson.databind.ObjectMapper
import cosmos.base.v1beta1.CoinOuterClass
import cosmwasm.wasm.v1.Tx.MsgExecuteContract

/**
 * An interface that designates the implementing class as a top-level message to be used in executing routes of the
 * Metadata Bilateral Exchange smart contract.
 */
interface BilateralContractExecuteMsg : BilateralContractMsg {
    /**
     * Converts the implementing class into a [MsgExecuteContract] by serializing it to a ByteString containing a
     * representation of itself in JSON.
     *
     * @param objectMapper A Jackson [ObjectMapper] instance that is used to serialize the class instance to JSON.
     * @param contractAddress The bech32 address of the targeted Metadata Bilateral Exchange smart contract.
     * @param senderBech32Address The bech32 address of the account that will execute this message against the contract.
     * @param funds The amount of coin sent with the request.  This value is nullable because some requests require
     * coin, and others require it to be completed omitted.
     */
    fun toExecuteMsg(
        objectMapper: ObjectMapper,
        contractAddress: String,
        senderBech32Address: String,
        funds: List<CoinOuterClass.Coin>? = null,
    ): MsgExecuteContract = MsgExecuteContract.newBuilder().also { msg ->
        msg.msg = this.toJsonByteString(objectMapper)
        msg.contract = contractAddress
        msg.sender = senderBech32Address
        funds?.also { msg.addAllFunds(funds) }
    }.build()
}
