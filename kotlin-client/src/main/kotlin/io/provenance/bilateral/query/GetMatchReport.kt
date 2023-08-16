package io.provenance.bilateral.query

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractQueryMsg
import io.provenance.bilateral.models.AdminMatchOptions

/**
 * Fetches a [io.provenance.bilateral.models.MatchReport] from the smart contract for the given ask and bid orders.
 *
 * @param askId The unique identifier for the target [io.provenance.bilateral.models.AskOrder].
 * @param bidId The unique identifier for the target [io.provenance.bilateral.models.BidOrder].
 * @param adminMatchOptions Various options available when the admin is executing the match.
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("get_match_report")
data class GetMatchReport(
    val askId: String,
    val bidId: String,
    val adminMatchOptions: AdminMatchOptions? = null,
) : BilateralContractQueryMsg {
    override fun toLoggingString(): String = "getMatchReport, askId = [$askId], bidId = [$bidId], adminMatchOptions = [$adminMatchOptions]"
}
