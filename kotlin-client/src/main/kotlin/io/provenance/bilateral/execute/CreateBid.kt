package io.provenance.bilateral.execute

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractExecuteMsg
import io.provenance.bilateral.models.RequestDescriptor

/**
 * The top-level execute message that comprises a bid.  See [Bid] for descriptions of each bid type.  The sender of this
 * message becomes the sole owner of the created [io.provenance.bilateral.models.BidOrder].
 *
 * @param bid Defines the type of bid to create.
 * @param descriptor Contains various options for a bid, universal to any request type.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("create_bid")
data class CreateBid(val bid: Bid, val descriptor: RequestDescriptor? = null) : BilateralContractExecuteMsg {
    override fun toLoggingString(): String = "createBid, ${bid.toLoggingStringSuffix()}"
}
