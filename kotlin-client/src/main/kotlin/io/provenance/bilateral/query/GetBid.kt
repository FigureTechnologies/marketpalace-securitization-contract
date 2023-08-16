package io.provenance.bilateral.query

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractQueryMsg

/**
 * The base JSON model for querying the Metadata Bilateral Exchange smart contract for an existing [io.provenance.bilateral.models.BidOrder].
 *
 * @param id The unique identifier for the target [io.provenance.bilateral.models.BidOrder].
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("get_bid")
data class GetBid(val id: String) : BilateralContractQueryMsg {
    override fun toLoggingString(): String = "getBid, id = [$id]"
}
