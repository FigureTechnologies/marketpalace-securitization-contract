package io.provenance.bilateral.query

import com.fasterxml.jackson.annotation.JsonTypeInfo
import com.fasterxml.jackson.annotation.JsonTypeName
import com.fasterxml.jackson.databind.PropertyNamingStrategies.SnakeCaseStrategy
import com.fasterxml.jackson.databind.annotation.JsonNaming
import io.provenance.bilateral.interfaces.BilateralContractQueryMsg

/**
 * This request searches for all asks with a target collateral id.  Each ask type's collateral id differs:
 * - Coin Trade: Identical to the ask id.
 * - Marker Trade: The marker's bech32 address (NOT denom).
 * - Marker Share Sale: The marker's bech32 address (NOT denom).
 * - Scope Trade: The scope's bech32 address.
 *
 * NOTE: This query will return an empty list if no asks are found for the given collateral id.  For all ask types except
 * marker share sale, the query will return at most one ask.  Marker share sales allow for multiple asks to be created
 * for the same marker simultaneously, so the result may include more than one ask order in that case.
 *
 * @param collateralId The collateral identifier for the target [io.provenance.bilateral.models.AskOrder].
 */
@JsonNaming(SnakeCaseStrategy::class)
@JsonTypeInfo(include = JsonTypeInfo.As.WRAPPER_OBJECT, use = JsonTypeInfo.Id.NAME)
@JsonTypeName("get_asks_by_collateral_id")
data class GetAsksByCollateralId(val collateralId: String) : BilateralContractQueryMsg {
    override fun toLoggingString(): String = "getAsksByCollateralId, collateralId = [$collateralId]"
}
