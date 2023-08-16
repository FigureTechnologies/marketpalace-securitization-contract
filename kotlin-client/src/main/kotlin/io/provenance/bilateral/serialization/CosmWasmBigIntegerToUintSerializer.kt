package io.provenance.bilateral.serialization

import com.fasterxml.jackson.core.JsonGenerator
import com.fasterxml.jackson.databind.JsonSerializer
import com.fasterxml.jackson.databind.SerializerProvider
import java.math.BigInteger

/**
 * Serializes a [BigInteger] value as a CosmWasm Uint value.  This essentially represents a string-quoted integer value.
 */
class CosmWasmBigIntegerToUintSerializer : JsonSerializer<BigInteger>() {
    override fun serialize(value: BigInteger?, gen: JsonGenerator?, serializers: SerializerProvider?) {
        value?.also { bigInt ->
            if (bigInt < BigInteger.ZERO) {
                throw IllegalArgumentException("CosmWasm Uint types require unsigned integers (greater than or equal to zero)")
            }
            // Write with BigInteger.toString() which uses the default radix of 10 to create a decimal output, as expected
            gen?.writeString(bigInt.toString())
        }
    }
}
