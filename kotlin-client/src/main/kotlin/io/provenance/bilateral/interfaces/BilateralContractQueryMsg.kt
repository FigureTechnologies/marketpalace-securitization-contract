package io.provenance.bilateral.interfaces

import com.fasterxml.jackson.databind.ObjectMapper
import cosmwasm.wasm.v1beta1.QueryOuterClass.QuerySmartContractStateRequest

/**
 * An interface that designates the implementing class as a top-level message to be used in querying routes of the
 * Metadata Bilateral Exchange smart contract.
 */
interface BilateralContractQueryMsg : BilateralContractMsg {
    /**
     * Converts the implementing class into a [QuerySmartContractStateRequest] by serializing it to a ByteString
     * containing a representation of itself in JSON.
     *
     * @param objectMapper A Jackson [ObjectMapper] instance that is used to serialize the class instance to JSON.
     * @param contractAddress The bech32 address of the targeted Metadata Bilateral Exchange smart contract.
     */
    fun toQueryMsg(
        objectMapper: ObjectMapper,
        contractAddress: String,
    ): QuerySmartContractStateRequest = QuerySmartContractStateRequest.newBuilder().also { msg ->
        msg.queryData = this.toJsonByteString(objectMapper)
        msg.address = contractAddress
    }.build()
}
