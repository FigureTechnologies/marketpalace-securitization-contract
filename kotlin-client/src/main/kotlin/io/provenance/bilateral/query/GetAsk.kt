package io.provenance.bilateral.query

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractQueryMsg

/**
 * The base JSON model for querying the Metadata Bilateral Exchange smart contract for an existing [io.provenance.bilateral.models.AskOrder].
 *
 * @param id The unique identifier for the target [io.provenance.bilateral.models.AskOrder].
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("get_ask")
data class GetAsk(val id: String) : BilateralContractQueryMsg {
    override fun toLoggingString(): String = "getAsk, id = [$id]"
}
