package io.provenance.bilateral.execute

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractExecuteMsg
import io.provenance.bilateral.models.RequestDescriptor

/**
 * The top-level execute message that comprises an update for a bid.  This must target an existing bid by its id, and
 * will replace the contents of that bid with the new specified values.  The current collateral will be refunded to the
 * bidder and replaced with the new collateral (quote).  This request can only be made by the owner of the bid being
 * modified.  See [Bid] for descriptions of each bid type.
 *
 * @param bid Defines the new structure of the bid.
 * @param descriptor Contains various options for a bid, universal to the request type.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("update_bid")
data class UpdateBid(
    val bid: Bid,
    val descriptor: RequestDescriptor? = null,
) : BilateralContractExecuteMsg {
    override fun toLoggingString(): String = "updateBid, ${bid.toLoggingStringSuffix()}"
}
