package io.provenance.bilateral.serialization

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.JsonDeserializer
import java.math.BigInteger

/**
 * Deserializes a CosmWasm Uint value in a response JSON payload to a BigInteger, assuming it is a quoted integer.
 */
class CosmWasmUintToBigIntegerDeserializer : JsonDeserializer<BigInteger>() {
    /**
     * This is a deserializer for Cosmwasm's Uint types (they include wrappers for most unsigned rust integer types).
     * The value is returned as a double-quoted numeric value, so simply invoking toBigIntegerOrNull is sufficient
     * for deserialization.
     */
    override fun deserialize(p: JsonParser?, ctxt: DeserializationContext?): BigInteger? = p
        ?.text
        ?.toBigIntegerOrNull()
}
