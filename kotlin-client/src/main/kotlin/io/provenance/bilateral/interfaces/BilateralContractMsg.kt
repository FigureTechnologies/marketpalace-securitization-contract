package io.provenance.bilateral.interfaces

import com.fasterxml.jackson.databind.ObjectMapper
import com.google.protobuf.ByteString
import io.provenance.scope.util.toByteString

/**
 * An interface that designates a class as a message to be converted to JSON and sent to the Metadata Bilateral Exchange
 * smart contract.
 */
interface BilateralContractMsg {
    /**
     * A required function that converts a message sent to the contract into a logging string, ensuring that all
     * messages can be relayed as logs.
     */
    fun toLoggingString(): String

    /**
     * A pre-built function that converts any message to a JSON byte string to make it transferable to the smart
     * contract.
     *
     * @param objectMapper A properly-configured Jackson [ObjectMapper] instance to write any contract-related message
     * as a JSON string.
     */
    fun toJsonByteString(objectMapper: ObjectMapper): ByteString = objectMapper.writeValueAsString(this).toByteString()
}
